use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

use crate::error::WarpError;

pub struct Logger;

impl Logger {
    pub fn init(debug_mode: bool) -> Result<(), WarpError> {
        let level = if debug_mode {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };

        Builder::from_default_env()
            .filter_level(level)
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{} {} {}:{}] {}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.args()
                )
            })
            .init();

        Ok(())
    }
}
