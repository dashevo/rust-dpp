use std::sync::Arc;
use serde_json::Value;
use crate::DashPlatformProtocolInitError;
use crate::identity::validation::TPublicKeysValidator;
use crate::validation::{JsonSchemaValidator, ValidationResult};
use crate::version::ProtocolVersionValidator;

pub struct IdentityCreateTransitionBasicValidator<T, S> {
    protocol_version_validator: Arc<ProtocolVersionValidator>,
    json_schema_validator: JsonSchemaValidator,
    public_keys_validator: Arc<T>,
    public_keys_in_identity_transition_validator: Arc<S>,
    proof_validator: Arc<u64>
}

impl<T: TPublicKeysValidator, S: TPublicKeysValidator> IdentityCreateTransitionBasicValidator<T, S> {
    pub fn new(
        protocol_version_validator: Arc<ProtocolVersionValidator>,
        public_keys_validator: Arc<T>,
        public_keys_in_identity_transition_validator: Arc<S>
    ) -> Result<Self, DashPlatformProtocolInitError> {
        let json_schema_validator =
            JsonSchemaValidator::new(crate::schema::identity::identity_json()?)?;

        let identity_validator = Self {
            protocol_version_validator,
            json_schema_validator,
            public_keys_validator,
            public_keys_in_identity_transition_validator,
            proof_validator: Arc::new(0)
        };

        Ok(identity_validator)
    }

    pub fn validate(&self, identity_create_transition: &Value) -> ValidationResult {
        ValidationResult::default()
    }
}