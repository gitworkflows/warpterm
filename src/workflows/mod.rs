use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use crate::error::WarpError;

pub mod manager;
pub mod executor;
pub mod builtin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub version: String,
    pub trigger: WorkflowTrigger,
    pub steps: Vec<WorkflowStep>,
    pub variables: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTrigger {
    Command { pattern: String },
    KeyBinding { key: String, modifiers: Vec<String> },
    FileChange { pattern: String },
    Schedule { cron: String },
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub action: WorkflowAction,
    pub condition: Option<String>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowAction {
    RunCommand { command: String, args: Vec<String> },
    SendKeys { keys: String },
    ShowNotification { message: String },
    SetVariable { name: String, value: String },
    CallScript { script: String, language: String },
    HttpRequest { url: String, method: String, body: Option<String> },
    FileOperation { operation: String, path: String },
}

pub struct WorkflowManager {
    workflows: HashMap<String, Workflow>,
    workflow_directories: Vec<PathBuf>,
}

impl WorkflowManager {
    pub async fn new() -> Result<Self, WarpError> {
        let mut manager = Self {
            workflows: HashMap::new(),
            workflow_directories: vec![
                dirs::config_dir().unwrap_or_default().join("warp/workflows"),
                PathBuf::from("workflows"),
            ],
        };

        manager.load_builtin_workflows().await?;
        manager.discover_workflows().await?;
        
        Ok(manager)
    }

    async fn load_builtin_workflows(&mut self) -> Result<(), WarpError> {
        for workflow in builtin::get_builtin_workflows() {
            self.workflows.insert(workflow.name.clone(), workflow);
        }
        Ok(())
    }

    async fn discover_workflows(&mut self) -> Result<(), WarpError> {
        for workflow_dir in &self.workflow_directories {
            if workflow_dir.exists() {
                self.load_workflows_from_directory(workflow_dir).await?;
            }
        }
        Ok(())
    }

    async fn load_workflows_from_directory(&mut self, dir: &PathBuf) -> Result<(), WarpError> {
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
               path.extension().and_then(|s| s.to_str()) == Some("yml") {
                if let Ok(workflow) = self.load_workflow_file(&path).await {
                    self.workflows.insert(workflow.name.clone(), workflow);
                }
            }
        }
        
        Ok(())
    }

    async fn load_workflow_file(&self, path: &PathBuf) -> Result<Workflow, WarpError> {
        let content = fs::read_to_string(path).await?;
        let workflow: Workflow = serde_yaml::from_str(&content)
            .map_err(|e| WarpError::ConfigError(format!("Failed to parse workflow: {}", e)))?;
        Ok(workflow)
    }

    pub fn get_workflow(&self, name: &str) -> Option<&Workflow> {
        self.workflows.get(name)
    }

    pub fn list_workflows(&self) -> Vec<&String> {
        self.workflows.keys().collect()
    }

    pub fn find_workflows_by_trigger(&self, trigger: &WorkflowTrigger) -> Vec<&Workflow> {
        self.workflows.values()
            .filter(|w| std::mem::discriminant(&w.trigger) == std::mem::discriminant(trigger))
            .collect()
    }
}
