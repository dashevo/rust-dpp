mod asset_lock_proof_validator;
mod asset_lock_transaction_validator;
mod instant;
pub mod chain;

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
