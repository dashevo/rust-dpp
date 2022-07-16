use crate::DPPError;
use crate::identity::errors::AssetLockOutputNotFoundError;
use crate::identity::state_transition::asset_lock_proof::asset_lock_transaction_output_fetcher::{AssetLockTransactionOutputFetcher, ExecutionContext};
use crate::identity::state_transition::asset_lock_proof::AssetLockProof;
use crate::state_repository::StateRepositoryLike;

pub struct AssetLockPublicKeyHashFetcher<SR> where SR: StateRepositoryLike {
    state_repository: SR,
    asset_lock_transaction_output_fetcher: AssetLockTransactionOutputFetcher<SR>
}

impl<SR> AssetLockPublicKeyHashFetcher<SR> where SR: StateRepositoryLike {
    pub fn new() {}

    pub async fn fetch_public_key_hash(&self, asset_lock_proof: AssetLockProof, execution_context: ExecutionContext) -> Result<[u8; 20], DPPError> {
        let output = self.asset_lock_transaction_output_fetcher.fetch(asset_lock_proof, execution_context).await?;

        // TODO
        let pk_hash = &output.script_pubkey;
    }
}