use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use crossterm::{
    event::KeyEvent,
    style::Color,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal as RatatuiTerminal,
    widgets::{Block, Borders, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier},
    text::{Spans, Span},
};

use crate::{config::Config, error::WarpError};

#[derive(Debug, Clone)]
pub enum UIEvent {
    PtyOutput(String),
    CommandExecuted(String),
    AIQuery(String),
    ThemeChanged(String),
    Resize(u16, u16),
}

pub struct UI {
    config: Arc<Mutex<Config>>,
    terminal: RatatuiTerminal<CrosstermBackend<std::io::Stdout>>,
    event_sender: mpsc::UnboundedSender<UIEvent>,
    output_buffer: Vec<String>,
    input_buffer: String,
    cursor_position: usize,
    ai_response: Option<String>,
}

impl UI {
    pub async fn new(
        config: Arc<Mutex<Config>>,
        event_sender: mpsc::UnboundedSender<UIEvent>,
    ) -> Result<Self, WarpError> {
        let backend = CrosstermBackend::new(std::io::stdout());
        let terminal = RatatuiTerminal::new(backend)?;

        Ok(Self {
            config,
            terminal,
            event_sender,
            output_buffer: Vec::new(),
            input_buffer: String::new(),
            cursor_position: 0,
            ai_response: None,
        })
    }

    pub async fn render(&mut self) -> Result<(), WarpError> {
        let config = self.config.lock().await;
        
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // Header
                    Constraint::Min(0),     // Main content
                    Constraint::Length(3),  // Input
                    Constraint::Length(5),  // AI response (if any)
                ].as_ref())
                .split(f.size());

            // Header
            let header = Paragraph::new("🚀 Warp Terminal - Modern Rust Terminal with AI")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(header, chunks[0]);

            // Main content (output)
            let output_items: Vec<ListItem> = self.output_buffer
                .iter()
                .map(|line| ListItem::new(line.as_ref()))
                .collect();
            
            let output_list = List::new(output_items)
                .block(Block::default().borders(Borders::ALL).title("Output"))
                .style(Style::default().fg(Color::White));
            f.render_widget(output_list, chunks[1]);

            // Input
            let input = Paragraph::new(self.input_buffer.as_ref())
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .style(Style::default().fg(Color::Green));
            f.render_widget(input, chunks[2]);

            // AI Response (if any)
            if let Some(ref response) = self.ai_response {
                let ai_widget = Paragraph::new(response.as_ref())
                    .block(Block::default().borders(Borders::ALL).title("🤖 AI Assistant"))
                    .style(Style::default().fg(Color::Yellow));
                f.render_widget(ai_widget, chunks[3]);
            }
        })?;

        Ok(())
    }

    pub async fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), WarpError> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event {
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                if !self.input_buffer.trim().is_empty() {
                    let command = self.input_buffer.clone();
                    self.output_buffer.push(format!("❯ {}", command));
                    
                    // Check for AI commands
                    if command.starts_with("ai ") {
                        let query = command[3..].to_string();
                        let _ = self.event_sender.send(UIEvent::AIQuery(query));
                    } else {
                        let _ = self.event_sender.send(UIEvent::CommandExecuted(command));
                    }
                    
                    self.input_buffer.clear();
                    self.cursor_position = 0;
                }
            }
            
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.input_buffer.remove(self.cursor_position);
                }
            }
            
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.input_buffer.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            
            _ => {}
        }

        Ok(())
    }

    pub async fn append_output(&mut self, output: String) -> Result<(), WarpError> {
        for line in output.lines() {
            self.output_buffer.push(line.to_string());
        }
        
        // Keep only last 1000 lines
        if self.output_buffer.len() > 1000 {
            self.output_buffer.drain(0..self.output_buffer.len() - 1000);
        }
        
        Ok(())
    }

    pub async fn show_ai_response(&mut self, response: String) -> Result<(), WarpError> {
        self.ai_response = Some(response);
        Ok(())
    }

    pub async fn resize(&mut self, width: u16, height: u16) -> Result<(), WarpError> {
        let _ = self.event_sender.send(UIEvent::Resize(width, height));
        Ok(())
    }
}
