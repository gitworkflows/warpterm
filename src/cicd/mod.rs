use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

pub mod github_actions;
pub mod gitlab_ci;
pub mod jenkins;
pub mod azure_devops;
pub mod circleci;
pub mod travis_ci;
pub mod pipeline_manager;
pub mod webhook_handler;
pub mod deployment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CICDConfig {
    pub enabled_providers: Vec<CICDProvider>,
    pub webhook_secret: String,
    pub auto_deploy_enabled: bool,
    pub test_required: bool,
    pub security_scan_required: bool,
    pub approval_required: bool,
    pub notification_channels: Vec<NotificationChannel>,
    pub deployment_environments: Vec<DeploymentEnvironment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CICDProvider {
    GitHubActions,
    GitLabCI,
    Jenkins,
    AzureDevOps,
    CircleCI,
    TravisCI,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub provider: CICDProvider,
    pub repository: Repository,
    pub stages: Vec<PipelineStage>,
    pub triggers: Vec<PipelineTrigger>,
    pub environment_variables: HashMap<String, String>,
    pub secrets: HashMap<String, String>,
    pub notifications: Vec<NotificationConfig>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: PipelineStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub url: String,
    pub branch: String,
    pub access_token: Option<String>,
    pub ssh_key: Option<String>,
    pub webhook_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub name: String,
    pub stage_type: StageType,
    pub commands: Vec<String>,
    pub environment: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub timeout: u64,
    pub retry_count: u32,
    pub allow_failure: bool,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageType {
    Build,
    Test,
    SecurityScan,
    QualityGate,
    Package,
    Deploy,
    Validate,
    Rollback,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub path: String,
    pub artifact_type: ArtifactType,
    pub retention_days: u32,
    pub public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    Binary,
    Package,
    Report,
    Logs,
    Coverage,
    Documentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineTrigger {
    Push { branches: Vec<String> },
    PullRequest { target_branches: Vec<String> },
    Schedule { cron: String },
    Manual,
    Tag { pattern: String },
    Webhook { event: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRun {
    pub id: String,
    pub pipeline_id: String,
    pub run_number: u64,
    pub commit_sha: String,
    pub branch: String,
    pub triggered_by: String,
    pub trigger_type: PipelineTrigger,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: PipelineStatus,
    pub stages: Vec<StageRun>,
    pub artifacts: Vec<Artifact>,
    pub logs: Vec<LogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageRun {
    pub stage_name: String,
    pub status: PipelineStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<std::time::Duration>,
    pub exit_code: Option<i32>,
    pub logs: Vec<LogEntry>,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub message: String,
    pub stage: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentEnvironment {
    pub name: String,
    pub environment_type: EnvironmentType,
    pub url: Option<String>,
    pub variables: HashMap<String, String>,
    pub secrets: HashMap<String, String>,
    pub approval_required: bool,
    pub auto_promote: bool,
    pub health_checks: Vec<HealthCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentType {
    Development,
    Staging,
    Production,
    Testing,
    Preview,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub url: String,
    pub method: String,
    pub expected_status: u16,
    pub timeout: u64,
    pub retry_count: u32,
    pub interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub channel: NotificationChannel,
    pub events: Vec<PipelineEvent>,
    pub conditions: Vec<NotificationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email { addresses: Vec<String> },
    Slack { webhook_url: String, channel: String },
    Discord { webhook_url: String },
    Teams { webhook_url: String },
    Webhook { url: String, headers: HashMap<String, String> },
    SMS { numbers: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineEvent {
    Started,
    Completed,
    Failed,
    Cancelled,
    StageCompleted,
    StageFailed,
    DeploymentStarted,
    DeploymentCompleted,
    DeploymentFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationCondition {
    Always,
    OnFailure,
    OnSuccess,
    OnChange,
    OnFirstFailure,
    OnRecovery,
}

pub struct CICDManager {
    config: Arc<Mutex<CICDConfig>>,
    pipeline_manager: Arc<pipeline_manager::PipelineManager>,
    webhook_handler: Arc<webhook_handler::WebhookHandler>,
    deployment_manager: Arc<deployment::DeploymentManager>,
    providers: HashMap<CICDProvider, Box<dyn CICDProviderTrait>>,
    active_runs: Arc<Mutex<HashMap<String, PipelineRun>>>,
}

#[async_trait::async_trait]
pub trait CICDProviderTrait: Send + Sync {
    async fn create_pipeline(&self, pipeline: &Pipeline) -> Result<String, WarpError>;
    async fn update_pipeline(&self, pipeline: &Pipeline) -> Result<(), WarpError>;
    async fn delete_pipeline(&self, pipeline_id: &str) -> Result<(), WarpError>;
    async fn trigger_pipeline(&self, pipeline_id: &str, parameters: HashMap<String, String>) -> Result<String, WarpError>;
    async fn get_pipeline_status(&self, run_id: &str) -> Result<PipelineStatus, WarpError>;
    async fn get_pipeline_logs(&self, run_id: &str) -> Result<Vec<LogEntry>, WarpError>;
    async fn cancel_pipeline(&self, run_id: &str) -> Result<(), WarpError>;
    async fn get_artifacts(&self, run_id: &str) -> Result<Vec<Artifact>, WarpError>;
}

impl CICDManager {
    pub async fn new() -> Result<Self, WarpError> {
        let config = Arc::new(Mutex::new(CICDConfig::default()));
        let pipeline_manager = Arc::new(pipeline_manager::PipelineManager::new().await?);
        let webhook_handler = Arc::new(webhook_handler::WebhookHandler::new().await?);
        let deployment_manager = Arc::new(deployment::DeploymentManager::new().await?);

        let mut providers: HashMap<CICDProvider, Box<dyn CICDProviderTrait>> = HashMap::new();
        providers.insert(CICDProvider::GitHubActions, Box::new(github_actions::GitHubActionsProvider::new().await?));
        providers.insert(CICDProvider::GitLabCI, Box::new(gitlab_ci::GitLabCIProvider::new().await?));
        providers.insert(CICDProvider::Jenkins, Box::new(jenkins::JenkinsProvider::new().await?));
        providers.insert(CICDProvider::AzureDevOps, Box::new(azure_devops::AzureDevOpsProvider::new().await?));
        providers.insert(CICDProvider::CircleCI, Box::new(circleci::CircleCIProvider::new().await?));
        providers.insert(CICDProvider::TravisCI, Box::new(travis_ci::TravisCIProvider::new().await?));

        Ok(Self {
            config,
            pipeline_manager,
            webhook_handler,
            deployment_manager,
            providers,
            active_runs: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn create_pipeline(&self, pipeline: Pipeline) -> Result<String, WarpError> {
        // Validate pipeline configuration
        self.validate_pipeline(&pipeline).await?;

        // Create pipeline with provider
        if let Some(provider) = self.providers.get(&pipeline.provider) {
            let pipeline_id = provider.create_pipeline(&pipeline).await?;
            
            // Store pipeline configuration
            self.pipeline_manager.store_pipeline(pipeline).await?;
            
            // Setup webhook
            self.webhook_handler.setup_webhook(&pipeline_id, &pipeline.repository.webhook_url).await?;
            
            Ok(pipeline_id)
        } else {
            Err(WarpError::ConfigError(format!("Unsupported CI/CD provider: {:?}", pipeline.provider)))
        }
    }

    pub async fn trigger_pipeline(&self, pipeline_id: &str, parameters: HashMap<String, String>) -> Result<String, WarpError> {
        let pipeline = self.pipeline_manager.get_pipeline(pipeline_id).await?;
        
        if let Some(provider) = self.providers.get(&pipeline.provider) {
            let run_id = provider.trigger_pipeline(pipeline_id, parameters).await?;
            
            // Create pipeline run record
            let pipeline_run = PipelineRun {
                id: run_id.clone(),
                pipeline_id: pipeline_id.to_string(),
                run_number: self.get_next_run_number(pipeline_id).await?,
                commit_sha: "".to_string(), // Will be updated by webhook
                branch: pipeline.repository.branch.clone(),
                triggered_by: "manual".to_string(), // Will be updated based on trigger
                trigger_type: PipelineTrigger::Manual,
                started_at: chrono::Utc::now(),
                finished_at: None,
                status: PipelineStatus::Pending,
                stages: Vec::new(),
                artifacts: Vec::new(),
                logs: Vec::new(),
            };

            let mut active_runs = self.active_runs.lock().await;
            active_runs.insert(run_id.clone(), pipeline_run);

            Ok(run_id)
        } else {
            Err(WarpError::ConfigError(format!("Unsupported CI/CD provider: {:?}", pipeline.provider)))
        }
    }

    pub async fn get_pipeline_status(&self, run_id: &str) -> Result<PipelineRun, WarpError> {
        let active_runs = self.active_runs.lock().await;
        if let Some(run) = active_runs.get(run_id) {
            Ok(run.clone())
        } else {
            // Try to fetch from storage
            self.pipeline_manager.get_pipeline_run(run_id).await
        }
    }

    pub async fn cancel_pipeline(&self, run_id: &str) -> Result<(), WarpError> {
        let active_runs = self.active_runs.lock().await;
        if let Some(run) = active_runs.get(run_id) {
            let pipeline = self.pipeline_manager.get_pipeline(&run.pipeline_id).await?;
            
            if let Some(provider) = self.providers.get(&pipeline.provider) {
                provider.cancel_pipeline(run_id).await?;
            }
        }
        Ok(())
    }

    pub async fn handle_webhook(&self, payload: serde_json::Value, headers: HashMap<String, String>) -> Result<(), WarpError> {
        self.webhook_handler.handle_webhook(payload, headers).await
    }

    pub async fn deploy_to_environment(&self, pipeline_id: &str, environment: &str, version: &str) -> Result<String, WarpError> {
        self.deployment_manager.deploy(pipeline_id, environment, version).await
    }

    pub async fn get_deployment_status(&self, deployment_id: &str) -> Result<deployment::DeploymentStatus, WarpError> {
        self.deployment_manager.get_status(deployment_id).await
    }

    pub async fn rollback_deployment(&self, deployment_id: &str) -> Result<(), WarpError> {
        self.deployment_manager.rollback(deployment_id).await
    }

    async fn validate_pipeline(&self, pipeline: &Pipeline) -> Result<(), WarpError> {
        // Validate pipeline configuration
        if pipeline.name.is_empty() {
            return Err(WarpError::ConfigError("Pipeline name cannot be empty".to_string()));
        }

        if pipeline.stages.is_empty() {
            return Err(WarpError::ConfigError("Pipeline must have at least one stage".to_string()));
        }

        // Validate stage dependencies
        for stage in &pipeline.stages {
            for dependency in &stage.dependencies {
                if !pipeline.stages.iter().any(|s| s.name == *dependency) {
                    return Err(WarpError::ConfigError(format!("Stage dependency '{}' not found", dependency)));
                }
            }
        }

        Ok(())
    }

    async fn get_next_run_number(&self, pipeline_id: &str) -> Result<u64, WarpError> {
        self.pipeline_manager.get_next_run_number(pipeline_id).await
    }
}

impl Default for CICDConfig {
    fn default() -> Self {
        Self {
            enabled_providers: vec![CICDProvider::GitHubActions],
            webhook_secret: uuid::Uuid::new_v4().to_string(),
            auto_deploy_enabled: false,
            test_required: true,
            security_scan_required: true,
            approval_required: true,
            notification_channels: Vec::new(),
            deployment_environments: vec![
                DeploymentEnvironment {
                    name: "development".to_string(),
                    environment_type: EnvironmentType::Development,
                    url: None,
                    variables: HashMap::new(),
                    secrets: HashMap::new(),
                    approval_required: false,
                    auto_promote: true,
                    health_checks: Vec::new(),
                },
                DeploymentEnvironment {
                    name: "staging".to_string(),
                    environment_type: EnvironmentType::Staging,
                    url: None,
                    variables: HashMap::new(),
                    secrets: HashMap::new(),
                    approval_required: false,
                    auto_promote: false,
                    health_checks: Vec::new(),
                },
                DeploymentEnvironment {
                    name: "production".to_string(),
                    environment_type: EnvironmentType::Production,
                    url: None,
                    variables: HashMap::new(),
                    secrets: HashMap::new(),
                    approval_required: true,
                    auto_promote: false,
                    health_checks: Vec::new(),
                },
            ],
        }
    }
}
