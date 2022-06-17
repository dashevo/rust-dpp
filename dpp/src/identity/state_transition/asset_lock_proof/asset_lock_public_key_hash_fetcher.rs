use crate::identity::errors::AssetLockOutputNotFoundError;
use crate::identity::state_transition::asset_lock_proof::asset_lock_transaction_output_fetcher::{AssetLockTransactionOutputFetcher, ExecutionContext};
use crate::identity::state_transition::asset_lock_proof::AssetLockProof;
use crate::state_repository::StateRepositoryLike;

pub struct AssetLockPublicKeyHashFetcher<SR> where SR: StateRepositoryLike {
    state_repository: SR,
    asset_lock_transaction_output_fetcher: AssetLockTransactionOutputFetcher
}

impl<SR> AssetLockPublicKeyHashFetcher<SR> where SR: StateRepositoryLike {
    pub fn new() {}

    pub async fn fetch_public_key_hash(&self, asset_lock_proof: AssetLockProof, execution_context: ExecutionContext) -> Result<[u8; 20], AssetLockOutputNotFoundError> {
        let output = self.asset_lock_transaction_output_fetcher.fetch(asset_lock_proof, execution_context).await;

        if let Some(_output) = output {
            //output.script.getData()
            Ok([0; 20])
        } else {
            Err(AssetLockOutputNotFoundError::new())
        }
    }
}