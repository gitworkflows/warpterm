use std::sync::Arc;
use tokio::sync::Mutex;
use crate::error::WarpError;

pub mod completion;
pub mod context_assistant;
pub mod providers;

use completion::{CompletionEngine, CompletionContext, CompletionItem};
use context_assistant::{ContextualAssistant, ContextualSuggestion};

pub struct AdvancedAI {
    completion_engine: Arc<CompletionEngine>,
    contextual_assistant: Arc<Mutex<ContextualAssistant>>,
    is_enabled: bool,
}

impl AdvancedAI {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            completion_engine: Arc::new(CompletionEngine::new().await?),
            contextual_assistant: Arc::new(Mutex::new(ContextualAssistant::new().await?)),
            is_enabled: true,
        })
    }

    pub async fn get_completions(
        &self,
        context: CompletionContext,
    ) -> Result<Vec<CompletionItem>, WarpError> {
        if !self.is_enabled {
            return Ok(vec![]);
        }

        self.completion_engine.get_completions(&context).await
    }

    pub async fn get_smart_suggestions(
        &self,
        context: CompletionContext,
    ) -> Result<Vec<ContextualSuggestion>, WarpError> {
        if !self.is_enabled {
            return Ok(vec![]);
        }

        let mut assistant = self.contextual_assistant.lock().await;
        assistant.analyze_context(&context.current_line).await
    }

    pub async fn learn_from_interaction(
        &self,
        suggestion_id: &str,
        accepted: bool,
        feedback: Option<String>,
    ) -> Result<(), WarpError> {
        let mut assistant = self.contextual_assistant.lock().await;
        assistant.learn_from_interaction(suggestion_id, accepted, feedback).await
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}
