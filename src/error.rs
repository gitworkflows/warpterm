use std::fmt;

#[derive(Debug)]
pub enum WarpError {
    IoError(std::io::Error),
    ConfigError(String),
    TerminalError(String),
    PtyError(String),
    AIError(String),
    PluginError(String),
    SerializationError(String),
}

impl fmt::Display for WarpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WarpError::IoError(e) => write!(f, "IO Error: {}", e),
            WarpError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
            WarpError::TerminalError(msg) => write!(f, "Terminal Error: {}", msg),
            WarpError::PtyError(msg) => write!(f, "PTY Error: {}", msg),
            WarpError::AIError(msg) => write!(f, "AI Error: {}", msg),
            WarpError::PluginError(msg) => write!(f, "Plugin Error: {}", msg),
            WarpError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
        }
    }
}

impl std::error::Error for WarpError {}

impl From<std::io::Error> for WarpError {
    fn from(error: std::io::Error) -> Self {
        WarpError::IoError(error)
    }
}

impl From<crossterm::ErrorKind> for WarpError {
    fn from(error: crossterm::ErrorKind) -> Self {
        WarpError::TerminalError(format!("Crossterm error: {:?}", error))
    }
}

impl From<toml::de::Error> for WarpError {
    fn from(error: toml::de::Error) -> Self {
        WarpError::SerializationError(format!("TOML deserialization error: {}", error))
    }
}

impl From<toml::ser::Error> for WarpError {
    fn from(error: toml::ser::Error) -> Self {
        WarpError::SerializationError(format!("TOML serialization error: {}", error))
    }
}
