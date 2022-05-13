use dashcore::{InstantLock, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstantAssetLockProof {
    #[serde(rename = "type")]
    asset_lock_type: u8,
    instant_lock: InstantLock,
    transaction: Transaction,
    output_index: u32,
}

impl Default for InstantAssetLockProof {
    fn default() -> Self {
        Self {
            // TODO: change to a const
            asset_lock_type: 0,
            instant_lock: InstantLock::default()
        }
    }
}

pub struct RawInstantLock {
    lock_type: u8,
    instant_lock: Vec<u8>,
    transaction: Vec<u8>,
    output_index: u32,
}

// @typedef {Object} RawInstantAssetLockProof
// @property {number} type
// @property {Buffer} instantLock
// @property {Buffer} transaction
// @property {number} outputIndex