use dashcore;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Invalid asset lock transaction: ${message}")]
pub struct InvalidIdentityAssetLockTransactionError {
    message: String,
    validation_error: Option<dashcore::Error>,
}

impl InvalidIdentityAssetLockTransactionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            validation_error: None,
        }
    }

    pub fn set_validation_error(&mut self, error: dashcore::Error) {
        self.validation_error = Some(error);
    }

    pub fn validation_error(&self) -> Option<&dashcore::Error> {
        self.validation_error.as_ref()
    }
}
