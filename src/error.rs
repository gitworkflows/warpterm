use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WarpError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    #[error("PTY error: {0}")]
    PtyError(String),
}

impl fmt::Display for WarpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WarpError::Io(e) => write!(f, "IO Error: {}", e),
            WarpError::Utf8(e) => write!(f, "UTF-8 Error: {}", e),
            WarpError::Terminal(msg) => write!(f, "Terminal Error: {}", msg),
            WarpError::CommandExecution(msg) => write!(f, "Command Execution Error: {}", msg),
            WarpError::PtyError(msg) => write!(f, "PTY Error: {}", msg),
        }
    }
}

impl std::error::Error for WarpError {}

impl WarpError {
    pub fn terminal_err(msg: impl Into<String>) -> Self {
        WarpError::Terminal(msg.into())
    }

    pub fn command_err(msg: impl Into<String>) -> Self {
        WarpError::CommandExecution(msg.into())
    }

    pub fn pty_err(msg: impl Into<String>) -> Self {
        WarpError::PtyError(msg.into())
    }
}
