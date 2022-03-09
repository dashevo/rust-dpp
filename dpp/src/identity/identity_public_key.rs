use crate::errors::ProtocolError;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

// TODO test how the enum will look from perspective JavaScript

// TODO import from dashcore
pub struct PublicKey {
    pub hash: Vec<u8>,
}

// TODO import from dashcore
impl<K> std::convert::From<K> for PublicKey
where
    K: AsRef<[u8]>,
{
    fn from(data: K) -> Self {
        return PublicKey {
            hash: Vec::from(data.as_ref()),
        };
    }
}

pub type KeyID = i64;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum KeyType {
    ECDSA_SECP256K1 = 0,
    BLS12_381 = 1,
    ECDSA_HASH160 = 2,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Purpose {
    AUTHENTICATION = 0,
    ENCRYPTION = 1,
    DECRYPTION = 2,
}

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

#[derive(Debug, Serialize, Deserialize)]
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

//TODO  toJSON() should be part of WASM interface - as rust natively support serialization
impl IdentityPublicKey {
    /// Get key ID
    pub fn get_id(&self) -> KeyID {
        self.id
    }

    /// Set key ID
    pub fn set_id(mut self, id: KeyID) -> Self {
        self.id = id;
        self
    }

    /// Get key type
    pub fn get_type(&self) -> KeyType {
        self.key_type
    }

    /// Set key type
    pub fn set_type(mut self, key_type: KeyType) -> Self {
        self.key_type = key_type;
        self
    }

    /// Get raw public key
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Set raw public key
    pub fn set_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }

    /// Get the purpose value
    pub fn get_purpose(&self) -> Purpose {
        self.purpose
    }

    /// Set the purpose value
    pub fn set_purpose(mut self, purpose: Purpose) -> Self {
        self.purpose = purpose;
        self
    }

    /// Get the raw security level value. A uint8 number
    pub fn get_security_level(&self) -> SecurityLevel {
        self.security_level
    }

    /// Set the raw security level
    //? maybe we should replace the enum with impl TryInto<SecurityLevel> or Into<SecurityLevel>
    pub fn set_security_level(mut self, security_level: SecurityLevel) -> Self {
        self.security_level = security_level;
        self
    }

    /// Get readOnly flag
    pub fn get_readonly(&self) -> bool {
        self.read_only
    }

    /// Set readOnly flag
    pub fn set_readonly(mut self, ro: bool) -> Self {
        self.read_only = ro;
        self
    }

    /// Get the original public key hash
    pub fn hash(&self) -> Result<Vec<u8>, ProtocolError> {
        if self.data.len() == 0 {
            return Err(ProtocolError::EmptyPublicKeyDataError);
        }
        if self.key_type == KeyType::ECDSA_HASH160 {
            return Ok(self.data.clone());
        }

        let original_key = PublicKey::from(&self.data);
        Ok(original_key.hash.clone())
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