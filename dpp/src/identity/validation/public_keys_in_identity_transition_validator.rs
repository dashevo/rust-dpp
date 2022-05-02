use crate::consensus::basic::identity::MissingMasterPublicKeyError;
use crate::errors::consensus::basic::identity::{
    DuplicatedIdentityPublicKeyError, DuplicatedIdentityPublicKeyIdError,
    InvalidIdentityPublicKeyDataError, InvalidIdentityPublicKeySecurityLevelError,
};
use crate::identity::{
    IdentityPublicKey, KeyType, Purpose, SecurityLevel, ALLOWED_SECURITY_LEVELS,
};
use crate::validation::{JsonSchemaValidator, ValidationResult};
use crate::{DashPlatformProtocolInitError, NonConsensusError, PublicKeyValidationError};
use bls_signatures::{PublicKey as BlsPublicKey, Serialize};
use dashcore::PublicKey;
use serde_json::Value;
use std::collections::HashMap;
use crate::identity::validation::TPublicKeysValidator;

#[derive(Eq, Hash, PartialEq)]
struct PurposeKey {
    purpose: Purpose,
    security_level: SecurityLevel,
}

#[derive(Default, Clone, Debug)]
pub struct PublicKeysInIdentityCreateTransitionValidator {}

impl TPublicKeysValidator for PublicKeysInIdentityCreateTransitionValidator {
    fn validate_keys(
        &self,
        raw_public_keys: &[Value],
    ) -> Result<ValidationResult, NonConsensusError> {
        let mut result = ValidationResult::default();

        let mut key_purposes_and_levels_count: HashMap<PurposeKey, usize> = HashMap::new();

        for raw_public_key in raw_public_keys {
            let public_key: IdentityPublicKey = serde_json::from_value(raw_public_key.clone())?;
            let combo = PurposeKey {
                purpose: public_key.purpose,
                security_level: public_key.security_level,
            };
            let count = key_purposes_and_levels_count
                .get(&combo)
                .unwrap_or(&0_usize);
            let count = count + 1;
            key_purposes_and_levels_count.insert(combo, count);
        }

        let master_key = PurposeKey {
            purpose: Purpose::AUTHENTICATION,
            security_level: SecurityLevel::MASTER,
        };
        if let None = key_purposes_and_levels_count.get(&master_key) {
            result.add_error(MissingMasterPublicKeyError {});
        }

        Ok(result)
    }
}

impl PublicKeysInIdentityCreateTransitionValidator {
    pub fn new() -> Result<Self, DashPlatformProtocolInitError> {
        Ok(Self::default())
    }
}
