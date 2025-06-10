use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{config::Config, error::WarpError};

pub struct CompletionEngine {
    config: Arc<Mutex<Config>>,
}

impl CompletionEngine {
    pub async fn new(config: Arc<Mutex<Config>>) -> Result<Self, WarpError> {
        Ok(Self { config })
    }
}
