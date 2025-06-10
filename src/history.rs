use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{config::Config, error::WarpError};

pub struct HistoryManager {
    config: Arc<Mutex<Config>>,
    commands: Vec<String>,
}

impl HistoryManager {
    pub async fn new(config: Arc<Mutex<Config>>) -> Result<Self, WarpError> {
        Ok(Self {
            config,
            commands: Vec::new(),
        })
    }

    pub async fn add_command(&mut self, command: String) -> Result<(), WarpError> {
        self.commands.push(command);
        Ok(())
    }
}
