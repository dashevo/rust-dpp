use crate::identity::state_transition::asset_lock_proof::{
    AssetLockProof, AssetLockProofType, InstantAssetLockProofStructureValidator, PublicKeyHash,
};
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

    pub async fn validate_structure(
        &self,
        raw_asset_lock_proof: &serde_json::Value,
    ) -> Result<ValidationResult<PublicKeyHash>, NonConsensusError> {
        let asset_lock_type = AssetLockProof::type_from_raw_value(&raw_asset_lock_proof);

        if let Some(proof_type) = asset_lock_type {
            match proof_type {
                AssetLockProofType::Instant => {
                    self.instant_asset_lock_structure_validator
                        .validate(raw_asset_lock_proof)
                        .await
                }
                AssetLockProofType::Chain => Err(NonConsensusError::SerdeJsonError(String::from(
                    "Not implemented",
                ))),
            }
        } else {
            Err(NonConsensusError::SerdeJsonError(String::from(
                "Asset lock proof should have type field",
            )))
        }
    }
}
