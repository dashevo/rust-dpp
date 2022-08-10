use std::convert::TryFrom;

use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as JsonValue;

pub use asset_lock_proof_validator::*;
pub use asset_lock_transaction_output_fetcher::*;
pub use asset_lock_transaction_validator::*;
pub use chain::*;
pub use instant::*;

use crate::identity::state_transition::asset_lock_proof::chain::ChainAssetLockProof;
use crate::prelude::Identifier;
use crate::util::json_value::JsonValueExt;
use crate::{InvalidVectorSizeError, SerdeParsingError};

mod asset_lock_proof_validator;
mod asset_lock_public_key_hash_fetcher;
mod asset_lock_transaction_output_fetcher;
mod asset_lock_transaction_validator;
pub mod chain;
mod instant;

#[derive(Clone, Debug)]
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
        self
    }
}

impl Serialize for AssetLockProof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AssetLockProof::Instant(instant_proof) => instant_proof.serialize(serializer),
            AssetLockProof::Chain(chain) => chain.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for AssetLockProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let proof_type_int = value
            .get_u64("type")
            .map_err(|e| D::Error::custom(e.to_string()))?;
        let proof_type = AssetLockProofType::try_from(proof_type_int)
            .map_err(|e| D::Error::custom(e.to_string()))?;

        match proof_type {
            AssetLockProofType::Instant => Ok(Self::Instant(
                serde_json::from_value(value).map_err(|e| D::Error::custom(e.to_string()))?,
            )),
            AssetLockProofType::Chain => Ok(Self::Chain(
                serde_json::from_value(value).map_err(|e| D::Error::custom(e.to_string()))?,
            )),
        }
    }
}

// impl Serialize for IdentityCreateTransition {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//     {
//         let raw = self
//             .to_json_object(Default::default())
//             .map_err(|e| S::Error::custom(e.to_string()))?;
//
//         raw.serialize(serializer)
//     }
// }
//
// impl<'de> Deserialize<'de> for IdentityCreateTransition {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: Deserializer<'de>,
//     {
//         let value = serde_json::Value::deserialize(deserializer)?;
//
//         Self::new(value).map_err(|e| D::Error::custom(e.to_string()))
//     }
// }

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
    pub fn type_from_raw_value(value: &JsonValue) -> Option<AssetLockProofType> {
        let proof_type_res = value.get_u64("type");

        match proof_type_res {
            Ok(proof_type_int) => {
                let proof_type = AssetLockProofType::try_from(proof_type_int);
                match proof_type {
                    Ok(pt) => Some(pt),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    pub fn create_identifier(&self) -> Result<Identifier, InvalidVectorSizeError> {
        match self {
            AssetLockProof::Instant(instant_proof) => instant_proof.create_identifier(),
            AssetLockProof::Chain(chain_proof) => {
                // TODO: fix return type
                Ok(chain_proof.create_identifier())
            }
        }
    }

    pub fn out_point(&self) -> Option<[u8; 36]> {
        match self {
            AssetLockProof::Instant(proof) => proof.out_point(),
            AssetLockProof::Chain(proof) => Some(*proof.out_point()),
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
