use thiserror::Error;

#[derive(Debug, Error, Clone, Eq, PartialEq)]
#[error("Serde parsing error: {message:?}")]
pub struct SerdeParsingError {
    message: String,
}

impl SerdeParsingError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
