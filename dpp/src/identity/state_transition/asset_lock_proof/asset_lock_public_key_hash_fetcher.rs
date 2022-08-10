use crate::DPPError;
use crate::identity::state_transition::asset_lock_proof::asset_lock_transaction_output_fetcher::AssetLockTransactionOutputFetcher;
use crate::identity::state_transition::asset_lock_proof::AssetLockProof;
use crate::state_repository::StateRepositoryLike;
use crate::util::vec::vec_to_array;

pub struct AssetLockPublicKeyHashFetcher<SR>
where
    SR: StateRepositoryLike,
{
    state_repository: SR,
    asset_lock_transaction_output_fetcher: AssetLockTransactionOutputFetcher<SR>,
}

impl<SR> AssetLockPublicKeyHashFetcher<SR>
where
    SR: StateRepositoryLike,
{
    pub fn new() {}

    pub async fn fetch_public_key_hash(
        &self,
        asset_lock_proof: AssetLockProof,
    ) -> Result<[u8; 20], DPPError> {
        let output = self
            .asset_lock_transaction_output_fetcher
            .fetch(&asset_lock_proof)
            .await?;

        if output.script_pubkey.is_op_return() {
            let public_key_hash = &output.script_pubkey.as_bytes()[2..];
            vec_to_array(public_key_hash).map_err(|_| DPPError::WrongPublicKeyHashSize)
        } else {
            Err(DPPError::WrongBurnOutputType)
        }
    }
}
