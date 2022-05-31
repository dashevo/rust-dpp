use crate::identifier::Identifier;
use crate::util::hash::hash;
use crate::util::vec::vec_to_array;
use dashcore::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ChainAssetLockProof {
    #[serde(rename = "type")]
    asset_lock_type: u8,
    core_chain_locked_height: u32,
    // outpoints are actually fixed sized, remove vec from here
    out_point: Vec<u8>,
}

impl ChainAssetLockProof {
    pub fn new(core_chain_locked_height: u32, out_point: Vec<u8>) -> Self {
        Self {
            // TODO: change to const
            asset_lock_type: 1,
            core_chain_locked_height,
            out_point,
        }
    }

    /// Get proof type
    pub fn get_type() -> u8 {
        return 1;
    }

    /// Get Asset Lock proof core height
    pub fn get_core_chain_locked_height(&self) -> u32 {
        return self.core_chain_locked_height;
    }

    /// Get out_point
    pub fn get_out_point(&self) -> &[u8] {
        return &self.out_point;
    }

    /// Create identifier
    pub fn create_identifier(&self) -> Identifier {
        return Identifier::new(
            vec_to_array(hash(self.get_out_point()).as_ref())
                .expect("Expected hash function to give a 32 byte output"),
        );
    }
}
