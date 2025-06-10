use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::error::WarpError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ui: UIConfig,
    pub terminal: TerminalConfig,
    pub ai: AIConfig,
    pub plugins: PluginConfig,
    pub keybindings: KeybindingConfig,
    pub debug: DebugConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: String,
    pub font_size: u16,
    pub font_family: String,
    pub opacity: f32,
    pub blur: bool,
    pub animations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub shell: String,
    pub scrollback_lines: usize,
    pub cursor_blink: bool,
    pub cursor_style: String,
    pub bell: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub enabled: bool,
    pub provider: String,
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled_plugins: Vec<String>,
    pub plugin_directory: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingConfig {
    pub copy: String,
    pub paste: String,
    pub new_tab: String,
    pub close_tab: String,
    pub split_horizontal: String,
    pub split_vertical: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub enabled: bool,
    pub log_level: String,
    pub log_file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UIConfig {
                theme: "dark".to_string(),
                font_size: 14,
                font_family: "JetBrains Mono".to_string(),
                opacity: 0.95,
                blur: true,
                animations: true,
            },
            terminal: TerminalConfig {
                shell: if cfg!(windows) { "powershell".to_string() } else { "zsh".to_string() },
                scrollback_lines: 10000,
                cursor_blink: true,
                cursor_style: "block".to_string(),
                bell: false,
            },
            ai: AIConfig {
                enabled: true,
                provider: "openai".to_string(),
                api_key: None,
                model: "gpt-3.5-turbo".to_string(),
                max_tokens: 1000,
                temperature: 0.7,
            },
            plugins: PluginConfig {
                enabled_plugins: vec!["git".to_string(), "docker".to_string()],
                plugin_directory: dirs::config_dir().unwrap_or_default().join("warp/plugins"),
            },
            keybindings: KeybindingConfig {
                copy: "Ctrl+C".to_string(),
                paste: "Ctrl+V".to_string(),
                new_tab: "Ctrl+T".to_string(),
                close_tab: "Ctrl+W".to_string(),
                split_horizontal: "Ctrl+Shift+H".to_string(),
                split_vertical: "Ctrl+Shift+V".to_string(),
            },
            debug: DebugConfig {
                enabled: false,
                log_level: "info".to_string(),
                log_file: None,
            },
        }
    }
}

impl Config {
    pub async fn load(config_path: Option<&String>) -> Result<Self, WarpError> {
        let path = if let Some(custom_path) = config_path {
            PathBuf::from(custom_path)
        } else {
            Self::default_config_path()?
        };

        if path.exists() {
            let content = fs::read_to_string(&path).await?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let default_config = Self::default();
            default_config.save(&path).await?;
            Ok(default_config)
        }
    }

    pub async fn save(&self, path: &PathBuf) -> Result<(), WarpError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(path, content).await?;
        Ok(())
    }

    fn default_config_path() -> Result<PathBuf, WarpError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| WarpError::ConfigError("Could not find config directory".to_string()))?;
        Ok(config_dir.join("warp").join("config.toml"))
    }
}
