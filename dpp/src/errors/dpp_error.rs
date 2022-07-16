use thiserror::Error;
use crate::identity::errors::{AssetLockOutputNotFoundError, AssetLockTransactionIsNotFoundError};

#[derive(Error, Debug)]
#[error("{0}")]
pub enum DPPError {
    AssetLockOutputNotFoundError(AssetLockOutputNotFoundError),
    AssetLockTransactionIsNotFoundError(AssetLockTransactionIsNotFoundError)
}