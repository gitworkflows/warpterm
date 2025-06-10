pub mod app;
pub mod error;
pub mod logger;
pub mod terminal;
pub mod ui;
pub mod plugins;
pub mod pty;
pub mod shell;
pub mod history;
pub mod completion;
pub mod search;
pub mod multiplexer;
pub mod network;
pub mod security;
pub mod performance;

pub mod modules {
    pub mod config;
    pub mod ai;
    pub mod themes;
}

pub use error::WarpError;
