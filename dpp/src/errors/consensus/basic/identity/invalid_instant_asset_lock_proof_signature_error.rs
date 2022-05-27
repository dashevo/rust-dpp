use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Invalid instant lock proof signature")]
pub struct InvalidInstantAssetLockProofSignatureError;

impl InvalidInstantAssetLockProofSignatureError {
    pub fn new() -> Self {
        Self {}
    }
}
