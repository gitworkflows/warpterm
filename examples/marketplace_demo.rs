use warp_terminal::{
    marketplace::{Marketplace, SearchQuery, ItemCategory, SortBy, PriceFilter},
    error::WarpError,
};

#[tokio::main]
async fn main() -> Result<(), WarpError> {
    // Initialize the marketplace
    let marketplace = Marketplace::new().await?;
    
    println!("🛒 Warp Marketplace Demo\n");
    
    // Demo 1: Search for themes
    println!("1. Searching for themes...");
    let search_query = SearchQuery {
        query: Some("dark".to_string()),
        category: Some(ItemCategory::Themes),
        tags: vec![],
        price_filter: Some(PriceFilter::Any),
        rating_filter: Some(4.0),
        sort_by: SortBy::Rating,
        page: 1,
        per_page: 5,
    };
    
    match marketplace.search(search_query).await {
        Ok(results) => {
            println!("   Found {} themes:", results.items.len());
            for item in results.items.iter().take(3) {
                println!("   • {} v{} - {} (⭐ {:.1})", 
                    item.name, item.version, item.description, item.rating.average);
            }
        }
        Err(e) => println!("   Search failed: {}", e),
    }
    
    // Demo 2: Get recommendations
    println!("\n2. Getting personalized recommendations...");
    match marketplace.get_recommendations().await {
        Ok(recommendations) => {
            println!("   Recommended items:");
            for item in recommendations.iter().take(3) {
                println!("   💡 {} - {}",
                    item.name, item.description);
            }
        }
        Err(e) => println!("   Recommendations failed: {}", e),
    }
    
    // Demo 3: Get item details
    println!("\n3. Getting item details...");
    match marketplace.get_item("catppuccin-theme").await {
        Ok(item) => {
            println!("   Item: {} v{}", item.name, item.version);
            println!("   Author: {} {}", 
                item.author.display_name,
                if item.author.verified { "✓" } else { "" }
            );
            println!("   Rating: ⭐ {:.1} ({} reviews)", 
                item.rating.average, item.rating.count);
            println!("   Downloads: {}", item.downloads);
            println!("   Price: {:?}", item.price);
        }
        Err(e) => println!("   Get item failed: {}", e),
    }
    
    // Demo 4: Check installed items
    println!("\n4. Checking installed items...");
    match marketplace.get_installed_items().await {
        Ok(installed) => {
            if installed.is_empty() {
                println!("   No items installed yet");
            } else {
                println!("   Installed items:");
                for item in installed {
                    println!("   ✅ {} v{}", item.name, item.version);
                }
            }
        }
        Err(e) => println!("   Failed to get installed items: {}", e),
    }
    
    // Demo 5: Simulate installation
    println!("\n5. Simulating installation...");
    println!("   Installing 'catppuccin-theme'...");
    match marketplace.install_item("catppuccin-theme").await {
        Ok(_) => println!("   ✅ Installation completed successfully!"),
        Err(e) => println!("   ❌ Installation failed: {}", e),
    }
    
    // Demo 6: Check for updates
    println!("\n6. Checking for updates...");
    match marketplace.get_updates().await {
        Ok(updates) => {
            if updates.is_empty() {
                println!("   All items are up to date");
            } else {
                println!("   Available updates:");
                for item in updates {
                    println!("   🔄 {} v{}", item.name, item.version);
                }
            }
        }
        Err(e) => println!("   Update check failed: {}", e),
    }
    
    println!("\n🎉 Marketplace Demo Complete!");
    println!("\nFeatures demonstrated:");
    println!("• 🔍 Search and discovery");
    println!("• 💡 Personalized recommendations");
    println!("• 📦 Package installation and management");
    println!("• ⭐ Ratings and reviews");
    println!("• 🔄 Update management");
    println!("• 🛡️ Security verification");
    println!("• 💰 Multiple pricing models");
    println!("• 🏷️ Categorization and tagging");
    
    Ok(())
}
