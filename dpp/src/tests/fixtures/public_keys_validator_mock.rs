use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Mutex;
use serde_json::Value;
use crate::identity::validation::{PublicKeysValidator, TPublicKeysValidator};
use crate::NonConsensusError;
use crate::validation::ValidationResult;

#[cfg(test)]
pub struct PublicKeysValidatorMock {
    returns: Mutex<Result<ValidationResult, NonConsensusError>>,
    called_with: Mutex<Vec<Value>>,
}

impl PublicKeysValidatorMock {
    pub fn new() -> Self {
        Self {
            returns: Mutex::new(Ok(ValidationResult::default())),
            called_with: Mutex::new(vec![])
        }
    }

    pub fn returns(&self, result: Result<ValidationResult, NonConsensusError>) {
        *self.returns.lock().unwrap() = result;
    }

    pub fn called_with(&self) -> Vec<Value> {
        self.called_with.lock().unwrap().clone()
    }
}

impl TPublicKeysValidator for PublicKeysValidatorMock {
    fn validate_keys(&self, raw_public_keys: &[Value]) -> Result<ValidationResult, NonConsensusError> {
        *self.called_with.lock().unwrap() = Vec::from(raw_public_keys);
        self.returns.lock().unwrap().clone()
    }
}