use crate::error::WarpError;

pub struct SecurityManager;

impl SecurityManager {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self)
    }
}
