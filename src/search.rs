use crate::error::WarpError;

pub struct SearchEngine;

impl SearchEngine {
    pub async fn new() -> Result<Self, WarpError> {
        Ok(Self)
    }
}
