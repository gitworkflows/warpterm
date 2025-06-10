use crate::error::WarpError;

pub struct SessionMultiplexer;

impl SessionMultiplexer {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self)
    }
}
