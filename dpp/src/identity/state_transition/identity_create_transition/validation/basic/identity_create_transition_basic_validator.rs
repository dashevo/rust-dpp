use std::sync::Arc;
use crate::DashPlatformProtocolInitError;
use crate::identity::validation::TPublicKeysValidator;
use crate::validation::JsonSchemaValidator;
use crate::version::ProtocolVersionValidator;

pub struct IdentityCreateTransitionBasicValidator<TPublicKeyValidator> {
    protocol_version_validator: Arc<ProtocolVersionValidator>,
    json_schema_validator: JsonSchemaValidator,
    public_keys_validator: Arc<TPublicKeyValidator>,
    public_keys_in_identity_transition_validator: Arc<u64>,
    proof_validator: Arc<u64>
}

impl<T: TPublicKeysValidator> IdentityCreateTransitionBasicValidator<T> {
    pub fn new(
        protocol_version_validator: Arc<ProtocolVersionValidator>,
        public_keys_validator: Arc<T>,
    ) -> Result<Self, DashPlatformProtocolInitError> {
        let json_schema_validator =
            JsonSchemaValidator::new(crate::schema::identity::identity_json()?)?;

        let identity_validator = Self {
            protocol_version_validator,
            json_schema_validator,
            public_keys_validator,
            public_keys_in_identity_transition_validator: Arc::new(0),
            proof_validator: Arc::new(0)
        };

        Ok(identity_validator)
    }
}