use std::collections::HashMap;

pub struct AIAssistant {
    command_explanations: HashMap<String, String>,
    suggestions: HashMap<String, Vec<String>>,
}

impl AIAssistant {
    pub fn new() -> Self {
        let mut command_explanations = HashMap::new();
        let mut suggestions = HashMap::new();

        // Pre-populate with common command explanations
        command_explanations.insert("ls".to_string(), "Lists files and directories in the current directory".to_string());
        command_explanations.insert("cd".to_string(), "Changes the current working directory".to_string());
        command_explanations.insert("pwd".to_string(), "Prints the current working directory path".to_string());
        command_explanations.insert("cat".to_string(), "Displays the contents of a file".to_string());
        command_explanations.insert("grep".to_string(), "Searches for patterns in text files".to_string());
        command_explanations.insert("find".to_string(), "Searches for files and directories".to_string());

        // Pre-populate with task suggestions
        suggestions.insert("list files".to_string(), vec!["ls -la".to_string(), "find . -type f".to_string()]);
        suggestions.insert("search text".to_string(), vec!["grep -r 'pattern' .".to_string(), "find . -name '*.txt' -exec grep 'pattern' {} +".to_string()]);
        suggestions.insert("file permissions".to_string(), vec!["chmod 755 filename".to_string(), "ls -la".to_string()]);

        Self {
            command_explanations,
            suggestions,
        }
    }

    pub fn process_query(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let query = query.trim().to_lowercase();

        if query.starts_with("explain ") {
            let command = &query[8..];
            self.explain_command(command)
        } else if query.starts_with("suggest ") {
            let task = &query[8..];
            self.suggest_commands(task)
        } else if query.starts_with("debug ") {
            let error = &query[6..];
            self.debug_error(error)
        } else {
            self.general_help(&query)
        }
    }

    fn explain_command(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(explanation) = self.command_explanations.get(command) {
            Ok(format!("📖 {}: {}", command, explanation))
        } else {
            Ok(format!("🤔 I don't have specific information about '{}', but you can try 'man {}' for detailed documentation.", command, command))
        }
    }

    fn suggest_commands(&self, task: &str) -> Result<String, Box<dyn std::error::Error>> {
        for (key, commands) in &self.suggestions {
            if task.contains(key) {
                let suggestions = commands.join("\n  • ");
                return Ok(format!("💡 Suggestions for '{}':\n  • {}", task, suggestions));
            }
        }

        Ok(format!("🤔 I don't have specific suggestions for '{}'. Try describing your task differently or use 'help' for available commands.", task))
    }

    fn debug_error(&self, error: &str) -> Result<String, Box<dyn std::error::Error>> {
        let error_lower = error.to_lowercase();
        
        if error_lower.contains("permission denied") {
            Ok("🔧 Permission denied errors can be fixed by:\n  • Using 'sudo' for admin commands\n  • Checking file permissions with 'ls -la'\n  • Using 'chmod' to modify permissions".to_string())
        } else if error_lower.contains("command not found") {
            Ok("🔧 Command not found errors can be fixed by:\n  • Checking if the command is installed\n  • Verifying the command spelling\n  • Adding the command's directory to PATH".to_string())
        } else if error_lower.contains("no such file") {
            Ok("🔧 File not found errors can be fixed by:\n  • Checking the file path with 'ls'\n  • Using 'find' to locate the file\n  • Verifying you're in the correct directory with 'pwd'".to_string())
        } else {
            Ok(format!("🔧 For the error '{}', try:\n  • Reading the full error message carefully\n  • Checking command syntax with 'man <command>'\n  • Searching online for the specific error", error))
        }
    }

    fn general_help(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("🤖 I can help with:\n  • 'ai explain <command>' - Explain commands\n  • 'ai suggest <task>' - Suggest commands for tasks\n  • 'ai debug <error>' - Help with errors\n\nYour query: '{}'", query))
    }
}
