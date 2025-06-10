use super::*;
use crate::error::WarpError;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

pub struct PackageManager {
    installed_packages: HashMap<String, InstalledPackage>,
    package_directory: PathBuf,
    cache_directory: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub auto_update: bool,
    pub dependencies: Vec<String>,
    pub install_path: PathBuf,
}

impl PackageManager {
    pub async fn new() -> Result<Self, WarpError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| WarpError::ConfigError("Could not find config directory".to_string()))?;
        
        let package_directory = config_dir.join("warp/packages");
        let cache_directory = config_dir.join("warp/cache");
        
        // Create directories if they don't exist
        fs::create_dir_all(&package_directory).await?;
        fs::create_dir_all(&cache_directory).await?;
        
        let mut manager = Self {
            installed_packages: HashMap::new(),
            package_directory,
            cache_directory,
        };
        
        manager.load_installed_packages().await?;
        
        Ok(manager)
    }

    async fn load_installed_packages(&mut self) -> Result<(), WarpError> {
        let manifest_path = self.package_directory.join("manifest.json");
        
        if manifest_path.exists() {
            let content = fs::read_to_string(&manifest_path).await?;
            let packages: HashMap<String, InstalledPackage> = serde_json::from_str(&content)
                .map_err(|e| WarpError::ConfigError(format!("Failed to parse manifest: {}", e)))?;
            
            self.installed_packages = packages;
        }
        
        Ok(())
    }

    async fn save_manifest(&self) -> Result<(), WarpError> {
        let manifest_path = self.package_directory.join("manifest.json");
        let content = serde_json::to_string_pretty(&self.installed_packages)
            .map_err(|e| WarpError::ConfigError(format!("Failed to serialize manifest: {}", e)))?;
        
        fs::write(&manifest_path, content).await?;
        Ok(())
    }

    pub async fn install_package(&mut self, item: &MarketplaceItem, package_data: Vec<u8>) -> Result<(), WarpError> {
        let install_path = self.package_directory.join(&item.id);
        
        // Create package directory
        fs::create_dir_all(&install_path).await?;
        
        // Extract package based on type
        match &item.item_type {
            ItemType::Theme(_) => {
                self.install_theme(&item, package_data, &install_path).await?;
            }
            ItemType::Plugin(_) => {
                self.install_plugin(&item, package_data, &install_path).await?;
            }
            ItemType::AIModel(_) => {
                self.install_ai_model(&item, package_data, &install_path).await?;
            }
            ItemType::Keyset(_) => {
                self.install_keyset(&item, package_data, &install_path).await?;
            }
            ItemType::Workflow(_) => {
                self.install_workflow(&item, package_data, &install_path).await?;
            }
            ItemType::Script(_) => {
                self.install_script(&item, package_data, &install_path).await?;
            }
        }
        
        // Add to installed packages
        let installed_package = InstalledPackage {
            id: item.id.clone(),
            name: item.name.clone(),
            version: item.version.clone(),
            installed_at: chrono::Utc::now(),
            auto_update: true,
            dependencies: vec![], // This would be extracted from package metadata
            install_path,
        };
        
        self.installed_packages.insert(item.id.clone(), installed_package);
        self.save_manifest().await?;
        
        Ok(())
    }

    async fn install_theme(&self, item: &MarketplaceItem, package_data: Vec<u8>, install_path: &PathBuf) -> Result<(), WarpError> {
        // Extract theme files
        let archive = zip::ZipArchive::new(std::io::Cursor::new(package_data))
            .map_err(|e| WarpError::ConfigError(format!("Failed to read theme package: {}", e)))?;
        
        // Extract to themes directory
        let themes_dir = dirs::config_dir()
            .ok_or_else(|| WarpError::ConfigError("Could not find config directory".to_string()))?
            .join("warp/themes");
        
        fs::create_dir_all(&themes_dir).await?;
        
        // This would extract the zip file to the themes directory
        // For now, just create a placeholder file
        let theme_file = themes_dir.join(format!("{}.yaml", item.id));
        fs::write(&theme_file, "# Theme placeholder").await?;
        
        Ok(())
    }

    async fn install_plugin(&self, item: &MarketplaceItem, package_data: Vec<u8>, install_path: &PathBuf) -> Result<(), WarpError> {
        // Extract plugin files
        fs::write(&install_path.join("plugin.wasm"), package_data).await?;
        
        // Create plugin manifest
        let manifest = serde_json::json!({
            "id": item.id,
            "name": item.name,
            "version": item.version,
            "entry_point": "plugin.wasm"
        });
        
        fs::write(&install_path.join("manifest.json"), manifest.to_string()).await?;
        
        Ok(())
    }

    async fn install_ai_model(&self, item: &MarketplaceItem, package_data: Vec<u8>, install_path: &PathBuf) -> Result<(), WarpError> {
        // Save model data
        fs::write(&install_path.join("model.bin"), package_data).await?;
        
        // Create model configuration
        let config = serde_json::json!({
            "id": item.id,
            "name": item.name,
            "version": item.version,
            "model_file": "model.bin"
        });
        
        fs::write(&install_path.join("config.json"), config.to_string()).await?;
        
        Ok(())
    }

    async fn install_keyset(&self, item: &MarketplaceItem, package_data: Vec<u8>, install_path: &PathBuf) -> Result<(), WarpError> {
        // Save keyset configuration
        fs::write(&install_path.join("keyset.yaml"), package_data).await?;
        
        Ok(())
    }

    async fn install_workflow(&self, item: &MarketplaceItem, package_data: Vec<u8>, install_path: &PathBuf) -> Result<(), WarpError> {
        // Save workflow configuration
        fs::write(&install_path.join("workflow.yaml"), package_data).await?;
        
        Ok(())
    }

    async fn install_script(&self, item: &MarketplaceItem, package_data: Vec<u8>, install_path: &PathBuf) -> Result<(), WarpError> {
        // Save script file
        if let ItemType::Script(metadata) = &item.item_type {
            let extension = match metadata.language.as_str() {
                "lua" => "lua",
                "javascript" => "js",
                "python" => "py",
                "shell" => "sh",
                _ => "txt",
            };
            
            fs::write(&install_path.join(format!("script.{}", extension)), package_data).await?;
        }
        
        Ok(())
    }

    pub async fn uninstall_package(&mut self, package_id: &str) -> Result<(), WarpError> {
        if let Some(package) = self.installed_packages.remove(package_id) {
            // Remove package files
            if package.install_path.exists() {
                fs::remove_dir_all(&package.install_path).await?;
            }
            
            // Remove from specific directories based on type
            self.remove_from_system_directories(package_id).await?;
            
            self.save_manifest().await?;
        }
        
        Ok(())
    }

    async fn remove_from_system_directories(&self, package_id: &str) -> Result<(), WarpError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| WarpError::ConfigError("Could not find config directory".to_string()))?;
        
        // Remove from themes
        let theme_file = config_dir.join("warp/themes").join(format!("{}.yaml", package_id));
        if theme_file.exists() {
            fs::remove_file(&theme_file).await?;
        }
        
        // Remove from other directories as needed
        
        Ok(())
    }

    pub async fn update_package(&mut self, package_id: &str) -> Result<(), WarpError> {
        if let Some(package) = self.installed_packages.get(package_id) {
            // Check for updates (this would call the marketplace API)
            // For now, just return success
            log::info!("Checking for updates for package: {}", package.name);
        }
        
        Ok(())
    }

    pub async fn check_updates(&self) -> Result<Vec<MarketplaceItem>, WarpError> {
        // This would check all installed packages for updates
        // For now, return empty list
        Ok(vec![])
    }

    pub fn get_installed_packages(&self) -> Vec<&InstalledPackage> {
        self.installed_packages.values().collect()
    }

    pub fn is_installed(&self, package_id: &str) -> bool {
        self.installed_packages.contains_key(package_id)
    }
}
