use super::*;
use crate::error::WarpError;
use reqwest::{Client, Response};
use serde_json;

pub struct MarketplaceClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl MarketplaceClient {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self {
            client: Client::new(),
            base_url: "https://marketplace.warp.dev/api/v1".to_string(),
            api_key: std::env::var("WARP_MARKETPLACE_API_KEY").ok(),
        })
    }

    pub async fn search(&self, query: SearchQuery) -> Result<SearchResult, WarpError> {
        let url = format!("{}/search", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&query)
            .send()
            .await
            .map_err(|e| WarpError::ConfigError(format!("Search request failed: {}", e)))?;

        self.handle_response(response).await
    }

    pub async fn get_item(&self, id: &str) -> Result<MarketplaceItem, WarpError> {
        let url = format!("{}/items/{}", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| WarpError::ConfigError(format!("Get item request failed: {}", e)))?;

        self.handle_response(response).await
    }

    pub async fn get_reviews(&self, item_id: &str, page: u32) -> Result<Vec<Review>, WarpError> {
        let url = format!("{}/items/{}/reviews?page={}", self.base_url, item_id, page);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| WarpError::ConfigError(format!("Get reviews request failed: {}", e)))?;

        self.handle_response(response).await
    }

    pub async fn download_item(&self, item_id: &str) -> Result<Vec<u8>, WarpError> {
        let url = format!("{}/items/{}/download", self.base_url, item_id);
        
        let mut request = self.client.get(&url);
        
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| WarpError::ConfigError(format!("Download request failed: {}", e)))?;

        if response.status().is_success() {
            response.bytes().await
                .map(|b| b.to_vec())
                .map_err(|e| WarpError::ConfigError(format!("Failed to read download: {}", e)))
        } else {
            Err(WarpError::ConfigError(format!("Download failed with status: {}", response.status())))
        }
    }

    pub async fn submit_rating(&self, item_id: &str, rating: u8, review: Option<String>) -> Result<(), WarpError> {
        let url = format!("{}/items/{}/reviews", self.base_url, item_id);
        
        let payload = serde_json::json!({
            "rating": rating,
            "review": review
        });
        
        let mut request = self.client.post(&url).json(&payload);
        
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| WarpError::ConfigError(format!("Rating submission failed: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(WarpError::ConfigError(format!("Rating submission failed with status: {}", response.status())))
        }
    }

    pub async fn get_featured_items(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        let url = format!("{}/featured", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| WarpError::ConfigError(format!("Featured items request failed: {}", e)))?;

        self.handle_response(response).await
    }

    pub async fn get_trending_items(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        let url = format!("{}/trending", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| WarpError::ConfigError(format!("Trending items request failed: {}", e)))?;

        self.handle_response(response).await
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(&self, response: Response) -> Result<T, WarpError> {
        if response.status().is_success() {
            response.json().await
                .map_err(|e| WarpError::ConfigError(format!("Failed to parse response: {}", e)))
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(WarpError::ConfigError(format!("Request failed with status {}: {}", status, text)))
        }
    }
}
