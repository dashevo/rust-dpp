use crate::identity::state_transition::asset_lock_proof::{AssetLockProof, InstantAssetLockProofStructureValidator, PublicKeyHash};
use crate::NonConsensusError;
use crate::state_repository::StateRepositoryLike;
use crate::validation::ValidationResult;

pub struct AssetLockProofValidator<SR: StateRepositoryLike> {
    instant_asset_lock_structure_validator: InstantAssetLockProofStructureValidator<SR>,
}

impl<SR: StateRepositoryLike> AssetLockProofValidator<SR> {
    pub fn new(
        instant_asset_lock_structure_validator: InstantAssetLockProofStructureValidator<SR>,
    ) -> Self {
        Self {
            instant_asset_lock_structure_validator,
        }
    }

    pub async fn validate_structure(&self, raw_asset_lock_proof: &serde_json::Value) -> Result<ValidationResult<PublicKeyHash>, NonConsensusError> {
        let asset_lock: AssetLockProof = serde_json::from_value(raw_asset_lock_proof.clone())?;
        match asset_lock {
            AssetLockProof::Instant(instant_asset_lock) => {
                self.instant_asset_lock_structure_validator.validate(raw_asset_lock_proof).await
            }
            AssetLockProof::Chain(_) => {
                Err(NonConsensusError::SerdeJsonError(String::from("Not implemented")))
            }
        }
    }
}
