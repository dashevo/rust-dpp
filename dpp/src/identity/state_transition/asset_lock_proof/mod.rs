use serde::{Deserialize, Serialize};

pub use asset_lock_proof_validator::*;
pub use asset_lock_transaction_validator::*;
pub use chain::*;
pub use instant::*;

use crate::identity::state_transition::asset_lock_proof::chain::ChainAssetLockProof;

mod asset_lock_proof_validator;
mod asset_lock_public_key_hash_fetcher;
mod asset_lock_transaction_output_fetcher;
mod asset_lock_transaction_validator;
pub mod chain;
mod instant;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AssetLockProof {
    Instant(InstantAssetLockProof),
    Chain(ChainAssetLockProof),
}

pub enum AssetLockProofType {
    Instant = 0,
    Chain = 1,
}
