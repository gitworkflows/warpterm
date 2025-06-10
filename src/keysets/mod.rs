use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use crate::error::WarpError;

pub mod manager;
pub mod presets;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: String,
    pub modifiers: Vec<String>,
    pub action: String,
    pub args: Option<Vec<String>>,
    pub when: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySet {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub version: String,
    pub bindings: Vec<KeyBinding>,
}

pub struct KeySetManager {
    keysets: HashMap<String, KeySet>,
    current_keyset: String,
    keyset_directories: Vec<PathBuf>,
}

impl KeySetManager {
    pub async fn new() -> Result<Self, WarpError> {
        let mut manager = Self {
            keysets: HashMap::new(),
            current_keyset: "default".to_string(),
            keyset_directories: vec![
                dirs::config_dir().unwrap_or_default().join("warp/keysets"),
                PathBuf::from("keysets"),
            ],
        };

        manager.load_builtin_keysets().await?;
        manager.discover_keysets().await?;
        
        Ok(manager)
    }

    async fn load_builtin_keysets(&mut self) -> Result<(), WarpError> {
        self.keysets.insert("default".to_string(), presets::default_keyset());
        self.keysets.insert("emacs".to_string(), presets::emacs_keyset());
        self.keysets.insert("vim".to_string(), presets::vim_keyset());
        Ok(())
    }

    async fn discover_keysets(&mut self) -> Result<(), WarpError> {
        for keyset_dir in &self.keyset_directories {
            if keyset_dir.exists() {
                self.load_keysets_from_directory(keyset_dir).await?;
            }
        }
        Ok(())
    }

    async fn load_keysets_from_directory(&mut self, dir: &PathBuf) -> Result<(), WarpError> {
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
               path.extension().and_then(|s| s.to_str()) == Some("yml") {
                if let Ok(keyset) = self.load_keyset_file(&path).await {
                    self.keysets.insert(keyset.name.clone(), keyset);
                }
            }
        }
        
        Ok(())
    }

    async fn load_keyset_file(&self, path: &PathBuf) -> Result<KeySet, WarpError> {
        let content = fs::read_to_string(path).await?;
        let keyset: KeySet = serde_yaml::from_str(&content)
            .map_err(|e| WarpError::ConfigError(format!("Failed to parse keyset: {}", e)))?;
        Ok(keyset)
    }

    pub fn get_keyset(&self, name: &str) -> Option<&KeySet> {
        self.keysets.get(name)
    }

    pub fn get_current_keyset(&self) -> Option<&KeySet> {
        self.keysets.get(&self.current_keyset)
    }

    pub fn set_current_keyset(&mut self, name: String) -> Result<(), WarpError> {
        if self.keysets.contains_key(&name) {
            self.current_keyset = name;
            Ok(())
        } else {
            Err(WarpError::ConfigError(format!("Keyset '{}' not found", name)))
        }
    }

    pub fn list_keysets(&self) -> Vec<&String> {
        self.keysets.keys().collect()
    }
}
