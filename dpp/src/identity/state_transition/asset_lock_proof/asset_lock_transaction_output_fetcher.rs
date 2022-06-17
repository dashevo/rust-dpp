use crate::identity::state_transition::asset_lock_proof::AssetLockProof;

pub struct AssetLockTransactionOutputFetcher {

}

pub type ExecutionContext = String;

impl AssetLockTransactionOutputFetcher {
    pub async fn fetch(&self, assetLockProof: AssetLockProof, executionContext: ExecutionContext) -> Option<()> {
        None
    }
}