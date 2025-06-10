use crate::error::WarpError;
use crate::terminal_state::TerminalState;

pub enum Command {
    Clear,
    Exit,
    Help,
    Ai(String),
    Unknown(String),
}

impl Command {
    pub fn parse(input: &str) -> Command {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Command::Unknown(input.to_string());
        }

        match parts[0].to_lowercase().as_str() {
            "clear" => Command::Clear,
            "exit" => Command::Exit,
            "help" => Command::Help,
            "ai" => {
                if parts.len() > 1 {
                    Command::Ai(parts[1..].join(" "))
                } else {
                    Command::Ai(String::new())
                }
            }
            _ => Command::Unknown(input.to_string()),
        }
    }

    pub fn execute(&self, state: &mut TerminalState) -> Result<(), WarpError> {
        match self {
            Command::Clear => {
                state.clear_input();
                state.history.clear();
                Ok(())
            }
            Command::Exit => {
                // Exit handling will be implemented in main loop
                Ok(())
            }
            Command::Help => {
                state.add_history_entry(
                    "Available commands: clear, exit, help, ai <query>".to_string(),
                );
                Ok(())
            }
            Command::Ai(query) => {
                state.add_history_entry(format!("AI processing: {}", query));
                // Actual AI integration will be added later
                Ok(())
            }
            Command::Unknown(cmd) => {
                Err(WarpError::CommandError(format!("Unknown command: {}", cmd)))
            }
        }
    }
}
