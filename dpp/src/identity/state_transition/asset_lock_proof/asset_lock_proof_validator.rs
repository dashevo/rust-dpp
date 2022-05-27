use crate::identity::state_transition::asset_lock_proof::{
    AssetLockProof, InstantAssetLockProofStructureValidator,
};
use crate::state_repository::StateRepositoryLike;
use crate::validation::ValidationResult;
use crate::NonConsensusError;

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

    // pub fn validate_structure(&self, asset_lock_proof: AssetLockProof) -> Result<ValidationResult<()>, NonConsensusError> {
    //     match asset_lock_proof {
    //         AssetLockProof::Instant(instant_asset_lock) => {
    //             self.instant_asset_lock_structure_validator.validate(instant_asset_lock)
    //         }
    //         AssetLockProof::Chain(_) => {}
    //     }
    // }
}
