use super::super::{CompletionProvider, CompletionItem, CompletionType, CompletionContext};
use crate::error::WarpError;
use std::collections::HashMap;

pub struct CommandCompletionProvider {
    commands: HashMap<String, CommandInfo>,
}

#[derive(Debug, Clone)]
struct CommandInfo {
    name: String,
    description: String,
    flags: Vec<FlagInfo>,
    subcommands: Vec<String>,
}

#[derive(Debug, Clone)]
struct FlagInfo {
    name: String,
    short: Option<String>,
    description: String,
    takes_value: bool,
}

impl CommandCompletionProvider {
    pub async fn new() -> Result<Self, WarpError> {
        let mut commands = HashMap::new();
        
        // Load common commands
        commands.insert("git".to_string(), CommandInfo {
            name: "git".to_string(),
            description: "Git version control system".to_string(),
            flags: vec![
                FlagInfo {
                    name: "--version".to_string(),
                    short: None,
                    description: "Show version".to_string(),
                    takes_value: false,
                },
                FlagInfo {
                    name: "--help".to_string(),
                    short: Some("-h".to_string()),
                    description: "Show help".to_string(),
                    takes_value: false,
                },
            ],
            subcommands: vec![
                "add".to_string(), "commit".to_string(), "push".to_string(),
                "pull".to_string(), "status".to_string(), "log".to_string(),
                "branch".to_string(), "checkout".to_string(), "merge".to_string(),
            ],
        });
        
        commands.insert("docker".to_string(), CommandInfo {
            name: "docker".to_string(),
            description: "Docker container platform".to_string(),
            flags: vec![
                FlagInfo {
                    name: "--version".to_string(),
                    short: None,
                    description: "Show version".to_string(),
                    takes_value: false,
                },
            ],
            subcommands: vec![
                "run".to_string(), "build".to_string(), "ps".to_string(),
                "images".to_string(), "pull".to_string(), "push".to_string(),
                "stop".to_string(), "start".to_string(), "rm".to_string(),
            ],
        });
        
        commands.insert("ls".to_string(), CommandInfo {
            name: "ls".to_string(),
            description: "List directory contents".to_string(),
            flags: vec![
                FlagInfo {
                    name: "-l".to_string(),
                    short: None,
                    description: "Long format".to_string(),
                    takes_value: false,
                },
                FlagInfo {
                    name: "-a".to_string(),
                    short: None,
                    description: "Show hidden files".to_string(),
                    takes_value: false,
                },
                FlagInfo {
                    name: "-h".to_string(),
                    short: None,
                    description: "Human readable sizes".to_string(),
                    takes_value: false,
                },
            ],
            subcommands: vec![],
        });
        
        Ok(Self { commands })
    }
}

impl CompletionProvider for CommandCompletionProvider {
    async fn get_completions(
        &self,
        context: &CompletionContext,
    ) -> Result<Vec<CompletionItem>, WarpError> {
        let mut completions = Vec::new();
        let parts: Vec<&str> = context.current_line.split_whitespace().collect();
        
        if parts.is_empty() || (parts.len() == 1 && !context.current_line.ends_with(' ')) {
            // Complete command names
            let prefix = parts.first().unwrap_or(&"");
            for (name, info) in &self.commands {
                if name.starts_with(prefix) {
                    completions.push(CompletionItem {
                        text: name.clone(),
                        display_text: name.clone(),
                        description: Some(info.description.clone()),
                        completion_type: CompletionType::Command,
                        score: 0.9,
                        insert_text: name.clone(),
                        documentation: Some(format!("Command: {}\n{}", name, info.description)),
                    });
                }
            }
        } else if parts.len() >= 1 {
            // Complete subcommands and flags
            let command = parts[0];
            if let Some(cmd_info) = self.commands.get(command) {
                let current_word = parts.last().unwrap_or(&"");
                
                // Complete subcommands
                for subcommand in &cmd_info.subcommands {
                    if subcommand.starts_with(current_word) {
                        completions.push(CompletionItem {
                            text: subcommand.clone(),
                            display_text: subcommand.clone(),
                            description: Some(format!("{} subcommand", command)),
                            completion_type: CompletionType::Command,
                            score: 0.8,
                            insert_text: subcommand.clone(),
                            documentation: None,
                        });
                    }
                }
                
                // Complete flags
                for flag in &cmd_info.flags {
                    if flag.name.starts_with(current_word) {
                        completions.push(CompletionItem {
                            text: flag.name.clone(),
                            display_text: flag.name.clone(),
                            description: Some(flag.description.clone()),
                            completion_type: CompletionType::Flag,
                            score: 0.7,
                            insert_text: flag.name.clone(),
                            documentation: Some(flag.description.clone()),
                        });
                    }
                    
                    if let Some(short) = &flag.short {
                        if short.starts_with(current_word) {
                            completions.push(CompletionItem {
                                text: short.clone(),
                                display_text: short.clone(),
                                description: Some(flag.description.clone()),
                                completion_type: CompletionType::Flag,
                                score: 0.7,
                                insert_text: short.clone(),
                                documentation: Some(flag.description.clone()),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(completions)
    }
    
    fn provider_name(&self) -> &str {
        "command"
    }
    
    fn priority(&self) -> u8 {
        90
    }
}
