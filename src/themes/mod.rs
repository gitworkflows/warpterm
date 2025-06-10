use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use crate::error::WarpError;

pub mod manager;
pub mod parser;
pub mod standard;
pub mod base16;
pub mod special_edition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpTheme {
    pub name: String,
    pub author: Option<String>,
    pub version: String,
    pub description: Option<String>,
    pub colors: ThemeColors,
    pub ui: ThemeUI,
    pub terminal: ThemeTerminal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection_background: String,
    pub selection_foreground: String,
    pub ansi: AnsiColors,
    pub bright: AnsiColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiColors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeUI {
    pub accent: String,
    pub border: String,
    pub tab_active: String,
    pub tab_inactive: String,
    pub status_bar: String,
    pub menu_background: String,
    pub menu_foreground: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeTerminal {
    pub bright_bold: bool,
    pub cursor_style: String,
    pub cursor_blink: bool,
}

pub struct ThemeManager {
    themes: HashMap<String, WarpTheme>,
    current_theme: String,
    theme_directories: Vec<PathBuf>,
}

impl ThemeManager {
    pub async fn new() -> Result<Self, WarpError> {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme: "standard_dark".to_string(),
            theme_directories: vec![
                dirs::config_dir().unwrap_or_default().join("warp/themes"),
                PathBuf::from("themes"),
            ],
        };

        manager.load_builtin_themes().await?;
        manager.discover_themes().await?;
        
        Ok(manager)
    }

    async fn load_builtin_themes(&mut self) -> Result<(), WarpError> {
        // Load standard themes
        self.themes.insert("standard_dark".to_string(), standard::dark_theme());
        self.themes.insert("standard_light".to_string(), standard::light_theme());
        
        // Load base16 themes
        for theme in base16::get_base16_themes() {
            self.themes.insert(theme.name.clone(), theme);
        }
        
        // Load special edition themes
        for theme in special_edition::get_special_themes() {
            self.themes.insert(theme.name.clone(), theme);
        }
        
        Ok(())
    }

    async fn discover_themes(&mut self) -> Result<(), WarpError> {
        for theme_dir in &self.theme_directories {
            if theme_dir.exists() {
                self.load_themes_from_directory(theme_dir).await?;
            }
        }
        Ok(())
    }

    async fn load_themes_from_directory(&mut self, dir: &PathBuf) -> Result<(), WarpError> {
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
               path.extension().and_then(|s| s.to_str()) == Some("yml") {
                if let Ok(theme) = self.load_theme_file(&path).await {
                    self.themes.insert(theme.name.clone(), theme);
                }
            }
        }
        
        Ok(())
    }

    async fn load_theme_file(&self, path: &PathBuf) -> Result<WarpTheme, WarpError> {
        let content = fs::read_to_string(path).await?;
        let theme: WarpTheme = serde_yaml::from_str(&content)
            .map_err(|e| WarpError::ConfigError(format!("Failed to parse theme: {}", e)))?;
        Ok(theme)
    }

    pub fn get_theme(&self, name: &str) -> Option<&WarpTheme> {
        self.themes.get(name)
    }

    pub fn get_current_theme(&self) -> Option<&WarpTheme> {
        self.themes.get(&self.current_theme)
    }

    pub fn set_current_theme(&mut self, name: String) -> Result<(), WarpError> {
        if self.themes.contains_key(&name) {
            self.current_theme = name;
            Ok(())
        } else {
            Err(WarpError::ConfigError(format!("Theme '{}' not found", name)))
        }
    }

    pub fn list_themes(&self) -> Vec<&String> {
        self.themes.keys().collect()
    }

    pub async fn install_theme_from_url(&mut self, url: &str) -> Result<(), WarpError> {
        let response = reqwest::get(url).await
            .map_err(|e| WarpError::ConfigError(format!("Failed to download theme: {}", e)))?;
        
        let content = response.text().await
            .map_err(|e| WarpError::ConfigError(format!("Failed to read theme content: {}", e)))?;
        
        let theme: WarpTheme = serde_yaml::from_str(&content)
            .map_err(|e| WarpError::ConfigError(format!("Failed to parse downloaded theme: {}", e)))?;
        
        self.themes.insert(theme.name.clone(), theme);
        Ok(())
    }
}
