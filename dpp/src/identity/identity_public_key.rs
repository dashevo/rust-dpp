use crate::errors::ProtocolError;
use anyhow::anyhow;
use dashcore::PublicKey;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::{collections::HashMap, hash::Hash};
use wasm_bindgen::prelude::wasm_bindgen;

pub type KeyID = i64;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum KeyType {
    ECDSA_SECP256K1 = 0,
    BLS12_381 = 1,
    ECDSA_HASH160 = 2,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Purpose {
    /// at least one authentication key must be registered for all security levels
    AUTHENTICATION = 0,
    /// this key cannot be used for signing documents
    ENCRYPTION = 1,
    /// this key cannot be used for signing documents
    DECRYPTION = 2,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SecurityLevel {
    MASTER = 0,
    CRITICAL = 1,
    HIGH = 2,
    MEDIUM = 3,
}

lazy_static! {
    static ref ALLOWED_SECURITY_LEVELS: HashMap<Purpose, Vec<SecurityLevel>> = {
        let mut m = HashMap::new();
        m.insert(
            Purpose::AUTHENTICATION,
            vec![
                SecurityLevel::MASTER,
                SecurityLevel::CRITICAL,
                SecurityLevel::HIGH,
                SecurityLevel::MEDIUM,
            ],
        );
        m.insert(Purpose::ENCRYPTION, vec![SecurityLevel::MEDIUM]);
        m.insert(Purpose::DECRYPTION, vec![SecurityLevel::MEDIUM]);
        m
    };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityPublicKey {
    pub id: KeyID,
    pub purpose: Purpose,
    pub security_level: SecurityLevel,
    #[serde(rename = "type")]
    pub key_type: KeyType,
    #[serde(
        serialize_with = "se_vec_to_base64",
        deserialize_with = "de_base64_to_vec"
    )]
    pub data: Vec<u8>,
    pub read_only: bool,
}

//? do we really need that???
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonIdentityPublicKey {
    pub id: KeyID,
    pub purpose: Purpose,
    pub security_level: SecurityLevel,
    pub key_type: KeyType,
    pub data: String,
    pub read_only: bool,
}

impl std::convert::Into<JsonIdentityPublicKey> for &IdentityPublicKey {
    fn into(self: Self) -> JsonIdentityPublicKey {
        JsonIdentityPublicKey {
            id: self.id,
            purpose: self.purpose,
            security_level: self.security_level,
            key_type: self.key_type,
            read_only: self.read_only,
            data: base64::encode(&self.data),
        }
    }
}

impl IdentityPublicKey {
    /// Get the original public key hash
    pub fn hash(&self) -> Result<Vec<u8>, ProtocolError> {
        if self.data.len() == 0 {
            return Err(ProtocolError::EmptyPublicKeyDataError);
        }
        if self.key_type == KeyType::ECDSA_HASH160 {
            return Ok(self.data.clone());
        }

        // TODO create another error type
        let original_key = PublicKey::from_slice(&self.data)
            .map_err(|e| anyhow!("unable to create pub key - {}", e))?;
        Ok(original_key.pubkey_hash().to_vec())
    }

    pub fn to_object(&self) -> Result<Value, ProtocolError> {
        let data_json = serde_json::to_value(&self.data)?;
        let mut object_json = serde_json::to_value(&self)?;
        if let Value::Object(ref mut o) = object_json {
            o.insert(String::from("data"), data_json);
        } else {
            return Err(anyhow!("identity public key is not an object").into());
        }
        Ok(object_json)
    }

    pub fn to_json(&self) -> Result<Value, ProtocolError> {
        serde_json::to_value(&self)
            .map_err(|e| ProtocolError::EncodingError(format!("corrupted data - {}", e)))
    }
}

pub fn de_base64_to_vec<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    let data: String = Deserialize::deserialize(d)?;
    base64::decode(&data)
        .map_err(|e| serde::de::Error::custom(format!("unable to decode from bas64 - {}", e)))
}

pub fn se_vec_to_base64<S>(buffer: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&base64::encode(&buffer))
}
