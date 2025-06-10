use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::WarpError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub suggestions: Vec<String>,
    pub confidence: f32,
}

pub struct AIAssistant {
    config: Arc<Mutex<Config>>,
    knowledge_base: HashMap<String, String>,
    command_patterns: HashMap<String, Vec<String>>,
    context_history: Vec<String>,
}

impl AIAssistant {
    pub async fn new(config: Arc<Mutex<Config>>) -> Result<Self, WarpError> {
        let mut knowledge_base = HashMap::new();
        let mut command_patterns = HashMap::new();

        // Initialize knowledge base
        Self::initialize_knowledge_base(&mut knowledge_base);
        Self::initialize_command_patterns(&mut command_patterns);

        Ok(Self {
            config,
            knowledge_base,
            command_patterns,
            context_history: Vec::new(),
        })
    }

    fn initialize_knowledge_base(kb: &mut HashMap<String, String>) {
        kb.insert("ls".to_string(), "Lists directory contents. Use -la for detailed view with hidden files.".to_string());
        kb.insert("cd".to_string(), "Changes current directory. Use 'cd ..' to go up one level.".to_string());
        kb.insert("grep".to_string(), "Searches for patterns in files. Use -r for recursive search.".to_string());
        kb.insert("find".to_string(), "Searches for files and directories. Use -name for filename patterns.".to_string());
        kb.insert("ps".to_string(), "Shows running processes. Use 'ps aux' for detailed process list.".to_string());
        kb.insert("kill".to_string(), "Terminates processes. Use 'kill -9' for force kill.".to_string());
        kb.insert("chmod".to_string(), "Changes file permissions. Use numeric notation like 755 or 644.".to_string());
        kb.insert("chown".to_string(), "Changes file ownership. Format: chown user:group filename".to_string());
        kb.insert("tar".to_string(), "Archives files. Use 'tar -czf archive.tar.gz files' to create compressed archive.".to_string());
        kb.insert("ssh".to_string(), "Secure shell for remote connections. Format: ssh user@hostname".to_string());
    }

    fn initialize_command_patterns(patterns: &mut HashMap<String, Vec<String>>) {
        patterns.insert("list files".to_string(), vec!["ls -la".to_string(), "find . -type f".to_string()]);
        patterns.insert("search text".to_string(), vec!["grep -r 'pattern' .".to_string(), "find . -name '*.txt' -exec grep 'pattern' {} +".to_string()]);
        patterns.insert("process management".to_string(), vec!["ps aux".to_string(), "top".to_string(), "htop".to_string()]);
        patterns.insert("file permissions".to_string(), vec!["chmod 755 file".to_string(), "ls -la".to_string()]);
        patterns.insert("network".to_string(), vec!["netstat -tulpn".to_string(), "ss -tulpn".to_string()]);
        patterns.insert("disk usage".to_string(), vec!["df -h".to_string(), "du -sh *".to_string()]);
        patterns.insert("system info".to_string(), vec!["uname -a".to_string(), "lscpu".to_string(), "free -h".to_string()]);
    }

    pub async fn process_query(&self, query: &str) -> Result<AIResponse, WarpError> {
        let query_lower = query.to_lowercase();

        // Check for command explanations
        if query_lower.starts_with("explain ") {
            let command = &query_lower[8..].trim();
            return self.explain_command(command).await;
        }

        // Check for suggestions
        if query_lower.starts_with("suggest ") || query_lower.starts_with("how to ") {
            let task = if query_lower.starts_with("suggest ") {
                &query_lower[8..]
            } else {
                &query_lower[7..]
            };
            return self.suggest_commands(task).await;
        }

        // Check for debugging help
        if query_lower.starts_with("debug ") || query_lower.starts_with("error ") {
            let error = if query_lower.starts_with("debug ") {
                &query_lower[6..]
            } else {
                &query_lower[6..]
            };
            return self.debug_error(error).await;
        }

        // General AI assistance
        self.general_assistance(&query_lower).await
    }

