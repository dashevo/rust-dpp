use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::convert::TryFrom;

pub use asset_lock_proof_validator::*;
pub use asset_lock_transaction_output_fetcher::*;
pub use asset_lock_transaction_validator::*;
pub use chain::*;
pub use instant::*;

use crate::identity::state_transition::asset_lock_proof::chain::ChainAssetLockProof;
use crate::prelude::Identifier;
use crate::util::json_value::JsonValueExt;
use crate::SerdeParsingError;

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
            0 => Ok(Self::Instant),
            1 => Ok(Self::Chain),
            _ => Err(SerdeParsingError::new("Unexpected asset lock proof type")),
        }
    }
}

impl AssetLockProof {
    /// TODO: Implement
    pub fn create_identifier(&self) -> Identifier {
        Identifier::default()
    }

    pub fn out_point(&self) -> Option<[u8; 36]> {
        match self {
            AssetLockProof::Instant(proof) => proof.out_point(),
            AssetLockProof::Chain(_) => {
                // TODO: Implement
                Some([0; 36])
            }
        }
    }
}

impl TryFrom<&JsonValue> for AssetLockProof {
    type Error = SerdeParsingError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        // let proof_map = value.as_object().ok_or_else(|| SerdeParsingError::new("Expected asset lock proof to be an object"))?;
        let proof_type_int = value
            .get_u64("type")
            .map_err(|e| SerdeParsingError::new(e.to_string()))?;
        let proof_type = AssetLockProofType::try_from(proof_type_int)?;

        match proof_type {
            AssetLockProofType::Instant => {
                Ok(Self::Instant(serde_json::from_value(value.clone())?))
            }
            AssetLockProofType::Chain => Ok(Self::Chain(serde_json::from_value(value.clone())?)),
        }
    }
}

impl TryFrom<AssetLockProof> for JsonValue {
    type Error = serde_json::Error;

    fn try_from(asset_lock_proof: AssetLockProof) -> Result<Self, Self::Error> {
        match asset_lock_proof {
            AssetLockProof::Instant(instant_proof) => serde_json::to_value(instant_proof),
            AssetLockProof::Chain(chain_proof) => serde_json::to_value(chain_proof),
        }
    }
}

impl TryFrom<&AssetLockProof> for JsonValue {
    type Error = serde_json::Error;

    fn try_from(asset_lock_proof: &AssetLockProof) -> Result<Self, Self::Error> {
        match asset_lock_proof {
            AssetLockProof::Instant(instant_proof) => serde_json::to_value(instant_proof),
            AssetLockProof::Chain(chain_proof) => serde_json::to_value(chain_proof),
        }
    }
}
