pub mod app;
pub mod completion;
pub mod error;
pub mod history;
pub mod logger;
pub mod multiplexer;
pub mod network;
pub mod performance;
pub mod plugins;
pub mod pty;
pub mod search;
pub mod security;
pub mod shell;
pub mod terminal;
pub mod ui;

pub mod modules {
    pub mod ai;
    pub mod config;
    pub mod themes;
}

pub use error::WarpError;
