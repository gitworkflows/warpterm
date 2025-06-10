use std::sync::Arc;
use tokio::sync::Mutex;
use warp_terminal::{
    app::WarpApp,
    config::WarpConfig,
    error::WarpError,
    keysets::KeySetManager,
    scripting::{ScriptLanguage, ScriptingManager},
    themes::ThemeManager,
    workflows::WorkflowManager,
};

#[tokio::main]
async fn main() -> Result<(), WarpError> {
    // Initialize logging
    env_logger::init();

    // Load configuration
    let config = Arc::new(Mutex::new(WarpConfig::load(None).await?));

    // Initialize managers
    let theme_manager = Arc::new(Mutex::new(ThemeManager::new().await?));
    let keyset_manager = Arc::new(Mutex::new(KeySetManager::new().await?));
    let workflow_manager = Arc::new(Mutex::new(WorkflowManager::new().await?));
    let scripting_manager = Arc::new(ScriptingManager::new().await?);

    // Example: Change theme
    {
        let mut themes = theme_manager.lock().await;
        themes.set_current_theme("standard_light".to_string())?;
        println!(
            "Current theme: {:?}",
            themes.get_current_theme().map(|t| &t.name)
        );
    }

    // Example: List available keysets
    {
        let keysets = keyset_manager.lock().await;
        println!("Available keysets: {:?}", keysets.list_keysets());
    }

    // Example: Execute a Lua script
    let lua_script = r#"
        terminal.print("Hello from Lua!")
        terminal.print("Current directory: " .. cwd)
        
        for key, value in pairs(env) do
            if key == "USER" or key == "USERNAME" then
                terminal.print("User: " .. value)
                break
            end
        end
    "#;

    match scripting_manager
        .execute_script(ScriptLanguage::Lua, lua_script, None)
        .await
    {
        Ok(result) => println!("Lua script result: {}", result),
        Err(e) => eprintln!("Lua script error: {}", e),
    }

    // Example: List workflows
    {
        let workflows = workflow_manager.lock().await;
        println!("Available workflows: {:?}", workflows.list_workflows());
    }

    // Create and run the main application
    let app = WarpApp::new(config).await?;
    app.run().await?;

    Ok(())
}
