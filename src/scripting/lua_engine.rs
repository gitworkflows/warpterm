use super::{ScriptEngine, ScriptLanguage, ScriptContext};
use crate::error::WarpError;
use mlua::{Lua, LuaSerdeExt};
use std::collections::HashMap;

pub struct LuaEngine {
    lua: Lua,
}

impl LuaEngine {
    pub async fn new() -> Result<Self, WarpError> {
        let lua = Lua::new();
        
        // Set up Warp-specific functions
        let globals = lua.globals();
        
        // Terminal functions
        let terminal_table = lua.create_table()?;
        terminal_table.set("print", lua.create_function(|_, text: String| {
            println!("{}", text);
            Ok(())
        })?)?;
        
        terminal_table.set("execute", lua.create_function(|_, command: String| {
            // This would integrate with the command executor
            Ok(format!("Executed: {}", command))
        })?)?;
        
        globals.set("terminal", terminal_table)?;
        
        // Utility functions
        let utils_table = lua.create_table()?;
        utils_table.set("sleep", lua.create_async_function(|_, seconds: f64| async move {
            tokio::time::sleep(tokio::time::Duration::from_secs_f64(seconds)).await;
            Ok(())
        })?)?;
        
        globals.set("utils", utils_table)?;

        Ok(Self { lua })
    }
}

impl ScriptEngine for LuaEngine {
    async fn execute(&self, script: &str, context: &ScriptContext) -> Result<String, WarpError> {
        // Set context variables
        let globals = self.lua.globals();
        
        // Set variables
        let vars_table = self.lua.create_table()?;
        for (key, value) in &context.variables {
            vars_table.set(key.as_str(), value.as_str())?;
        }
        globals.set("vars", vars_table)?;
        
        // Set environment
        let env_table = self.lua.create_table()?;
        for (key, value) in &context.environment {
            env_table.set(key.as_str(), value.as_str())?;
        }
        globals.set("env", env_table)?;
        
        // Set current directory
        globals.set("cwd", context.current_directory.as_str())?;
        
        // Execute script
        match self.lua.load(script).exec() {
            Ok(_) => Ok("Script executed successfully".to_string()),
            Err(e) => Err(WarpError::ConfigError(format!("Lua script error: {}", e))),
        }
    }

    async fn evaluate(&self, expression: &str, context: &ScriptContext) -> Result<String, WarpError> {
        // Set context (similar to execute)
        let globals = self.lua.globals();
        
        let vars_table = self.lua.create_table()?;
        for (key, value) in &context.variables {
            vars_table.set(key.as_str(), value.as_str())?;
        }
        globals.set("vars", vars_table)?;
        
        // Evaluate expression
        match self.lua.load(expression).eval::<mlua::Value>() {
            Ok(value) => {
                match value {
                    mlua::Value::String(s) => Ok(s.to_str()?.to_string()),
                    mlua::Value::Number(n) => Ok(n.to_string()),
                    mlua::Value::Boolean(b) => Ok(b.to_string()),
                    mlua::Value::Nil => Ok("nil".to_string()),
                    _ => Ok(format!("{:?}", value)),
                }
            }
            Err(e) => Err(WarpError::ConfigError(format!("Lua evaluation error: {}", e))),
        }
    }

    fn language(&self) -> ScriptLanguage {
        ScriptLanguage::Lua
    }
}
