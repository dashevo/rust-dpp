use serde_json::Value;
use crate::identity::validation::{PublicKeysValidator, TPublicKeysValidator};
use crate::NonConsensusError;
use crate::validation::ValidationResult;

pub struct PublicKeysValidatorMock {
    returns: Result<ValidationResult, NonConsensusError>,
}

impl PublicKeysValidatorMock {
    pub fn new() -> Self {
        Self {
            returns: Default::default()
        }
    }

    pub fn returns(&mut self, result: Result<ValidationResult, NonConsensusError>) {
        self.returns = result;
    }
}

impl TPublicKeysValidator for PublicKeysValidatorMock {
    fn validate_keys(&self, _raw_public_keys: &[Value]) -> Result<ValidationResult, NonConsensusError> {
        self.returns.clone()
    }
}