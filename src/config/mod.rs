use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use tokio::fs;
use crate::error::WarpError;

pub mod manager;
pub mod validation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpConfig {
    pub general: GeneralConfig,
    pub ui: UIConfig,
    pub terminal: TerminalConfig,
    pub ai: AIConfig,
    pub plugins: PluginConfig,
    pub themes: ThemeConfig,
    pub keysets: KeysetConfig,
    pub workflows: WorkflowConfig,
    pub scripting: ScriptingConfig,
    pub ssh: SSHConfig,
    pub docker: DockerConfig,
    pub gpu: GPUConfig,
    pub wasm: WASMConfig,
    pub keybindings: KeybindingConfig,
    pub debug: DebugConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub auto_update: bool,
    pub telemetry: bool,
    pub crash_reporting: bool,
    pub startup_command: Option<String>,
    pub working_directory: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: String,
    pub font_size: u16,
    pub font_family: String,
    pub opacity: f32,
    pub blur: bool,
    pub animations: bool,
    pub tab_bar_position: String,
    pub status_bar: bool,
    pub line_numbers: bool,
    pub minimap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub shell: String,
    pub shell_args: Vec<String>,
    pub scrollback_lines: usize,
    pub cursor_blink: bool,
    pub cursor_style: String,
    pub bell: bool,
    pub bell_sound: Option<String>,
    pub word_separators: String,
    pub copy_on_select: bool,
    pub paste_on_right_click: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub enabled: bool,
    pub provider: String,
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub auto_suggestions: bool,
    pub command_explanation: bool,
    pub error_analysis: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled_plugins: Vec<String>,
    pub plugin_directory: PathBuf,
    pub auto_update_plugins: bool,
    pub plugin_repositories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub current_theme: String,
    pub theme_directories: Vec<PathBuf>,
    pub auto_switch_theme: bool,
    pub light_theme: String,
    pub dark_theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeysetConfig {
    pub current_keyset: String,
    pub keyset_directories: Vec<PathBuf>,
    pub custom_bindings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub enabled: bool,
    pub workflow_directories: Vec<PathBuf>,
    pub auto_execute: bool,
    pub max_concurrent_workflows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptingConfig {
    pub enabled: bool,
    pub default_language: String,
    pub script_directories: Vec<PathBuf>,
    pub timeout: u64,
    pub max_memory: usize,
    pub allowed_modules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHConfig {
    pub enabled: bool,
    pub default_user: Option<String>,
    pub key_directory: PathBuf,
    pub known_hosts_file: PathBuf,
    pub connection_timeout: u64,
    pub keep_alive_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerConfig {
    pub enabled: bool,
    pub socket_path: String,
    pub default_registry: String,
    pub auto_pull_images: bool,
    pub cleanup_containers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUConfig {
    pub enabled: bool,
    pub backend: String, // vulkan, dx12, metal, gl
    pub vsync: bool,
    pub max_fps: u32,
    pub power_preference: String, // low, high
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WASMConfig {
    pub enabled: bool,
    pub max_memory: usize,
    pub timeout: u64,
    pub allowed_imports: Vec<String>,
    pub sandbox_level: String, // strict, moderate, permissive
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingConfig {
    pub copy: String,
    pub paste: String,
    pub new_tab: String,
    pub close_tab: String,
    pub split_horizontal: String,
    pub split_vertical: String,
    pub search: String,
    pub command_palette: String,
    pub settings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub enabled: bool,
    pub log_level: String,
    pub log_file: Option<PathBuf>,
    pub performance_monitoring: bool,
    pub memory_profiling: bool,
}

impl Default for WarpConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                auto_update: true,
                telemetry: false,
                crash_reporting: true,
                startup_command: None,
                working_directory: None,
            },
            ui: UIConfig {
                theme: "standard_dark".to_string(),
                font_size: 14,
                font_family: "JetBrains Mono".to_string(),
                opacity: 0.95,
                blur: true,
                animations: true,
                tab_bar_position: "top".to_string(),
                status_bar: true,
                line_numbers: false,
                minimap: false,
            },
            terminal: TerminalConfig {
                shell: if cfg!(windows) { "powershell".to_string() } else { "zsh".to_string() },
                shell_args: vec![],
                scrollback_lines: 10000,
                cursor_blink: true,
                cursor_style: "block".to_string(),
                bell: false,
                bell_sound: None,
                word_separators: " \t\n\r\"'`()[]{}|\\".to_string(),
                copy_on_select: false,
                paste_on_right_click: true,
            },
            ai: AIConfig {
                enabled: true,
                provider: "openai".to_string(),
                api_key: None,
                model: "gpt-3.5-turbo".to_string(),
                max_tokens: 1000,
                temperature: 0.7,
                auto_suggestions: true,
                command_explanation: true,
                error_analysis: true,
            },
            plugins: PluginConfig {
                enabled_plugins: vec!["git".to_string(), "docker".to_string()],
                plugin_directory: dirs::config_dir().unwrap_or_default().join("warp/plugins"),
                auto_update_plugins: false,
                plugin_repositories: vec!["https://github.com/warpdotdev/plugins".to_string()],
            },
            themes: ThemeConfig {
                current_theme: "standard_dark".to_string(),
                theme_directories: vec![
                    dirs::config_dir().unwrap_or_default().join("warp/themes"),
                ],
                auto_switch_theme: false,
                light_theme: "standard_light".to_string(),
                dark_theme: "standard_dark".to_string(),
            },
            keysets: KeysetConfig {
                current_keyset: "default".to_string(),
                keyset_directories: vec![
                    dirs::config_dir().unwrap_or_default().join("warp/keysets"),
                ],
                custom_bindings: HashMap::new(),
            },
            workflows: WorkflowConfig {
                enabled: true,
                workflow_directories: vec![
                    dirs::config_dir().unwrap_or_default().join("warp/workflows"),
                ],
                auto_execute: false,
                max_concurrent_workflows: 5,
            },
            scripting: ScriptingConfig {
                enabled: true,
                default_language: "lua".to_string(),
                script_directories: vec![
                    dirs::config_dir().unwrap_or_default().join("warp/scripts"),
                ],
                timeout: 30,
                max_memory: 100 * 1024 * 1024, // 100MB
                allowed_modules: vec!["fs".to_string(), "http".to_string()],
            },
            ssh: SSHConfig {
                enabled: true,
                default_user: None,
                key_directory: dirs::home_dir().unwrap_or_default().join(".ssh"),
                known_hosts_file: dirs::home_dir().unwrap_or_default().join(".ssh/known_hosts"),
                connection_timeout: 30,
                keep_alive_interval: 60,
            },
            docker: DockerConfig {
                enabled: true,
                socket_path: if cfg!(windows) {
                    "npipe:////./pipe/docker_engine".to_string()
                } else {
                    "/var/run/docker.sock".to_string()
                },
                default_registry: "docker.io".to_string(),
                auto_pull_images: false,
                cleanup_containers: true,
            },
            gpu: GPUConfig {
                enabled: true,
                backend: "auto".to_string(),
                vsync: true,
                max_fps: 60,
                power_preference: "high".to_string(),
            },
            wasm: WASMConfig {
                enabled: true,
                max_memory: 50 * 1024 * 1024, // 50MB
                timeout: 10,
                allowed_imports: vec!["wasi_snapshot_preview1".to_string()],
                sandbox_level: "strict".to_string(),
            },
            keybindings: KeybindingConfig {
                copy: "Ctrl+C".to_string(),
                paste: "Ctrl+V".to_string(),
                new_tab: "Ctrl+T".to_string(),
                close_tab: "Ctrl+W".to_string(),
                split_horizontal: "Ctrl+Shift+H".to_string(),
                split_vertical: "Ctrl+Shift+V".to_string(),
                search: "Ctrl+F".to_string(),
                command_palette: "Ctrl+Shift+P".to_string(),
                settings: "Ctrl+,".to_string(),
            },
            debug: DebugConfig {
                enabled: false,
                log_level: "info".to_string(),
                log_file: None,
                performance_monitoring: false,
                memory_profiling: false,
            },
        }
    }
}

impl WarpConfig {
    pub async fn load(config_path: Option<&String>) -> Result<Self, WarpError> {
        let path = if let Some(custom_path) = config_path {
            PathBuf::from(custom_path)
        } else {
            Self::default_config_path()?
        };

        if path.exists() {
            let content = fs::read_to_string(&path).await?;
            let config: WarpConfig = toml::from_str(&content)
                .map_err(|e| WarpError::ConfigError(format!("Failed to parse config: {}", e)))?;
            
            // Validate configuration
            validation::validate_config(&config)?;
            
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

        let content = toml::to_string_pretty(self)
            .map_err(|e| WarpError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(path, content).await?;
        Ok(())
    }

    fn default_config_path() -> Result<PathBuf, WarpError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| WarpError::ConfigError("Could not find config directory".to_string()))?;
        Ok(config_dir.join("warp").join("config.toml"))
    }
}
