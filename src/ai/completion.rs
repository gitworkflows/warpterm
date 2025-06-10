use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub text: String,
    pub display_text: String,
    pub description: Option<String>,
    pub completion_type: CompletionType,
    pub score: f32,
    pub insert_text: String,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionType {
    Command,
    Flag,
    Argument,
    FilePath,
    Directory,
    Variable,
    Function,
    Keyword,
    Snippet,
    AIGenerated,
}

#[derive(Debug, Clone)]
pub struct CompletionContext {
    pub current_line: String,
    pub cursor_position: usize,
    pub working_directory: String,
    pub shell_type: String,
    pub command_history: Vec<String>,
    pub environment_variables: HashMap<String, String>,
    pub git_status: Option<GitStatus>,
    pub docker_context: Option<DockerContext>,
}

#[derive(Debug, Clone)]
pub struct GitStatus {
    pub branch: String,
    pub modified_files: Vec<String>,
    pub untracked_files: Vec<String>,
    pub staged_files: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DockerContext {
    pub containers: Vec<String>,
    pub images: Vec<String>,
    pub networks: Vec<String>,
}

pub struct CompletionEngine {
    providers: Vec<Box<dyn CompletionProvider>>,
    cache: Arc<Mutex<HashMap<String, Vec<CompletionItem>>>>,
    ai_provider: Arc<dyn AICompletionProvider>,
}

pub trait CompletionProvider: Send + Sync {
    async fn get_completions(
        &self,
        context: &CompletionContext,
    ) -> Result<Vec<CompletionItem>, WarpError>;
    
    fn provider_name(&self) -> &str;
    fn priority(&self) -> u8;
}

pub trait AICompletionProvider: Send + Sync {
    async fn get_ai_completions(
        &self,
        context: &CompletionContext,
        query: &str,
    ) -> Result<Vec<CompletionItem>, WarpError>;
    
    async fn explain_command(
        &self,
        command: &str,
        context: &CompletionContext,
    ) -> Result<String, WarpError>;
    
    async fn suggest_fix(
        &self,
        error: &str,
        context: &CompletionContext,
    ) -> Result<Vec<String>, WarpError>;
}

impl CompletionEngine {
    pub async fn new() -> Result<Self, WarpError> {
        let mut providers: Vec<Box<dyn CompletionProvider>> = Vec::new();
        
        // Add built-in providers
        providers.push(Box::new(CommandCompletionProvider::new().await?));
        providers.push(Box::new(FilePathCompletionProvider::new().await?));
        providers.push(Box::new(GitCompletionProvider::new().await?));
        providers.push(Box::new(DockerCompletionProvider::new().await?));
        providers.push(Box::new(HistoryCompletionProvider::new().await?));
        providers.push(Box::new(VariableCompletionProvider::new().await?));
        
        // Sort providers by priority
        providers.sort_by(|a, b| b.priority().cmp(&a.priority()));
        
        let ai_provider = Arc::new(OpenAICompletionProvider::new().await?);
        
        Ok(Self {
            providers,
            cache: Arc::new(Mutex::new(HashMap::new())),
            ai_provider,
        })
    }

    pub async fn get_completions(
        &self,
        context: &CompletionContext,
    ) -> Result<Vec<CompletionItem>, WarpError> {
        let cache_key = format!("{}:{}", context.current_line, context.cursor_position);
        
        // Check cache first
        {
            let cache = self.cache.lock().await;
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }
        
        let mut all_completions = Vec::new();
        
        // Get completions from all providers
        for provider in &self.providers {
            match provider.get_completions(context).await {
                Ok(mut completions) => {
                    all_completions.append(&mut completions);
                }
                Err(e) => {
                    log::warn!("Provider {} failed: {}", provider.provider_name(), e);
                }
            }
        }
        
        // Get AI completions if enabled
        if !context.current_line.is_empty() {
            match self.ai_provider.get_ai_completions(context, &context.current_line).await {
                Ok(mut ai_completions) => {
                    all_completions.append(&mut ai_completions);
                }
                Err(e) => {
                    log::warn!("AI completion failed: {}", e);
                }
            }
        }
        
        // Sort by score and deduplicate
        all_completions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        all_completions.dedup_by(|a, b| a.text == b.text);
        
        // Limit results
        all_completions.truncate(50);
        
        // Cache results
        {
            let mut cache = self.cache.lock().await;
            cache.insert(cache_key, all_completions.clone());
        }
        
        Ok(all_completions)
    }

