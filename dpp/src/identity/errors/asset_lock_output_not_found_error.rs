use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Asset Lock transaction output not found")]
pub struct AssetLockOutputNotFoundError {

}

impl AssetLockOutputNotFoundError {
    pub fn new() -> Self {
        Self {}
    }
}