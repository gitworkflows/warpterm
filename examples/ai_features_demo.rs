use warp_terminal::{
    ai::{AdvancedAI, completion::{CompletionContext, CompletionType}},
    error::WarpError,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), WarpError> {
    // Initialize the advanced AI system
    let ai = AdvancedAI::new().await?;
    
    // Create a sample context
    let context = CompletionContext {
        current_line: "git ".to_string(),
        cursor_position: 4,
        working_directory: "/home/user/project".to_string(),
        shell_type: "zsh".to_string(),
        command_history: vec![
            "git status".to_string(),
            "git add .".to_string(),
            "npm install".to_string(),
        ],
        environment_variables: std::env::vars().collect(),
        git_status: Some(crate::ai::completion::GitStatus {
            branch: "main".to_string(),
            modified_files: vec!["src/main.rs".to_string()],
            untracked_files: vec!["new_file.txt".to_string()],
            staged_files: vec![],
        }),
        docker_context: None,
    };
    
    println!("🤖 Advanced AI Features Demo\n");
    
    // Demo 1: Code Completion
    println!("1. Code Completion for 'git ':");
    match ai.get_completions(context.clone()).await {
        Ok(completions) => {
            for completion in completions.iter().take(5) {
                println!("   {} - {}", 
                    completion.text, 
                    completion.description.as_deref().unwrap_or("No description")
                );
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // Demo 2: Smart Suggestions
    println!("\n2. Smart Suggestions:");
    match ai.get_smart_suggestions(context.clone()).await {
        Ok(suggestions) => {
            for suggestion in suggestions.iter().take(3) {
                println!("   💡 {} - {}", suggestion.title, suggestion.description);
                if let Some(command) = &suggestion.command {
                    println!("      Command: {}", command);
                }
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // Demo 3: Context-Aware Assistance
    println!("\n3. Context-Aware Analysis:");
    let dangerous_context = CompletionContext {
        current_line: "rm -rf ".to_string(),
        cursor_position: 7,
        ..context.clone()
    };
    
    match ai.get_smart_suggestions(dangerous_context).await {
        Ok(suggestions) => {
            for suggestion in suggestions {
                println!("   ⚠️  {} - {}", suggestion.title, suggestion.description);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    // Demo 4: Learning from Interaction
    println!("\n4. Learning System:");
    println!("   Simulating user accepting a suggestion...");
    ai.learn_from_interaction("suggestion-123", true, Some("Very helpful!".to_string())).await?;
    println!("   ✅ Feedback recorded and learning system updated");
    
    println!("\n🎉 AI Features Demo Complete!");
    
    Ok(())
}
