use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("{message:?}")]
pub struct TestConsensusError {
    pub message: String
}

impl TestConsensusError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into()
        }
    }
}