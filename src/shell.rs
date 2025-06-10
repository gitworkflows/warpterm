use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{config::Config, error::WarpError};

pub struct ShellManager {
    config: Arc<Mutex<Config>>,
    current_shell: String,
}

impl ShellManager {
    pub async fn new(config: Arc<Mutex<Config>>) -> Result<Self, WarpError> {
        let shell = {
            let cfg = config.lock().await;
            cfg.terminal.shell.clone()
        };

        Ok(Self {
            config,
            current_shell: shell,
        })
    }
}
