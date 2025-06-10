use crate::error::WarpError;

pub struct PerformanceMonitor;

impl PerformanceMonitor {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self)
    }
}
