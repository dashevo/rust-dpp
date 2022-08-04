use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use asset_lock_proof_validator::*;
pub use asset_lock_transaction_validator::*;
pub use chain::*;
pub use instant::*;

use crate::identity::state_transition::asset_lock_proof::chain::ChainAssetLockProof;
use crate::prelude::Identifier;
use crate::SerdeParsingError;
use crate::util::json_value::JsonValueExt;

mod asset_lock_proof_validator;
mod asset_lock_public_key_hash_fetcher;
mod asset_lock_transaction_output_fetcher;
mod asset_lock_transaction_validator;
pub mod chain;
mod instant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AssetLockProof {
    Instant(InstantAssetLockProof),
    Chain(ChainAssetLockProof),
}

impl Default for AssetLockProof {
    fn default() -> Self {
        Self::Instant(InstantAssetLockProof::default())
    }
}

impl AsRef<AssetLockProof> for AssetLockProof {
    fn as_ref(&self) -> &AssetLockProof {
        &self
    }
}

pub enum AssetLockProofType {
    Instant = 0,
    Chain = 1,
}

impl TryFrom<u64> for AssetLockProofType {
    type Error = SerdeParsingError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => { Ok(Self::Instant) },
            1 => { Ok(Self::Chain) },
            _ => { Err(SerdeParsingError::new("Unexpected asset lock proof type")) }
        }
    }
}

impl AssetLockProof {
    /// TODO: Implement
    pub fn create_identifier(&self) -> Identifier {
        Identifier::default()
    }
}

impl TryFrom<&serde_json::Value> for AssetLockProof {
    type Error = SerdeParsingError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        //let proof_map = value.as_object().ok_or_else(|| SerdeParsingError::new("Expected asset lock proof to be an object"))?;
        let proof_type_int = value.get_u64("type").map_err(|e| SerdeParsingError::new(e.to_string()))?;
        let proof_type = AssetLockProofType::try_from(proof_type_int)?;

        match proof_type {
            AssetLockProofType::Instant => {
                Ok(Self::Instant(serde_json::from_value(value.clone())?))
            }
            AssetLockProofType::Chain => {
                Ok(Self::Chain(serde_json::from_value(value.clone())?))
            }
        }
    }
}

impl TryFrom<AssetLockProof> for serde_json::Value {
    type Error = serde_json::Error;

    fn try_from(asset_lock_proof: AssetLockProof) -> Result<Self, Self::Error> {
        match asset_lock_proof {
            AssetLockProof::Instant(instant_proof) => {
                serde_json::to_value(instant_proof)
            }
            AssetLockProof::Chain(chain_proof) => {
                serde_json::to_value(chain_proof)
            }
        }
    }
}

impl TryFrom<&AssetLockProof> for serde_json::Value {
    type Error = serde_json::Error;

    fn try_from(asset_lock_proof: &AssetLockProof) -> Result<Self, Self::Error> {
        match asset_lock_proof {
            AssetLockProof::Instant(instant_proof) => {
                serde_json::to_value(instant_proof)
            }
            AssetLockProof::Chain(chain_proof) => {
                serde_json::to_value(chain_proof)
            }
        }
    }
}