    async fn explain_command(&self, command: &str) -> Result<AIResponse, WarpError> {
        if let Some(explanation) = self.knowledge_base.get(command) {
            Ok(AIResponse {
                content: format!("📖 {}: {}", command, explanation),
                suggestions: vec![
                    format!("man {}", command),
                    format!("{} --help", command),
                ],
                confidence: 0.9,
            })
        } else {
            Ok(AIResponse {
                content: format!("🤔 I don't have specific information about '{}'. Try 'man {}' for detailed documentation.", command, command),
                suggestions: vec![
                    format!("man {}", command),
                    format!("{} --help", command),
                    "apropos keyword".to_string(),
                ],
                confidence: 0.3,
            })
        }
    }

    async fn suggest_commands(&self, task: &str) -> Result<AIResponse, WarpError> {
        for (pattern, commands) in &self.command_patterns {
            if task.contains(pattern) {
                return Ok(AIResponse {
                    content: format!("💡 Suggestions for '{}':", task),
                    suggestions: commands.clone(),
                    confidence: 0.8,
                });
            }
        }

        // Fuzzy matching for common tasks
        let suggestions = self.fuzzy_match_suggestions(task);
        Ok(AIResponse {
            content: format!("🤔 Here are some general suggestions for '{}':", task),
            suggestions,
            confidence: 0.5,
        })
    }

    async fn debug_error(&self, error: &str) -> Result<AIResponse, WarpError> {
        let error_lower = error.to_lowercase();
        
        if error_lower.contains("permission denied") {
            Ok(AIResponse {
                content: "🔧 Permission denied errors can be fixed by:".to_string(),
                suggestions: vec![
                    "sudo command".to_string(),
                    "ls -la (check permissions)".to_string(),
                    "chmod +x filename".to_string(),
                    "chown user:group filename".to_string(),
                ],
                confidence: 0.9,
            })
        } else if error_lower.contains("command not found") {
            Ok(AIResponse {
                content: "🔧 Command not found errors can be fixed by:".to_string(),
                suggestions: vec![
                    "which command_name".to_string(),
                    "echo $PATH".to_string(),
                    "apt install package_name".to_string(),
                    "brew install package_name".to_string(),
                ],
                confidence: 0.9,
            })
        } else if error_lower.contains("no such file") {
            Ok(AIResponse {
                content: "🔧 File not found errors can be fixed by:".to_string(),
                suggestions: vec![
                    "ls -la".to_string(),
                    "find . -name 'filename'".to_string(),
                    "pwd (check current directory)".to_string(),
                ],
                confidence: 0.9,
            })
        } else {
            Ok(AIResponse {
                content: format!("🔧 For the error '{}', try:", error),
                suggestions: vec![
                    "Check the command syntax".to_string(),
                    "Read error messages carefully".to_string(),
                    "Search online for the specific error".to_string(),
                ],
                confidence: 0.6,
            })
        }
    }

    async fn general_assistance(&self, query: &str) -> Result<AIResponse, WarpError> {
        Ok(AIResponse {
            content: "🤖 I can help with:".to_string(),
            suggestions: vec![
                "explain <command> - Explain commands".to_string(),
                "suggest <task> - Suggest commands for tasks".to_string(),
                "debug <error> - Help with errors".to_string(),
                "how to <task> - Get step-by-step help".to_string(),
            ],
            confidence: 0.7,
        })
    }

    fn fuzzy_match_suggestions(&self, task: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if task.contains("file") {
            suggestions.extend(vec!["ls -la".to_string(), "find . -name '*pattern*'".to_string()]);
        }
        if task.contains("process") {
            suggestions.extend(vec!["ps aux".to_string(), "top".to_string()]);
        }
        if task.contains("network") {
            suggestions.extend(vec!["netstat -tulpn".to_string(), "ping hostname".to_string()]);
        }
        
        if suggestions.is_empty() {
            suggestions.push("help".to_string());
        }
        
        suggestions
    }

    pub async fn start_background_processing(&self) {
        // Background AI processing tasks
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            // Perform background AI tasks like learning from user patterns
        }
    }
}
