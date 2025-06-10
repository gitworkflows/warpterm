use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

pub mod rest_api;
pub mod graphql_api;
pub mod webhook_api;
pub mod auth_middleware;
pub mod rate_limiting;
pub mod api_documentation;
pub mod sdk_generator;
pub mod integration_manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIConfig {
    pub base_url: String,
    pub version: String,
    pub rate_limits: RateLimitConfig,
    pub authentication: AuthConfig,
    pub cors_config: CorsConfig,
    pub documentation_enabled: bool,
    pub metrics_enabled: bool,
    pub logging_enabled: bool,
    pub webhook_config: WebhookConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub requests_per_day: u32,
    pub burst_limit: u32,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry: u64,
    pub refresh_token_expiry: u64,
    pub api_key_enabled: bool,
    pub oauth_enabled: bool,
    pub oauth_providers: Vec<OAuthProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub authorization_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub expose_headers: Vec<String>,
    pub max_age: u32,
    pub credentials: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub enabled: bool,
    pub secret_key: String,
    pub retry_attempts: u32,
    pub timeout: u64,
    pub supported_events: Vec<WebhookEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEvent {
    ItemInstalled,
    ItemUninstalled,
    ItemUpdated,
    ItemRated,
    ItemReviewed,
    UserRegistered,
    UserSubscribed,
    PaymentCompleted,
    PaymentFailed,
    AnalyticsUpdate,
    SystemAlert,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIKey {
    pub key_id: String,
    pub key_value: String,
    pub name: String,
    pub description: String,
    pub user_id: String,
    pub scopes: Vec<APIScope>,
    pub rate_limit: Option<RateLimitConfig>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum APIScope {
    // Marketplace scopes
    MarketplaceRead,
    MarketplaceWrite,
    MarketplaceAdmin,
    
    // Analytics scopes
    AnalyticsRead,
    AnalyticsWrite,
    
    // User scopes
    UserRead,
    UserWrite,
    
    // CI/CD scopes
    CICDRead,
    CICDWrite,
    CICDExecute,
    
    // Collaboration scopes
    CollaborationRead,
    CollaborationWrite,
    CollaborationManage,
    
    // Visualization scopes
    VisualizationRead,
    VisualizationWrite,
    
    // System scopes
    SystemRead,
    SystemWrite,
    SystemAdmin,
    
    // Custom scopes
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIRequest {
    pub request_id: String,
    pub method: String,
    pub path: String,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub user_id: Option<String>,
    pub api_key_id: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub ip_address: String,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIResponse {
    pub request_id: String,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub processing_time: std::time::Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub requests_per_endpoint: HashMap<String, u64>,
    pub requests_per_user: HashMap<String, u64>,
    pub error_rates: HashMap<u16, u64>,
    pub rate_limit_hits: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integration {
    pub integration_id: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub integration_type: IntegrationType,
    pub configuration: IntegrationConfig,
    pub status: IntegrationStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub sync_frequency: u64,
    pub error_count: u32,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationType {
    DataSource,
    Notification,
    Authentication,
    Storage,
    Analytics,
    CICD,
    Monitoring,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub endpoint: String,
    pub authentication: IntegrationAuth,
    pub headers: HashMap<String, String>,
    pub parameters: HashMap<String, String>,
    pub timeout: u64,
    pub retry_config: RetryConfig,
    pub data_mapping: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationAuth {
    None,
    ApiKey { key: String },
    Bearer { token: String },
    Basic { username: String, password: String },
    OAuth2 { client_id: String, client_secret: String, access_token: String, refresh_token: String },
    Custom(HashMap<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: u64,
    pub max_delay: u64,
    pub backoff_multiplier: f64,
    pub retry_on_status: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationStatus {
    Active,
    Inactive,
    Error,
    Syncing,
    Paused,
}

pub struct MarketplaceAPI {
    config: Arc<Mutex<APIConfig>>,
    rest_api: Arc<rest_api::RestAPI>,
    graphql_api: Arc<graphql_api::GraphQLAPI>,
    webhook_api: Arc<webhook_api::WebhookAPI>,
    auth_middleware: Arc<auth_middleware::AuthMiddleware>,
    rate_limiting: Arc<rate_limiting::RateLimiter>,
    api_documentation: Arc<api_documentation::APIDocumentation>,
    sdk_generator: Arc<sdk_generator::SDKGenerator>,
    integration_manager: Arc<integration_manager::IntegrationManager>,
    api_keys: Arc<Mutex<HashMap<String, APIKey>>>,
    integrations: Arc<Mutex<HashMap<String, Integration>>>,
    metrics: Arc<Mutex<APIMetrics>>,
}

impl MarketplaceAPI {
    pub async fn new() -> Result<Self, WarpError> {
        let config = Arc::new(Mutex::new(APIConfig::default()));
        
        Ok(Self {
            config: config.clone(),
            rest_api: Arc::new(rest_api::RestAPI::new(config.clone()).await?),
            graphql_api: Arc::new(graphql_api::GraphQLAPI::new(config.clone()).await?),
            webhook_api: Arc::new(webhook_api::WebhookAPI::new(config.clone()).await?),
            auth_middleware: Arc::new(auth_middleware::AuthMiddleware::new(config.clone()).await?),
            rate_limiting: Arc::new(rate_limiting::RateLimiter::new(config.clone()).await?),
            api_documentation: Arc::new(api_documentation::APIDocumentation::new().await?),
            sdk_generator: Arc::new(sdk_generator::SDKGenerator::new().await?),
            integration_manager: Arc::new(integration_manager::IntegrationManager::new().await?),
            api_keys: Arc::new(Mutex::new(HashMap::new())),
            integrations: Arc::new(Mutex::new(HashMap::new())),
            metrics: Arc::new(Mutex::new(APIMetrics::default())),
        })
    }

    pub async fn start_server(&self, port: u16) -> Result<(), WarpError> {
        // Start REST API server
        let rest_server = self.rest_api.start_server(port).await?;
        
        // Start GraphQL server
        let graphql_server = self.graphql_api.start_server(port + 1).await?;
        
        // Start webhook server
        let webhook_server = self.webhook_api.start_server(port + 2).await?;
        
        // Start metrics collection
        self.start_metrics_collection().await?;
        
        // Wait for all servers
        tokio::try_join!(rest_server, graphql_server, webhook_server)?;
        
        Ok(())
    }

    pub async fn create_api_key(&self, user_id: &str, name: &str, scopes: Vec<APIScope>, expires_at: Option<chrono::DateTime<chrono::Utc>>) -> Result<APIKey, WarpError> {
        let key_id = uuid::Uuid::new_v4().to_string();
        let key_value = self.generate_api_key().await?;
        
        let api_key = APIKey {
            key_id: key_id.clone(),
            key_value: key_value.clone(),
            name: name.to_string(),
            description: String::new(),
            user_id: user_id.to_string(),
            scopes,
            rate_limit: None,
            created_at: chrono::Utc::now(),
            expires_at,
            last_used: None,
            is_active: true,
        };

        let mut api_keys = self.api_keys.lock().await;
        api_keys.insert(key_id.clone(), api_key.clone());

        Ok(api_key)
    }

    pub async fn revoke_api_key(&self, key_id: &str) -> Result<(), WarpError> {
        let mut api_keys = self.api_keys.lock().await;
        if let Some(api_key) = api_keys.get_mut(key_id) {
            api_key.is_active = false;
            Ok(())
        } else {
            Err(WarpError::ConfigError("API key not found".to_string()))
        }
    }

    pub async fn create_integration(&self, user_id: &str, name: &str, integration_type: IntegrationType, config: IntegrationConfig) -> Result<String, WarpError> {
        let integration_id = uuid::Uuid::new_v4().to_string();
        
        let integration = Integration {
            integration_id: integration_id.clone(),
            name: name.to_string(),
            description: String::new(),
            provider: user_id.to_string(),
            integration_type,
            configuration: config,
            status: IntegrationStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_sync: None,
            sync_frequency: 3600, // 1 hour
            error_count: 0,
            last_error: None,
        };

        let mut integrations = self.integrations.lock().await;
        integrations.insert(integration_id.clone(), integration);

        // Register integration with manager
        self.integration_manager.register_integration(&integration_id).await?;

        Ok(integration_id)
    }

    pub async fn test_integration(&self, integration_id: &str) -> Result<bool, WarpError> {
        let integrations = self.integrations.lock().await;
        if let Some(integration) = integrations.get(integration_id) {
            self.integration_manager.test_integration(integration).await
        } else {
            Err(WarpError::ConfigError("Integration not found".to_string()))
        }
    }

    pub async fn sync_integration(&self, integration_id: &str) -> Result<(), WarpError> {
        let mut integrations = self.integrations.lock().await;
        if let Some(integration) = integrations.get_mut(integration_id) {
            integration.status = IntegrationStatus::Syncing;
            integration.last_sync = Some(chrono::Utc::now());
            
            match self.integration_manager.sync_integration(integration).await {
                Ok(_) => {
                    integration.status = IntegrationStatus::Active;
                    integration.error_count = 0;
                    integration.last_error = None;
                }
                Err(e) => {
                    integration.status = IntegrationStatus::Error;
                    integration.error_count += 1;
                    integration.last_error = Some(e.to_string());
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub async fn generate_sdk(&self, language: &str, version: &str) -> Result<Vec<u8>, WarpError> {
        self.sdk_generator.generate_sdk(language, version).await
    }

    pub async fn get_api_documentation(&self, format: &str) -> Result<String, WarpError> {
        self.api_documentation.generate_documentation(format).await
    }

    pub async fn register_webhook(&self, user_id: &str, url: &str, events: Vec<WebhookEvent>, secret: Option<String>) -> Result<String, WarpError> {
        self.webhook_api.register_webhook(user_id, url, events, secret).await
    }

    pub async fn send_webhook(&self, webhook_id: &str, event: WebhookEvent, payload: serde_json::Value) -> Result<(), WarpError> {
        self.webhook_api.send_webhook(webhook_id, event, payload).await
    }

    pub async fn get_metrics(&self) -> Result<APIMetrics, WarpError> {
        let metrics = self.metrics.lock().await;
        Ok(metrics.clone())
    }

    async fn generate_api_key(&self) -> Result<String, WarpError> {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        const KEY_LEN: usize = 64;
        
        let mut rng = rand::thread_rng();
        let key: String = (0..KEY_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        
        Ok(format!("warp_{}", key))
    }

    async fn start_metrics_collection(&self) -> Result<(), WarpError> {
        let metrics = self.metrics.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                // Update metrics
                let mut metrics_guard = metrics.lock().await;
                metrics_guard.last_updated = chrono::Utc::now();
                // Additional metrics collection logic would go here
            }
        });
        
        Ok(())
    }
}

impl Default for APIConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.warp.dev".to_string(),
            version: "v1".to_string(),
            rate_limits: RateLimitConfig {
                requests_per_minute: 100,
                requests_per_hour: 1000,
                requests_per_day: 10000,
                burst_limit: 20,
                whitelist: Vec::new(),
                blacklist: Vec::new(),
            },
            authentication: AuthConfig {
                jwt_secret: uuid::Uuid::new_v4().to_string(),
                token_expiry: 3600,      // 1 hour
                refresh_token_expiry: 604800, // 1 week
                api_key_enabled: true,
                oauth_enabled: true,
                oauth_providers: Vec::new(),
            },
            cors_config: CorsConfig {
                allowed_origins: vec!["*".to_string()],
                allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
                allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
                expose_headers: Vec::new(),
                max_age: 3600,
                credentials: true,
            },
            documentation_enabled: true,
            metrics_enabled: true,
            logging_enabled: true,
            webhook_config: WebhookConfig {
                enabled: true,
                secret_key: uuid::Uuid::new_v4().to_string(),
                retry_attempts: 3,
                timeout: 30,
                supported_events: vec![
                    WebhookEvent::ItemInstalled,
                    WebhookEvent::ItemUninstalled,
                    WebhookEvent::ItemUpdated,
                    WebhookEvent::ItemRated,
                    WebhookEvent::ItemReviewed,
                    WebhookEvent::UserRegistered,
                    WebhookEvent::PaymentCompleted,
                    WebhookEvent::AnalyticsUpdate,
                ],
            },
        }
    }
}

impl Default for APIMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: 0.0,
            requests_per_endpoint: HashMap::new(),
            requests_per_user: HashMap::new(),
            error_rates: HashMap::new(),
            rate_limit_hits: 0,
            last_updated: chrono::Utc::now(),
        }
    }
}
