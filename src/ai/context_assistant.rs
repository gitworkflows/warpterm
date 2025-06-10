use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualAssistant {
    context_analyzer: ContextAnalyzer,
    suggestion_engine: SuggestionEngine,
    error_detector: ErrorDetector,
    learning_system: LearningSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAnalyzer {
    current_session: SessionContext,
    project_context: Option<ProjectContext>,
    user_patterns: UserPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub commands_executed: Vec<CommandExecution>,
    pub current_directory: String,
    pub environment_variables: HashMap<String, String>,
    pub active_processes: Vec<ProcessInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecution {
    pub command: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub exit_code: i32,
    pub duration: std::time::Duration,
    pub output_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub project_type: ProjectType,
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub build_tools: Vec<String>,
    pub git_info: Option<GitInfo>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    WebApp,
    MobileApp,
    Desktop,
    Library,
    Script,
    DataScience,
    DevOps,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitInfo {
    pub branch: String,
    pub remote_url: Option<String>,
    pub last_commit: String,
    pub uncommitted_changes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPatterns {
    pub frequent_commands: HashMap<String, u32>,
    pub command_sequences: Vec<CommandSequence>,
    pub preferred_flags: HashMap<String, Vec<String>>,
    pub working_hours: Option<(u8, u8)>, // (start_hour, end_hour)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSequence {
    pub commands: Vec<String>,
    pub frequency: u32,
    pub success_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionEngine {
    pub active_suggestions: Vec<ContextualSuggestion>,
    pub suggestion_history: Vec<SuggestionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualSuggestion {
    pub id: String,
    pub suggestion_type: SuggestionType,
    pub title: String,
    pub description: String,
    pub command: Option<String>,
    pub confidence: f32,
    pub context_relevance: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    CommandCompletion,
    WorkflowOptimization,
    ErrorPrevention,
    BestPractice,
    TimesSaving,
    SecurityWarning,
    PerformanceImprovement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionResult {
    pub suggestion_id: String,
    pub accepted: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetector {
    pub common_errors: HashMap<String, ErrorPattern>,
    pub user_specific_errors: Vec<UserError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPattern {
    pub pattern: String,
    pub description: String,
    pub suggested_fixes: Vec<String>,
    pub prevention_tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserError {
    pub command: String,
    pub error_message: String,
    pub frequency: u32,
    pub last_occurrence: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSystem {
    pub user_preferences: UserPreferences,
    pub adaptation_metrics: AdaptationMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub verbosity_level: VerbosityLevel,
    pub suggestion_frequency: SuggestionFrequency,
    pub preferred_explanation_style: ExplanationStyle,
    pub auto_execute_safe_suggestions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerbosityLevel {
    Minimal,
    Moderate,
    Detailed,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionFrequency {
    Rare,
    Occasional,
    Frequent,
    Continuous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExplanationStyle {
    Concise,
    StepByStep,
    WithExamples,
    Technical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationMetrics {
    pub suggestion_acceptance_rate: f32,
    pub command_success_rate: f32,
    pub learning_progress: f32,
    pub user_satisfaction_score: f32,
}

impl ContextualAssistant {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            context_analyzer: ContextAnalyzer::new().await?,
            suggestion_engine: SuggestionEngine::new().await?,
            error_detector: ErrorDetector::new().await?,
            learning_system: LearningSystem::new().await?,
        })
    }

    pub async fn analyze_context(&mut self, command: &str) -> Result<Vec<ContextualSuggestion>, WarpError> {
        // Update session context
        self.context_analyzer.update_session_context(command).await?;
        
        // Detect project context if not already detected
        if self.context_analyzer.project_context.is_none() {
            self.context_analyzer.detect_project_context().await?;
        }
        
        // Generate contextual suggestions
        let mut suggestions = Vec::new();
        
        // Command completion suggestions
        suggestions.extend(self.get_command_completion_suggestions(command).await?);
        
        // Workflow optimization suggestions
        suggestions.extend(self.get_workflow_suggestions().await?);
        
        // Error prevention suggestions
        suggestions.extend(self.get_error_prevention_suggestions(command).await?);
        
        // Best practice suggestions
        suggestions.extend(self.get_best_practice_suggestions(command).await?);
        
        // Filter and rank suggestions
        self.filter_and_rank_suggestions(&mut suggestions).await?;
        
        Ok(suggestions)
    }

    async fn get_command_completion_suggestions(&self, command: &str) -> Result<Vec<ContextualSuggestion>, WarpError> {
        let mut suggestions = Vec::new();
        
        // Analyze partial command
        if !command.is_empty() && !command.ends_with(' ') {
            // Get intelligent completions based on context
            if let Some(project_context) = &self.context_analyzer.project_context {
                match project_context.project_type {
                    ProjectType::WebApp => {
                        if command.starts_with("npm") {
                            suggestions.push(ContextualSuggestion {
                                id: uuid::Uuid::new_v4().to_string(),
                                suggestion_type: SuggestionType::CommandCompletion,
                                title: "npm run dev".to_string(),
                                description: "Start development server".to_string(),
                                command: Some("npm run dev".to_string()),
                                confidence: 0.8,
                                context_relevance: 0.9,
                                created_at: chrono::Utc::now(),
                            });
                        }
                    }
                    ProjectType::DataScience => {
                        if command.starts_with("python") {
                            suggestions.push(ContextualSuggestion {
                                id: uuid::Uuid::new_v4().to_string(),
                                suggestion_type: SuggestionType::CommandCompletion,
                                title: "python -m jupyter notebook".to_string(),
                                description: "Start Jupyter notebook".to_string(),
                                command: Some("python -m jupyter notebook".to_string()),
                                confidence: 0.7,
                                context_relevance: 0.8,
                                created_at: chrono::Utc::now(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        
        Ok(suggestions)
    }

    async fn get_workflow_suggestions(&self) -> Result<Vec<ContextualSuggestion>, WarpError> {
        let mut suggestions = Vec::new();
        
        // Analyze command patterns for workflow optimization
        let patterns = &self.context_analyzer.user_patterns;
        
        // Look for repetitive command sequences
        for sequence in &patterns.command_sequences {
            if sequence.frequency > 5 && sequence.commands.len() > 2 {
                suggestions.push(ContextualSuggestion {
                    id: uuid::Uuid::new_v4().to_string(),
                    suggestion_type: SuggestionType::WorkflowOptimization,
                    title: "Create workflow for common sequence".to_string(),
                    description: format!("You often run: {}", sequence.commands.join(" && ")),
                    command: Some(format!("# Create alias or workflow for: {}", sequence.commands.join(" && "))),
                    confidence: 0.6,
                    context_relevance: 0.7,
                    created_at: chrono::Utc::now(),
                });
            }
        }
        
        Ok(suggestions)
    }

    async fn get_error_prevention_suggestions(&self, command: &str) -> Result<Vec<ContextualSuggestion>, WarpError> {
        let mut suggestions = Vec::new();
        
        // Check for potentially dangerous commands
        if command.contains("rm -rf") && !command.contains("--") {
            suggestions.push(ContextualSuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::SecurityWarning,
                title: "⚠️ Dangerous command detected".to_string(),
                description: "rm -rf can permanently delete files. Consider using trash or adding safety flags.".to_string(),
                command: Some("trash".to_string()),
                confidence: 0.9,
                context_relevance: 1.0,
                created_at: chrono::Utc::now(),
            });
        }
        
        // Check for common typos
        if command == "cd.." {
            suggestions.push(ContextualSuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::ErrorPrevention,
                title: "Did you mean 'cd ..'?".to_string(),
                description: "Missing space in cd command".to_string(),
                command: Some("cd ..".to_string()),
                confidence: 0.95,
                context_relevance: 0.9,
                created_at: chrono::Utc::now(),
            });
        }
        
        Ok(suggestions)
    }

    async fn get_best_practice_suggestions(&self, command: &str) -> Result<Vec<ContextualSuggestion>, WarpError> {
        let mut suggestions = Vec::new();
        
        // Git best practices
        if command.starts_with("git commit") && !command.contains("-m") {
            suggestions.push(ContextualSuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::BestPractice,
                title: "Add commit message".to_string(),
                description: "Consider adding a descriptive commit message with -m".to_string(),
                command: Some(format!("{} -m \"\"", command)),
                confidence: 0.7,
                context_relevance: 0.8,
                created_at: chrono::Utc::now(),
            });
        }
        
        // Docker best practices
        if command.starts_with("docker run") && !command.contains("--rm") {
            suggestions.push(ContextualSuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::BestPractice,
                title: "Auto-remove container".to_string(),
                description: "Consider adding --rm to automatically remove container when it exits".to_string(),
                command: Some(command.replace("docker run", "docker run --rm")),
                confidence: 0.6,
                context_relevance: 0.7,
                created_at: chrono::Utc::now(),
            });
        }
        
        Ok(suggestions)
    }

    async fn filter_and_rank_suggestions(&self, suggestions: &mut Vec<ContextualSuggestion>) -> Result<(), WarpError> {
        // Remove low-confidence suggestions
        suggestions.retain(|s| s.confidence > 0.3);
        
        // Sort by relevance and confidence
        suggestions.sort_by(|a, b| {
            let score_a = a.confidence * a.context_relevance;
            let score_b = b.confidence * b.context_relevance;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Limit to top suggestions based on user preferences
        let max_suggestions = match self.learning_system.user_preferences.suggestion_frequency {
            SuggestionFrequency::Rare => 1,
            SuggestionFrequency::Occasional => 3,
            SuggestionFrequency::Frequent => 5,
            SuggestionFrequency::Continuous => 10,
        };
        
        suggestions.truncate(max_suggestions);
        
        Ok(())
    }

    pub async fn learn_from_interaction(&mut self, suggestion_id: &str, accepted: bool, feedback: Option<String>) -> Result<(), WarpError> {
        // Record the interaction
        self.suggestion_engine.suggestion_history.push(SuggestionResult {
            suggestion_id: suggestion_id.to_string(),
            accepted,
            timestamp: chrono::Utc::now(),
            user_feedback: feedback,
        });
        
        // Update learning metrics
        self.learning_system.update_metrics().await?;
        
        // Adapt suggestions based on feedback
        if !accepted {
            self.learning_system.adapt_to_rejection(suggestion_id).await?;
        }
        
        Ok(())
    }
}

impl ContextAnalyzer {
    async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            current_session: SessionContext {
                start_time: chrono::Utc::now(),
                commands_executed: Vec::new(),
                current_directory: std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                environment_variables: std::env::vars().collect(),
                active_processes: Vec::new(),
            },
            project_context: None,
            user_patterns: UserPatterns {
                frequent_commands: HashMap::new(),
                command_sequences: Vec::new(),
                preferred_flags: HashMap::new(),
                working_hours: None,
            },
        })
    }

    async fn update_session_context(&mut self, command: &str) -> Result<(), WarpError> {
        self.current_session.commands_executed.push(CommandExecution {
            command: command.to_string(),
            timestamp: chrono::Utc::now(),
            exit_code: 0, // This would be updated after execution
            duration: std::time::Duration::from_secs(0), // This would be measured
            output_length: 0, // This would be measured
        });
        
        // Update user patterns
        *self.user_patterns.frequent_commands.entry(command.to_string()).or_insert(0) += 1;
        
        Ok(())
    }

    async fn detect_project_context(&mut self) -> Result<(), WarpError> {
        let current_dir = std::path::Path::new(&self.current_session.current_directory);
        
        // Check for common project files
        let mut project_type = ProjectType::Unknown;
        let mut languages = Vec::new();
        let mut frameworks = Vec::new();
        let mut build_tools = Vec::new();
        
        // Check for web development
        if current_dir.join("package.json").exists() {
            project_type = ProjectType::WebApp;
            languages.push("JavaScript".to_string());
            build_tools.push("npm".to_string());
            
            // Check for specific frameworks
            if current_dir.join("next.config.js").exists() {
                frameworks.push("Next.js".to_string());
            }
            if current_dir.join("nuxt.config.js").exists() {
                frameworks.push("Nuxt.js".to_string());
            }
        }
        
        // Check for Python projects
        if current_dir.join("requirements.txt").exists() || current_dir.join("pyproject.toml").exists() {
            languages.push("Python".to_string());
            
            // Check for data science indicators
            if current_dir.join("jupyter").exists() || 
               current_dir.join("notebooks").exists() {
                project_type = ProjectType::DataScience;
            }
        }
        
        // Check for Rust projects
        if current_dir.join("Cargo.toml").exists() {
            languages.push("Rust".to_string());
            build_tools.push("cargo".to_string());
        }
        
        // Check for Git
        let git_info = if current_dir.join(".git").exists() {
            // This would integrate with git to get actual info
            Some(GitInfo {
                branch: "main".to_string(),
                remote_url: None,
                last_commit: "".to_string(),
                uncommitted_changes: false,
            })
        } else {
            None
        };
        
        self.project_context = Some(ProjectContext {
            project_type,
            languages,
            frameworks,
            build_tools,
            git_info,
            dependencies: Vec::new(),
        });
        
        Ok(())
    }
}

impl SuggestionEngine {
    async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            active_suggestions: Vec::new(),
            suggestion_history: Vec::new(),
        })
    }
}

impl ErrorDetector {
    async fn new() -> Result<Self, WarpError> {
        let mut common_errors = HashMap::new();
        
        common_errors.insert("command not found".to_string(), ErrorPattern {
            pattern: "command not found".to_string(),
            description: "The command you entered is not recognized".to_string(),
            suggested_fixes: vec![
                "Check the spelling of the command".to_string(),
                "Install the required package".to_string(),
                "Add the command to your PATH".to_string(),
            ],
            prevention_tips: vec![
                "Use tab completion to avoid typos".to_string(),
                "Use 'which command' to check if a command exists".to_string(),
            ],
        });
        
        Ok(Self {
            common_errors,
            user_specific_errors: Vec::new(),
        })
    }
}

impl LearningSystem {
    async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            user_preferences: UserPreferences {
                verbosity_level: VerbosityLevel::Moderate,
                suggestion_frequency: SuggestionFrequency::Occasional,
                preferred_explanation_style: ExplanationStyle::StepByStep,
                auto_execute_safe_suggestions: false,
            },
            adaptation_metrics: AdaptationMetrics {
                suggestion_acceptance_rate: 0.0,
                command_success_rate: 0.0,
                learning_progress: 0.0,
                user_satisfaction_score: 0.0,
            },
        })
    }

    async fn update_metrics(&mut self) -> Result<(), WarpError> {
        // This would calculate metrics based on interaction history
        Ok(())
    }

    async fn adapt_to_rejection(&mut self, _suggestion_id: &str) -> Result<(), WarpError> {
        // This would adapt the suggestion algorithm based on rejections
        Ok(())
    }
}
