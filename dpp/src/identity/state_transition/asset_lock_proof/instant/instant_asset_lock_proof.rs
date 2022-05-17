use dashcore::{InstantLock, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
            instant_lock: InstantLock::default(),
            transaction: Transaction {
                version: 0,
                lock_time: 0,
                input: vec![],
                output: vec![],
            },
            output_index: 0,
        }
    }
}

impl InstantAssetLockProof {
    pub fn new(instant_lock: InstantLock, transaction: Transaction, output_index: u32) -> Self {
        Self {
            // TODO: change the type to a const
            instant_lock,
            transaction,
            output_index,
            asset_lock_type: 0,
        }
    }

    pub fn asset_lock_type(&self) -> u8 {
        self.asset_lock_type
    }

    pub fn instant_lock(&self) -> &InstantLock {
        &self.instant_lock
    }

    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    pub fn output_index(&self) -> u32 {
        self.output_index
    }
}

pub struct RawInstantLock {
    lock_type: u8,
    instant_lock: Vec<u8>,
    transaction: Vec<u8>,
    output_index: u32,
}
