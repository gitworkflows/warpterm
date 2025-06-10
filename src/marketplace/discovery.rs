use super::*;
use crate::error::WarpError;
use std::collections::HashMap;

pub struct DiscoveryEngine {
    user_preferences: UserPreferences,
    usage_analytics: UsageAnalytics,
    recommendation_cache: HashMap<String, Vec<MarketplaceItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_categories: Vec<ItemCategory>,
    pub preferred_tags: Vec<String>,
    pub price_preference: PriceFilter,
    pub rating_threshold: f32,
    pub language_preferences: Vec<String>,
    pub theme_preferences: ThemePreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePreferences {
    pub color_scheme: Option<String>,
    pub prefers_dark_mode: bool,
    pub accent_color_preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAnalytics {
    pub most_used_commands: HashMap<String, u32>,
    pub active_hours: Vec<u8>,
    pub project_types: HashMap<String, u32>,
    pub workflow_patterns: Vec<String>,
}

impl DiscoveryEngine {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            user_preferences: UserPreferences::default(),
            usage_analytics: UsageAnalytics::default(),
            recommendation_cache: HashMap::new(),
        })
    }

    pub async fn get_recommendations(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        let cache_key = "general_recommendations".to_string();
        
        if let Some(cached) = self.recommendation_cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        let mut recommendations = Vec::new();
        
        // Get personalized recommendations based on user preferences
        recommendations.extend(self.get_category_recommendations().await?);
        recommendations.extend(self.get_usage_based_recommendations().await?);
        recommendations.extend(self.get_trending_recommendations().await?);
        
        // Sort by relevance score
        recommendations.sort_by(|a, b| {
            let score_a = self.calculate_relevance_score(a);
            let score_b = self.calculate_relevance_score(b);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Limit to top 20 recommendations
        recommendations.truncate(20);
        
        Ok(recommendations)
    }

    async fn get_category_recommendations(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        let mut recommendations = Vec::new();
        
        for category in &self.user_preferences.preferred_categories {
            let query = SearchQuery {
                query: None,
                category: Some(category.clone()),
                tags: vec![],
                price_filter: Some(self.user_preferences.price_preference.clone()),
                rating_filter: Some(self.user_preferences.rating_threshold),
                sort_by: SortBy::Rating,
                page: 1,
                per_page: 5,
            };
            
            // This would call the marketplace API
            // For now, create mock recommendations
            recommendations.extend(self.create_mock_recommendations(category).await?);
        }
        
        Ok(recommendations)
    }

    async fn get_usage_based_recommendations(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        let mut recommendations = Vec::new();
        
        // Analyze most used commands and suggest relevant plugins
        for (command, _usage_count) in &self.usage_analytics.most_used_commands {
            if command.starts_with("git") {
                recommendations.extend(self.get_git_related_items().await?);
            } else if command.starts_with("docker") {
                recommendations.extend(self.get_docker_related_items().await?);
            } else if command.starts_with("npm") || command.starts_with("yarn") {
                recommendations.extend(self.get_nodejs_related_items().await?);
            }
        }
        
        Ok(recommendations)
    }

    async fn get_trending_recommendations(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        // This would fetch trending items from the marketplace
        // For now, return mock trending items
        Ok(vec![])
    }

    async fn create_mock_recommendations(&self, category: &ItemCategory) -> Result<Vec<MarketplaceItem>, WarpError> {
        let mut items = Vec::new();
        
        match category {
            ItemCategory::Themes => {
                items.push(MarketplaceItem {
                    id: "catppuccin-theme".to_string(),
                    name: "Catppuccin Theme".to_string(),
                    description: "Soothing pastel theme for Warp".to_string(),
                    category: ItemCategory::Themes,
                    item_type: ItemType::Theme(ThemeMetadata {
                        color_scheme: "pastel".to_string(),
                        supports_dark_mode: true,
                        supports_light_mode: true,
                        accent_colors: vec!["#cba6f7".to_string(), "#89b4fa".to_string()],
                        preview_images: vec!["preview1.png".to_string()],
                    }),
                    version: "1.0.0".to_string(),
                    author: Author {
                        id: "catppuccin".to_string(),
                        username: "catppuccin".to_string(),
                        display_name: "Catppuccin".to_string(),
                        email: None,
                        website: Some("https://catppuccin.com".to_string()),
                        verified: true,
                        reputation: 95,
                    },
                    tags: vec!["pastel".to_string(), "dark".to_string(), "light".to_string()],
                    rating: Rating {
                        average: 4.8,
                        count: 1250,
                        distribution: HashMap::from([(5, 1000), (4, 200), (3, 30), (2, 15), (1, 5)]),
                    },
                    downloads: 15000,
                    price: Price::Free,
                    license: License {
                        name: "MIT".to_string(),
                        url: Some("https://opensource.org/licenses/MIT".to_string()),
                        open_source: true,
                    },
                    compatibility: Compatibility {
                        min_warp_version: "1.0.0".to_string(),
                        max_warp_version: None,
                        platforms: vec!["macos".to_string(), "linux".to_string(), "windows".to_string()],
                        architectures: vec!["x86_64".to_string(), "arm64".to_string()],
                    },
                    screenshots: vec!["screenshot1.png".to_string()],
                    readme: "# Catppuccin Theme\n\nA soothing pastel theme for Warp terminal.".to_string(),
                    changelog: "## v1.0.0\n- Initial release".to_string(),
                    created_at: chrono::Utc::now() - chrono::Duration::days(30),
                    updated_at: chrono::Utc::now() - chrono::Duration::days(5),
                    verified: true,
                    featured: true,
                });
            }
            ItemCategory::Plugins => {
                items.push(MarketplaceItem {
                    id: "git-enhanced".to_string(),
                    name: "Git Enhanced".to_string(),
                    description: "Enhanced Git integration with visual diff and branch management".to_string(),
                    category: ItemCategory::Plugins,
                    item_type: ItemType::Plugin(PluginMetadata {
                        entry_point: "git_enhanced.wasm".to_string(),
                        permissions: vec!["filesystem".to_string(), "process".to_string()],
                        dependencies: vec!["git".to_string()],
                        supported_platforms: vec!["macos".to_string(), "linux".to_string()],
                        api_version: "1.0".to_string(),
                    }),
                    version: "2.1.0".to_string(),
                    author: Author {
                        id: "gittools".to_string(),
                        username: "gittools".to_string(),
                        display_name: "Git Tools Team".to_string(),
                        email: Some("team@gittools.dev".to_string()),
                        website: Some("https://gittools.dev".to_string()),
                        verified: true,
                        reputation: 88,
                    },
                    tags: vec!["git".to_string(), "vcs".to_string(), "productivity".to_string()],
                    rating: Rating {
                        average: 4.6,
                        count: 890,
                        distribution: HashMap::from([(5, 650), (4, 180), (3, 40), (2, 15), (1, 5)]),
                    },
                    downloads: 8500,
                    price: Price::Paid { amount: 999, currency: "USD".to_string() },
                    license: License {
                        name: "Commercial".to_string(),
                        url: None,
                        open_source: false,
                    },
                    compatibility: Compatibility {
                        min_warp_version: "1.0.0".to_string(),
                        max_warp_version: None,
                        platforms: vec!["macos".to_string(), "linux".to_string()],
                        architectures: vec!["x86_64".to_string(), "arm64".to_string()],
                    },
                    screenshots: vec!["plugin_screenshot1.png".to_string()],
                    readme: "# Git Enhanced Plugin\n\nPowerful Git integration for Warp.".to_string(),
                    changelog: "## v2.1.0\n- Added visual diff support\n- Improved branch management".to_string(),
                    created_at: chrono::Utc::now() - chrono::Duration::days(60),
                    updated_at: chrono::Utc::now() - chrono::Duration::days(10),
                    verified: true,
                    featured: false,
                });
            }
            _ => {}
        }
        
        Ok(items)
    }

    async fn get_git_related_items(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        // Return Git-related plugins and themes
        Ok(vec![])
    }

    async fn get_docker_related_items(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        // Return Docker-related plugins and tools
        Ok(vec![])
    }

    async fn get_nodejs_related_items(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        // Return Node.js-related plugins and tools
        Ok(vec![])
    }

    fn calculate_relevance_score(&self, item: &MarketplaceItem) -> f32 {
        let mut score = 0.0;
        
        // Base score from rating and downloads
        score += item.rating.average * 0.3;
        score += (item.downloads as f32).log10() * 0.2;
        
        // Category preference bonus
        if self.user_preferences.preferred_categories.contains(&item.category) {
            score += 2.0;
        }
        
        // Tag preference bonus
        for tag in &item.tags {
            if self.user_preferences.preferred_tags.contains(tag) {
                score += 0.5;
            }
        }
        
        // Price preference
        match (&item.price, &self.user_preferences.price_preference) {
            (Price::Free, PriceFilter::Free) => score += 1.0,
            (Price::Paid { .. }, PriceFilter::Paid) => score += 0.5,
            _ => {}
        }
        
        // Verified bonus
        if item.verified {
            score += 0.5;
        }
        
        // Featured bonus
        if item.featured {
            score += 1.0;
        }
        
        score
    }

    pub async fn update_user_preferences(&mut self, preferences: UserPreferences) -> Result<(), WarpError> {
        self.user_preferences = preferences;
        // Clear cache to force regeneration with new preferences
        self.recommendation_cache.clear();
        Ok(())
    }

    pub async fn track_usage(&mut self, command: &str) -> Result<(), WarpError> {
        *self.usage_analytics.most_used_commands.entry(command.to_string()).or_insert(0) += 1;
        Ok(())
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            preferred_categories: vec![ItemCategory::Themes, ItemCategory::Plugins],
            preferred_tags: vec![],
            price_preference: PriceFilter::Any,
            rating_threshold: 4.0,
            language_preferences: vec!["en".to_string()],
            theme_preferences: ThemePreferences {
                color_scheme: None,
                prefers_dark_mode: true,
                accent_color_preferences: vec![],
            },
        }
    }
}

impl Default for UsageAnalytics {
    fn default() -> Self {
        Self {
            most_used_commands: HashMap::new(),
            active_hours: vec![],
            project_types: HashMap::new(),
            workflow_patterns: vec![],
        }
    }
}
