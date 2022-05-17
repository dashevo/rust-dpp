use crate::identity::{Purpose, SecurityLevel};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error(
    "Identity doesn't contain any master key, thus can not be updated. Please add a master key"
)]
pub struct MissingMasterPublicKeyError {}

impl MissingMasterPublicKeyError {
    pub fn new() -> Self {
        Self {}
    }
}
