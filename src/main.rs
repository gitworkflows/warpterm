use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use crossterm::{
    cursor, execute, queue,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

mod terminal_state;
mod command_executor;
mod ai_assistant;
mod theme;

use terminal_state::TerminalState;
use command_executor::CommandExecutor;
use ai_assistant::AIAssistant;
use theme::Theme;

use std::sync::Arc;
use tokio::sync::Mutex;
use clap::{Arg, Command};
use warp_terminal::{
    app::WarpApp,
    config::Config,
    logger::Logger,
    error::WarpError,
};

#[tokio::main]
async fn main() -> Result<(), WarpError> {
    // Parse command line arguments
    let matches = Command::new("warp")
        .version("1.0.0")
        .author("Warp Terminal Team")
        .about("A modern, Rust-based terminal with AI built in")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file"))
        .arg(Arg::new("theme")
            .short('t')
            .long("theme")
            .value_name("THEME")
            .help("Sets the theme"))
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .help("Enable debug mode")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    // Initialize logger
    let debug_mode = matches.get_flag("debug");
    Logger::init(debug_mode)?;

    // Load configuration
    let config_path = matches.get_one::<String>("config");
    let config = Config::load(config_path).await?;

    // Override theme if specified
    let mut final_config = config;
    if let Some(theme_name) = matches.get_one::<String>("theme") {
        final_config.ui.theme = theme_name.clone();
    }

    // Create and run the application
    let app = WarpApp::new(Arc::new(Mutex::new(final_config))).await?;
    app.run().await?;

    Ok(())
}

fn draw_header(stdout: &mut io::Stdout, theme: &Theme) -> Result<(), Box<dyn std::error::Error>> {
    queue!(
        stdout,
        SetForegroundColor(theme.accent_color),
        Print("╭─────────────────────────────────────────────────────────────────────────────╮\n"),
        Print("│                                    WARP                                     │\n"),
        Print("│                          Modern Rust Terminal                              │\n"),
        Print("╰─────────────────────────────────────────────────────────────────────────────╯\n"),
        ResetColor
    )?;
    Ok(())
}

fn draw_prompt(stdout: &mut io::Stdout, state: &TerminalState, theme: &Theme) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = terminal::size()?;
    queue!(
        stdout,
        cursor::MoveTo(0, rows - 2),
        SetForegroundColor(theme.prompt_color),
        Print("❯ "),
        SetForegroundColor(theme.text_color),
        Print(&state.current_input),
        Print("█"), // Cursor
        ResetColor
    )?;
    Ok(())
}

fn draw_history(stdout: &mut io::Stdout, state: &TerminalState, theme: &Theme) -> Result<(), Box<dyn std::error::Error>> {
    let (_, rows) = terminal::size()?;
    let start_row = 5;
    let max_lines = (rows as usize).saturating_sub(8);
    
    for (i, entry) in state.history.iter().rev().take(max_lines).enumerate() {
        queue!(
            stdout,
            cursor::MoveTo(0, start_row + i as u16),
            SetForegroundColor(theme.history_color),
            Print(entry),
            ResetColor,
            Print("\n")
        )?;
    }
    Ok(())
}

fn handle_key_event(
    key_event: KeyEvent,
    state: &mut TerminalState,
    executor: &CommandExecutor,
    ai: &AIAssistant,
) -> Result<bool, Box<dyn std::error::Error>> {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => return Ok(true), // Exit on Ctrl+C
        
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            if !state.current_input.trim().is_empty() {
                let input = state.current_input.clone();
                
                // Check for AI commands
                if input.starts_with("ai ") {
                    let query = &input[3..];
                    let response = ai.process_query(query)?;
                    state.add_history_entry(format!("❯ {}", input));
                    state.add_history_entry(format!("🤖 {}", response));
                } else {
                    // Execute regular command
                    let output = executor.execute(&input)?;
                    state.add_history_entry(format!("❯ {}", input));
                    state.add_history_entry(output);
                }
                
                state.clear_input();
            }
        }
        
        KeyEvent {
            code: KeyCode::Backspace,
            ..
        } => {
            state.backspace();
        }
        
        KeyEvent {
            code: KeyCode::Char(c),
            ..
        } => {
            state.add_char(c);
        }
        
        _ => {}
    }
    
    Ok(false)
}
