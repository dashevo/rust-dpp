use dashcore::Transaction;
use crate::identity::state_transition::asset_lock_proof::{AssetLockProof, AssetLockProofType};
use crate::state_repository::StateRepositoryLike;

pub struct AssetLockTransactionOutputFetcher<SR: StateRepositoryLike> {
    stateRepositorty: SR,
}

pub type ExecutionContext = String;

impl<SR: StateRepositoryLike> AssetLockTransactionOutputFetcher<SR> {
    pub fn new(state_repository: SR) -> Self {
        Self {
            stateRepositorty: state_repository
        }
    }

    pub async fn fetch(&self, assetLockProof: AssetLockProof, executionContext: ExecutionContext) -> Result<()> {
        if (assetLockProof.getType() == AssetLockProofType::Instant) {
            return assetLockProof.getOutput();
        }

        if (assetLockProof.getType() == AssetLockProofType::Chain) {
            let outPoint = Transaction.parseOutPointBuffer(assetLockProof.getOutPoint());

            let { outputIndex, transactionHash } = outPoint;

            let rawTransaction = self.stateRepository.fetchTransaction(
                transactionHash,
                executionContext,
            ).await;

            if (rawTransaction.is_none()) {
                Err(AssetLockTransactionIsNotFoundError::new(transactionHash));
            }

            let transaction = new Transaction(rawTransaction.data);
            return transaction.outputs[outputIndex];
        }

        throw new UnknownAssetLockProofTypeError(assetLockProof.getType());
        
        None
    }
}