use std::convert::{TryFrom, TryInto};
use anyhow::bail;
use dashcore::{InstantLock, Transaction, TxOut};
use dashcore::consensus::Encodable;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as DeError;
use serde::ser::Error;

use crate::identifier::Identifier;
use crate::util::hash::hash;
use crate::util::vec::vec_to_array;
use crate::{InvalidVectorSizeError, ProtocolError};
use crate::util::cbor_value::CborCanonicalMap;

#[derive(Clone, Debug)]
pub struct InstantAssetLockProof {
    asset_lock_type: u8,
    instant_lock: InstantLock,
    transaction: Transaction,
    output_index: u32,
}

impl Serialize for InstantAssetLockProof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let raw = RawInstantLock::try_from(self).map_err(|e| S::Error::custom(e.to_string()))?;

        // match raw_kek {
        //     Ok(raw) => { raw.serialize(serializer) }
        //     Err(e) => { Err(S::Error::custom(e.to_string())) }
        // }
        // //
        raw.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InstantAssetLockProof {
    fn deserialize<D>(deserializer: D) -> Result<Self,D::Error> where D: Deserializer<'de> {
        let raw = RawInstantLock::deserialize(deserializer)?;
        Ok(raw.try_into().map_err(|e: ProtocolError| D::Error::custom(e.to_string()))?)
    }
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

    pub fn output_index(&self) -> usize {
        self.output_index as usize
    }

    pub fn out_point(&self) -> Option<[u8; 36]> {
        self.transaction.out_point_buffer(self.output_index())
    }

    pub fn output(&self) -> Option<&TxOut> {
        self.transaction.output.get(self.output_index())
    }

    pub fn create_identifier(&self) -> Result<Identifier, InvalidVectorSizeError> {
        // TODO: remove unwrap
        let buffer = hash(
            self.transaction()
                .out_point_buffer(self.output_index())
                .unwrap(),
        );
        Ok(Identifier::new(vec_to_array(&buffer)?))
    }

    pub fn to_buffer(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut map = CborCanonicalMap::new();
        let mut is_lock_buffer = Vec::<u8>::new();
        let mut transaction_buffer = Vec::<u8>::new();
        self.instant_lock.consensus_encode(&mut is_lock_buffer).map_err(|e| ProtocolError::EncodingError(e.to_string()))?;
        self.transaction.consensus_encode(&mut transaction_buffer).map_err(|e| ProtocolError::EncodingError(e.to_string()))?;

        map.insert("type", self.asset_lock_type);
        map.insert("outputIndex", self.output_index);
        map.insert("transaction", transaction_buffer);
        map.insert("instantLock", is_lock_buffer);

        map.to_bytes().map_err(|e| ProtocolError::EncodingError(e.to_string()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawInstantLock {
    #[serde(rename = "type")]
    lock_type: u8,
    instant_lock: Vec<u8>,
    transaction: Vec<u8>,
    output_index: u32,
}

impl TryFrom<RawInstantLock> for InstantAssetLockProof {
    type Error = ProtocolError;

    fn try_from(raw_instant_lock: RawInstantLock) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&InstantAssetLockProof> for RawInstantLock {
    type Error = ProtocolError;

    fn try_from(instant_asset_lock_proof: &InstantAssetLockProof) -> Result<Self, Self::Error> {
        let mut is_lock_buffer = Vec::<u8>::new();
        let mut transaction_buffer = Vec::<u8>::new();
        instant_asset_lock_proof.instant_lock.consensus_encode(&mut is_lock_buffer).map_err(|e| ProtocolError::EncodingError(e.to_string()))?;
        instant_asset_lock_proof.transaction.consensus_encode(&mut transaction_buffer).map_err(|e| ProtocolError::EncodingError(e.to_string()))?;

        Ok(Self {
            lock_type: instant_asset_lock_proof.asset_lock_type,
            instant_lock: is_lock_buffer,
            transaction: transaction_buffer,
            output_index: instant_asset_lock_proof.output_index
        })
    }
}