    pub async fn get_smart_suggestions(
        &self,
        context: &CompletionContext,
    ) -> Result<Vec<SmartSuggestion>, WarpError> {
        let mut suggestions = Vec::new();
        
        // Analyze current context for smart suggestions
        if let Some(git_status) = &context.git_status {
            suggestions.extend(self.get_git_suggestions(git_status, context).await?);
        }
        
        if let Some(docker_context) = &context.docker_context {
            suggestions.extend(self.get_docker_suggestions(docker_context, context).await?);
        }
        
        // Get AI-powered suggestions
        suggestions.extend(self.get_ai_suggestions(context).await?);
        
        // Sort by relevance
        suggestions.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(suggestions)
    }

    async fn get_git_suggestions(
        &self,
        git_status: &GitStatus,
        context: &CompletionContext,
    ) -> Result<Vec<SmartSuggestion>, WarpError> {
        let mut suggestions = Vec::new();
        
        if !git_status.modified_files.is_empty() {
            suggestions.push(SmartSuggestion {
                title: "Stage modified files".to_string(),
                description: format!("You have {} modified files", git_status.modified_files.len()),
                command: "git add .".to_string(),
                relevance: 0.8,
                category: SuggestionCategory::Git,
            });
        }
        
        if !git_status.staged_files.is_empty() {
            suggestions.push(SmartSuggestion {
                title: "Commit staged changes".to_string(),
                description: format!("You have {} staged files", git_status.staged_files.len()),
                command: "git commit -m \"\"".to_string(),
                relevance: 0.9,
                category: SuggestionCategory::Git,
            });
        }
        
        Ok(suggestions)
    }

    async fn get_docker_suggestions(
        &self,
        docker_context: &DockerContext,
        context: &CompletionContext,
    ) -> Result<Vec<SmartSuggestion>, WarpError> {
        let mut suggestions = Vec::new();
        
        if !docker_context.containers.is_empty() {
            suggestions.push(SmartSuggestion {
                title: "View running containers".to_string(),
                description: format!("You have {} containers", docker_context.containers.len()),
                command: "docker ps".to_string(),
                relevance: 0.7,
                category: SuggestionCategory::Docker,
            });
        }
        
        Ok(suggestions)
    }

    async fn get_ai_suggestions(
        &self,
        context: &CompletionContext,
    ) -> Result<Vec<SmartSuggestion>, WarpError> {
        // This would integrate with the AI provider to get contextual suggestions
        // For now, return some basic suggestions based on context
        let mut suggestions = Vec::new();
        
        if context.current_line.is_empty() && !context.command_history.is_empty() {
            if let Some(last_command) = context.command_history.last() {
                suggestions.push(SmartSuggestion {
                    title: "Repeat last command".to_string(),
                    description: format!("Run: {}", last_command),
                    command: last_command.clone(),
                    relevance: 0.6,
                    category: SuggestionCategory::History,
                });
            }
        }
        
        Ok(suggestions)
    }
}

#[derive(Debug, Clone)]
pub struct SmartSuggestion {
    pub title: String,
    pub description: String,
    pub command: String,
    pub relevance: f32,
    pub category: SuggestionCategory,
}

#[derive(Debug, Clone)]
pub enum SuggestionCategory {
    Git,
    Docker,
    File,
    History,
    AI,
    System,
}
