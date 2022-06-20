mod asset_lock_proof_validator;
mod asset_lock_transaction_validator;
mod instant;
pub mod chain;
mod asset_lock_public_key_hash_fetcher;
mod asset_lock_transaction_output_fetcher;

pub use asset_lock_proof_validator::*;
pub use asset_lock_transaction_validator::*;
pub use instant::*;
pub use chain::*;

use serde::{Serialize, Deserialize};
use crate::identity::state_transition::asset_lock_proof::chain::ChainAssetLockProof;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AssetLockProof {
    Instant(InstantAssetLockProof),
    Chain(ChainAssetLockProof),
}

pub enum AssetLockProofType {
    Instant = 0,
    Chain = 1,
}
