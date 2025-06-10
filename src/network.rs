use crate::error::WarpError;

pub struct NetworkManager;

impl NetworkManager {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self)
    }
}
