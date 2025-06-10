use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::error::WarpError;

pub mod lua_engine;
pub mod javascript_engine;
pub mod python_engine;
pub mod shell_engine;

#[derive(Debug, Clone)]
pub enum ScriptLanguage {
    Lua,
    JavaScript,
    Python,
    Shell,
}

#[derive(Debug, Clone)]
pub struct ScriptContext {
    pub variables: HashMap<String, String>,
    pub terminal_state: Option<String>,
    pub current_directory: String,
    pub environment: HashMap<String, String>,
}

pub trait ScriptEngine: Send + Sync {
    async fn execute(&self, script: &str, context: &ScriptContext) -> Result<String, WarpError>;
    async fn evaluate(&self, expression: &str, context: &ScriptContext) -> Result<String, WarpError>;
    fn language(&self) -> ScriptLanguage;
}

pub struct ScriptingManager {
    engines: HashMap<ScriptLanguage, Box<dyn ScriptEngine>>,
    global_context: Arc<Mutex<ScriptContext>>,
}

impl ScriptingManager {
    pub async fn new() -> Result<Self, WarpError> {
        let mut engines: HashMap<ScriptLanguage, Box<dyn ScriptEngine>> = HashMap::new();
        
        engines.insert(ScriptLanguage::Lua, Box::new(lua_engine::LuaEngine::new().await?));
        engines.insert(ScriptLanguage::JavaScript, Box::new(javascript_engine::JavaScriptEngine::new().await?));
        engines.insert(ScriptLanguage::Python, Box::new(python_engine::PythonEngine::new().await?));
        engines.insert(ScriptLanguage::Shell, Box::new(shell_engine::ShellEngine::new().await?));

        let global_context = Arc::new(Mutex::new(ScriptContext {
            variables: HashMap::new(),
            terminal_state: None,
            current_directory: std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            environment: std::env::vars().collect(),
        }));

        Ok(Self {
            engines,
            global_context,
        })
    }

    pub async fn execute_script(
        &self,
        language: ScriptLanguage,
        script: &str,
        context: Option<ScriptContext>,
    ) -> Result<String, WarpError> {
        let engine = self.engines.get(&language)
            .ok_or_else(|| WarpError::ConfigError(format!("Script engine for {:?} not found", language)))?;

        let ctx = if let Some(ctx) = context {
            ctx
        } else {
            self.global_context.lock().await.clone()
        };

        engine.execute(script, &ctx).await
    }

    pub async fn evaluate_expression(
        &self,
        language: ScriptLanguage,
        expression: &str,
        context: Option<ScriptContext>,
    ) -> Result<String, WarpError> {
        let engine = self.engines.get(&language)
            .ok_or_else(|| WarpError::ConfigError(format!("Script engine for {:?} not found", language)))?;

        let ctx = if let Some(ctx) = context {
            ctx
        } else {
            self.global_context.lock().await.clone()
        };

        engine.evaluate(expression, &ctx).await
    }

    pub async fn set_global_variable(&self, name: String, value: String) {
        let mut context = self.global_context.lock().await;
        context.variables.insert(name, value);
    }

    pub async fn get_global_variable(&self, name: &str) -> Option<String> {
        let context = self.global_context.lock().await;
        context.variables.get(name).cloned()
    }

    pub async fn update_terminal_state(&self, state: String) {
        let mut context = self.global_context.lock().await;
        context.terminal_state = Some(state);
    }
}
