use dashcore::psbt::serialize::Deserialize;
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
        match asset_lock_proof {
            AssetLockProof::Instant(asset_lock_proof) => {
                Ok(asset_lock_proof.output())
            }
            AssetLockProof::Chain(asset_lock_proof) => {
                let out_point = Transaction::parseOutPointBuffer(asset_lock_proof.out_point());

                let output_index = out_point.outputIndex;
                let transaction_hash = out_point.transactionHash;

                if let Some(raw_transaction) = self.state_repository.fetch_transaction(
                    transaction_hash,
                    execution_context,
                ).await? {
                    let transaction = Transaction::deserialize(&raw_transaction)?;
                    Ok(transaction.outputs[output_index])
                } else {
                    Err(AssetLockTransactionIsNotFoundError::new(transaction_hash))
                }
            }
        }
    }
}