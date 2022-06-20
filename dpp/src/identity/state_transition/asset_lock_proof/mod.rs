mod asset_lock_proof_validator;
mod asset_lock_transaction_validator;
mod instant;
pub mod chain;
mod asset_lock_public_key_hash_fetcher;
mod asset_lock_transaction_output_fetcher;

pub use asset_lock_proof_validator::*;
pub use asset_lock_transaction_validator::*;
pub use instant::*;

use serde::{Serialize, Deserialize};

// TODO implement!
type ChainAssetLockProof = String;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AssetLockProof {
    Instant(InstantAssetLockProof),
    Chain(ChainAssetLockProof),
}

pub enum AssetLockProofType {
    Instant = 0,
    Chain = 1,
}
