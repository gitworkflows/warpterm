use std::process::{Command, Stdio};
use std::env;

pub struct CommandExecutor {
    current_dir: std::path::PathBuf,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            current_dir: env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")),
        }
    }

    pub fn execute(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return Ok(String::new());
        }

        // Handle built-in commands
        match trimmed {
            "help" => return Ok(self.help_text()),
            "clear" => return Ok("\x1b[2J\x1b[H".to_string()),
            "pwd" => return Ok(format!("{}", self.current_dir.display())),
            _ => {}
        }

        // Handle cd command
        if trimmed.starts_with("cd ") {
            let path = &trimmed[3..].trim();
            return self.change_directory(path);
        }

        // Execute external command
        self.execute_external_command(trimmed)
    }

    fn help_text(&self) -> String {
        r#"Warp Terminal - Available Commands:

Built-in Commands:
  help          - Show this help message
  clear         - Clear the terminal screen
  pwd           - Print working directory
  cd <path>     - Change directory
  ai <query>    - Ask AI assistant

External Commands:
  ls, cat, echo, grep, find, etc. - Standard shell commands

AI Features:
  ai explain <command>    - Explain what a command does
  ai suggest <task>       - Get command suggestions for a task
  ai debug <error>        - Help debug command errors

Press Ctrl+C to exit."#.to_string()
    }

    fn change_directory(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let new_path = if path.starts_with('/') {
            std::path::PathBuf::from(path)
        } else {
            self.current_dir.join(path)
        };

        if new_path.exists() && new_path.is_dir() {
            env::set_current_dir(&new_path)?;
            Ok(format!("Changed directory to: {}", new_path.display()))
        } else {
            Ok(format!("Directory not found: {}", path))
        }
    }

    fn execute_external_command(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(String::new());
        }

        let output = Command::new(parts[0])
            .args(&parts[1..])
            .current_dir(&self.current_dir)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                if !stderr.is_empty() {
                    Ok(format!("{}\n❌ {}", stdout, stderr))
                } else {
                    Ok(stdout.to_string())
                }
            }
            Err(e) => Ok(format!("Command failed: {}", e)),
        }
    }
}
