use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

use crate::{config::Config, error::WarpError};

pub struct PluginManager {
    config: Arc<Mutex<Config>>,
    loaded_plugins: HashMap<String, Box<dyn Plugin>>,
}

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, command: &str) -> Result<String, WarpError>;
}

impl PluginManager {
    pub async fn new(config: Arc<Mutex<Config>>) -> Result<Self, WarpError> {
        Ok(Self {
            config,
            loaded_plugins: HashMap::new(),
        })
    }

    pub async fn start(&self) -> Result<(), WarpError> {
        // Load and initialize plugins
        Ok(())
    }
}
