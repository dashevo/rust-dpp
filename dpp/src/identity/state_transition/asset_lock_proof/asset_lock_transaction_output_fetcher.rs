use dashcore::Transaction;
use crate::identity::errors::{AssetLockTransactionIsNotFoundError, UnknownAssetLockProofTypeError};
use crate::identity::state_transition::asset_lock_proof::{AssetLockProof, AssetLockProofType};
use crate::state_repository::StateRepositoryLike;

pub struct AssetLockTransactionOutputFetcher<SR: StateRepositoryLike> {
    state_repository: SR,
}

pub type ExecutionContext = String;

impl<SR: StateRepositoryLike> AssetLockTransactionOutputFetcher<SR> {
    pub fn new(state_repository: SR) -> Self {
        Self {
            state_repository
        }
    }

    pub async fn fetch(&self, asset_lock_proof: AssetLockProof, execution_context: ExecutionContext) -> Result<(), AssetLockTransactionIsNotFoundError> {
        if asset_lock_proof.getType() == AssetLockProofType::Instant {
            return asset_lock_proof.getOutput();
        }

        if asset_lock_proof.getType() == AssetLockProofType::Chain {
            let out_point = Transaction::parseOutPointBuffer(asset_lock_proof.getOutPoint());

            let output_index = out_point.outputIndex;
            let transaction_hash = out_point.transactionHash;

            let raw_transaction = self.state_repository.fetchTransaction(
                transaction_hash,
                execution_context,
            ).await;

            if raw_transaction.is_none() {
                return Err(AssetLockTransactionIsNotFoundError::new(transaction_hash));
            }

            let transaction = Transaction::new(raw_transaction.data);
            return Ok(transaction.outputs[output_index]);
        }

        Err(UnknownAssetLockProofTypeError::new(asset_lock_proof.getType()))
    }
}