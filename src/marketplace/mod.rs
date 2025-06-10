use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

pub mod client;
pub mod package_manager;
pub mod store;
pub mod auth;
pub mod discovery;
pub mod installer;
pub mod publisher;
pub mod security;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ItemCategory,
    pub item_type: ItemType,
    pub version: String,
    pub author: Author,
    pub tags: Vec<String>,
    pub rating: Rating,
    pub downloads: u64,
    pub price: Price,
    pub license: License,
    pub compatibility: Compatibility,
    pub screenshots: Vec<String>,
    pub readme: String,
    pub changelog: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub verified: bool,
    pub featured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCategory {
    Themes,
    Plugins,
    AIModels,
    Keysets,
    Workflows,
    Scripts,
    Extensions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Theme(ThemeMetadata),
    Plugin(PluginMetadata),
    AIModel(AIModelMetadata),
    Keyset(KeysetMetadata),
    Workflow(WorkflowMetadata),
    Script(ScriptMetadata),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub color_scheme: String,
    pub supports_dark_mode: bool,
    pub supports_light_mode: bool,
    pub accent_colors: Vec<String>,
    pub preview_images: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub entry_point: String,
    pub permissions: Vec<String>,
    pub dependencies: Vec<String>,
    pub supported_platforms: Vec<String>,
    pub api_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelMetadata {
    pub model_type: String,
    pub provider: String,
    pub capabilities: Vec<String>,
    pub context_length: u32,
    pub parameters: u64,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeysetMetadata {
    pub editor_style: String,
    pub supported_shells: Vec<String>,
    pub key_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    pub triggers: Vec<String>,
    pub actions: Vec<String>,
    pub complexity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMetadata {
    pub language: String,
    pub runtime_requirements: Vec<String>,
    pub script_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub website: Option<String>,
    pub verified: bool,
    pub reputation: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub average: f32,
    pub count: u32,
    pub distribution: HashMap<u8, u32>, // 1-5 stars
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Price {
    Free,
    Paid { amount: u32, currency: String },
    PayWhatYouWant { suggested: u32, currency: String },
    Subscription { monthly: u32, yearly: u32, currency: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: Option<String>,
    pub open_source: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compatibility {
    pub min_warp_version: String,
    pub max_warp_version: Option<String>,
    pub platforms: Vec<String>,
    pub architectures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub rating: u8,
    pub title: String,
    pub content: String,
    pub helpful_votes: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub verified_purchase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: Option<String>,
    pub category: Option<ItemCategory>,
    pub tags: Vec<String>,
    pub price_filter: Option<PriceFilter>,
    pub rating_filter: Option<f32>,
    pub sort_by: SortBy,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceFilter {
    Free,
    Paid,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    Relevance,
    Downloads,
    Rating,
    Recent,
    Name,
    Price,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub items: Vec<MarketplaceItem>,
    pub total_count: u32,
    pub page: u32,
    pub per_page: u32,
    pub facets: SearchFacets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFacets {
    pub categories: HashMap<ItemCategory, u32>,
    pub tags: HashMap<String, u32>,
    pub authors: HashMap<String, u32>,
    pub price_ranges: HashMap<String, u32>,
}

pub struct Marketplace {
    client: Arc<client::MarketplaceClient>,
    package_manager: Arc<Mutex<package_manager::PackageManager>>,
    store: Arc<Mutex<store::LocalStore>>,
    auth: Arc<Mutex<auth::AuthManager>>,
    discovery: Arc<discovery::DiscoveryEngine>,
    installer: Arc<installer::Installer>,
    publisher: Arc<publisher::Publisher>,
    security: Arc<security::SecurityManager>,
}

impl Marketplace {
    pub async fn new() -> Result<Self, WarpError> {
        let client = Arc::new(client::MarketplaceClient::new().await?);
        let package_manager = Arc::new(Mutex::new(package_manager::PackageManager::new().await?));
        let store = Arc::new(Mutex::new(store::LocalStore::new().await?));
        let auth = Arc::new(Mutex::new(auth::AuthManager::new().await?));
        let discovery = Arc::new(discovery::DiscoveryEngine::new().await?);
        let installer = Arc::new(installer::Installer::new().await?);
        let publisher = Arc::new(publisher::Publisher::new().await?);
        let security = Arc::new(security::SecurityManager::new().await?);

        Ok(Self {
            client,
            package_manager,
            store,
            auth,
            discovery,
            installer,
            publisher,
            security,
        })
    }

    pub async fn search(&self, query: SearchQuery) -> Result<SearchResult, WarpError> {
        self.client.search(query).await
    }

    pub async fn get_item(&self, id: &str) -> Result<MarketplaceItem, WarpError> {
        self.client.get_item(id).await
    }

    pub async fn get_reviews(&self, item_id: &str, page: u32) -> Result<Vec<Review>, WarpError> {
        self.client.get_reviews(item_id, page).await
    }

    pub async fn install_item(&self, item_id: &str) -> Result<(), WarpError> {
        // Security check
        self.security.verify_item(item_id).await?;
        
        // Download and install
        self.installer.install(item_id).await?;
        
        // Update local store
        let mut store = self.store.lock().await;
        store.mark_installed(item_id).await?;
        
        Ok(())
    }

    pub async fn uninstall_item(&self, item_id: &str) -> Result<(), WarpError> {
        self.installer.uninstall(item_id).await?;
        
        let mut store = self.store.lock().await;
        store.mark_uninstalled(item_id).await?;
        
        Ok(())
    }

    pub async fn update_item(&self, item_id: &str) -> Result<(), WarpError> {
        let mut package_manager = self.package_manager.lock().await;
        package_manager.update_package(item_id).await
    }

    pub async fn get_installed_items(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        let store = self.store.lock().await;
        store.get_installed_items().await
    }

    pub async fn get_updates(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        let mut package_manager = self.package_manager.lock().await;
        package_manager.check_updates().await
    }

    pub async fn get_recommendations(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        self.discovery.get_recommendations().await
    }

    pub async fn rate_item(&self, item_id: &str, rating: u8, review: Option<String>) -> Result<(), WarpError> {
        let auth = self.auth.lock().await;
        if !auth.is_authenticated() {
            return Err(WarpError::ConfigError("Authentication required".to_string()));
        }
        
        self.client.submit_rating(item_id, rating, review).await
    }

    pub async fn publish_item(&self, item: MarketplaceItem, package_data: Vec<u8>) -> Result<String, WarpError> {
        let auth = self.auth.lock().await;
        if !auth.is_authenticated() {
            return Err(WarpError::ConfigError("Authentication required".to_string()));
        }
        
        // Security scan
        self.security.scan_package(&package_data).await?;
        
        // Publish
        self.publisher.publish(item, package_data).await
    }
}
