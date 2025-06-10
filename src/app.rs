use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, stdout};

use crate::{
    config::Config,
    terminal::Terminal,
    ui::{UI, UIEvent},
    ai::AIAssistant,
    plugins::PluginManager,
    pty::PtyManager,
    shell::ShellManager,
    history::HistoryManager,
    completion::CompletionEngine,
    search::SearchEngine,
    multiplexer::SessionMultiplexer,
    error::WarpError,
    ai::{AdvancedAI, CompletionContext, CompletionItem, ContextualSuggestion},
};

pub struct WarpApp {
    config: Arc<Mutex<Config>>,
    terminal: Arc<Mutex<Terminal>>,
    ui: Arc<Mutex<UI>>,
    ai_assistant: Arc<AIAssistant>,
    plugin_manager: Arc<PluginManager>,
    pty_manager: Arc<Mutex<PtyManager>>,
    shell_manager: Arc<Mutex<ShellManager>>,
    history_manager: Arc<Mutex<HistoryManager>>,
    completion_engine: Arc<CompletionEngine>,
    search_engine: Arc<SearchEngine>,
    session_multiplexer: Arc<Mutex<SessionMultiplexer>>,
    event_sender: mpsc::UnboundedSender<UIEvent>,
    event_receiver: Arc<Mutex<mpsc::UnboundedReceiver<UIEvent>>>,
    advanced_ai: Arc<AdvancedAI>,
}

impl WarpApp {
    pub async fn new(config: Arc<Mutex<Config>>) -> Result<Self, WarpError> {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        
        let terminal = Arc::new(Mutex::new(Terminal::new().await?));
        let ui = Arc::new(Mutex::new(UI::new(config.clone(), event_sender.clone()).await?));
        let ai_assistant = Arc::new(AIAssistant::new(config.clone()).await?);
        let plugin_manager = Arc::new(PluginManager::new(config.clone()).await?);
        let pty_manager = Arc::new(Mutex::new(PtyManager::new().await?));
        let shell_manager = Arc::new(Mutex::new(ShellManager::new(config.clone()).await?));
        let history_manager = Arc::new(Mutex::new(HistoryManager::new(config.clone()).await?));
        let completion_engine = Arc::new(CompletionEngine::new(config.clone()).await?);
        let search_engine = Arc::new(SearchEngine::new().await?);
        let session_multiplexer = Arc::new(Mutex::new(SessionMultiplexer::new().await?));

        let advanced_ai = Arc::new(AdvancedAI::new().await?);

        Ok(Self {
            config,
            terminal,
            ui,
            ai_assistant,
            plugin_manager,
            pty_manager,
            shell_manager,
            history_manager,
            completion_engine,
            search_engine,
            session_multiplexer,
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
            advanced_ai,
        })
    }

    pub async fn run(&self) -> Result<(), WarpError> {
        // Initialize terminal
        terminal::enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        // Start background tasks
        self.start_background_tasks().await?;

        // Main event loop
        let result = self.event_loop().await;

        // Cleanup
        terminal::disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;

        result
    }

    async fn start_background_tasks(&self) -> Result<(), WarpError> {
        // Start PTY monitoring
        let pty_manager = self.pty_manager.clone();
        let event_sender = self.event_sender.clone();
        tokio::spawn(async move {
            if let Err(e) = Self::pty_monitor_task(pty_manager, event_sender).await {
                log::error!("PTY monitor task failed: {}", e);
            }
        });

        // Start AI assistant background processing
        let ai_assistant = self.ai_assistant.clone();
        tokio::spawn(async move {
            ai_assistant.start_background_processing().await;
        });

        // Start plugin manager
        self.plugin_manager.start().await?;

        Ok(())
    }

    async fn pty_monitor_task(
        pty_manager: Arc<Mutex<PtyManager>>,
        event_sender: mpsc::UnboundedSender<UIEvent>,
    ) -> Result<(), WarpError> {
        loop {
            let output = {
                let mut pty = pty_manager.lock().await;
                pty.read_output().await?
            };

            if !output.is_empty() {
                let _ = event_sender.send(UIEvent::PtyOutput(output));
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }

    async fn event_loop(&self) -> Result<(), WarpError> {
        loop {
            tokio::select! {
                // Handle terminal events
                Ok(event) = event::read() => {
                    match event {
                        Event::Key(key_event) => {
                            if self.handle_key_event(key_event).await? {
                                break;
                            }
                        }
                        Event::Resize(width, height) => {
                            self.handle_resize(width, height).await?;
                        }
                        _ => {}
                    }
                }
                
                // Handle UI events
                ui_event = async {
                    let mut receiver = self.event_receiver.lock().await;
                    receiver.recv().await
                } => {
                    if let Some(event) = ui_event {
                        self.handle_ui_event(event).await?;
                    }
                }
            }

            // Render UI
            self.render().await?;
        }

        Ok(())
    }

    async fn handle_key_event(&self, key_event: KeyEvent) -> Result<bool, WarpError> {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return Ok(true), // Exit

            KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                // Toggle debug mode
                let mut config = self.config.lock().await;
                config.debug.enabled = !config.debug.enabled;
            }

            _ => {
                // Forward to UI
                let mut ui = self.ui.lock().await;
                ui.handle_key_event(key_event).await?;
            }
        }

        Ok(false)
    }

    async fn handle_resize(&self, width: u16, height: u16) -> Result<(), WarpError> {
        let mut terminal = self.terminal.lock().await;
        terminal.resize(width, height).await?;

        let mut ui = self.ui.lock().await;
        ui.resize(width, height).await?;

        Ok(())
    }

    async fn handle_ui_event(&self, event: UIEvent) -> Result<(), WarpError> {
        match event {
            UIEvent::PtyOutput(output) => {
                let mut ui = self.ui.lock().await;
                ui.append_output(output).await?;
            }
            UIEvent::CommandExecuted(command) => {
                let mut history = self.history_manager.lock().await;
                history.add_command(command).await?;
            }
            UIEvent::AIQuery(query) => {
                let response = self.ai_assistant.process_query(&query).await?;
                let mut ui = self.ui.lock().await;
                ui.show_ai_response(response).await?;
            }
            _ => {}
        }

        Ok(())
    }

    async fn render(&self) -> Result<(), WarpError> {
        let mut ui = self.ui.lock().await;
        ui.render().await?;
        Ok(())
    }

    pub async fn get_completions(&self, input: &str, cursor_pos: usize) -> Result<Vec<CompletionItem>, WarpError> {
        let context = CompletionContext {
            current_line: input.to_string(),
            cursor_position: cursor_pos,
            working_directory: std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            shell_type: "zsh".to_string(), // This would be detected
            command_history: vec![], // This would come from history manager
            environment_variables: std::env::vars().collect(),
            git_status: None, // This would be detected
            docker_context: None, // This would be detected
        };
        
        self.advanced_ai.get_completions(context).await
    }

    pub async fn get_smart_suggestions(&self, input: &str) -> Result<Vec<ContextualSuggestion>, WarpError> {
        let context = CompletionContext {
            current_line: input.to_string(),
            cursor_position: input.len(),
            working_directory: std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            shell_type: "zsh".to_string(),
            command_history: vec![],
            environment_variables: std::env::vars().collect(),
            git_status: None,
            docker_context: None,
        };
        
        self.advanced_ai.get_smart_suggestions(context).await
    }
}
