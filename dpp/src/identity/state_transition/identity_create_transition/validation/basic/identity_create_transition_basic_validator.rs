use crate::identity::validation::TPublicKeysValidator;
use crate::validation::{JsonSchemaValidator, ValidationResult};
use crate::version::ProtocolVersionValidator;
use crate::{DashPlatformProtocolInitError, NonConsensusError};
use serde_json::Value;
use std::sync::Arc;

use lazy_static::lazy_static;

lazy_static! {
    static ref INDENTITY_CREATE_TRANSITION_SCHEMA: Value =
        serde_json::from_str(include_str!("../../../../../schema/identity/stateTransition/identityCreate.json")).unwrap();
}

pub struct IdentityCreateTransitionBasicValidator<T, S> {
    protocol_version_validator: Arc<ProtocolVersionValidator>,
    json_schema_validator: JsonSchemaValidator,
    public_keys_validator: Arc<T>,
    public_keys_in_identity_transition_validator: Arc<S>,
    proof_validator: Arc<u64>,
}

impl<T: TPublicKeysValidator, S: TPublicKeysValidator>
    IdentityCreateTransitionBasicValidator<T, S>
{
    pub fn new(
        protocol_version_validator: Arc<ProtocolVersionValidator>,
        public_keys_validator: Arc<T>,
        public_keys_in_identity_transition_validator: Arc<S>,
    ) -> Result<Self, DashPlatformProtocolInitError> {
        let json_schema_validator =
            JsonSchemaValidator::new(INDENTITY_CREATE_TRANSITION_SCHEMA)?;

        let identity_validator = Self {
            protocol_version_validator,
            json_schema_validator,
            public_keys_validator,
            public_keys_in_identity_transition_validator,
            proof_validator: Arc::new(0),
        };

        Ok(identity_validator)
    }

    pub fn validate(&self, identity_create_transition: &Value) -> Result<ValidationResult, NonConsensusError> {
        let mut result = self.json_schema_validator.validate(identity_create_transition)?;

        if !result.is_valid() {
            return Ok(result);
        }

        Ok(result)
    }
}
