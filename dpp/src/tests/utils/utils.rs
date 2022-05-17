use anyhow::Result;
use getrandom::getrandom;
use serde_json::Value;
use std::num::ParseIntError;
use sha2::digest::typenum::Len;
use crate::InvalidVectorSizeError;

pub fn generate_random_identifier() -> [u8; 32] {
    let mut buffer = [0u8; 32];
    let _ = getrandom(&mut buffer);
    buffer
}

/// Sets a key value pair in serde_json object, returns the modified object
pub fn serde_set<T, S>(mut object: serde_json::Value, key: T, value: S) -> serde_json::Value
where
    T: Into<String>,
    S: Into<serde_json::Value>,
    serde_json::Value: From<S>,
{
    let map = object
        .as_object_mut()
        .expect("Expected value to be an JSON object");
    map.insert(key.into(), serde_json::Value::from(value));

    object
}

/// Sets a key value pair in serde_json object, returns the modified object
pub fn serde_set_ref<T, S>(object: &mut Value, key: T, value: S)
where
    T: Into<String>,
    S: Into<serde_json::Value>,
    serde_json::Value: From<S>,
{
    let map = object
        .as_object_mut()
        .expect("Expected value to be an JSON object");
    map.insert(key.into(), serde_json::Value::from(value));
}

/// Removes a key value pair in serde_json object, returns the modified object
pub fn serde_remove<T>(mut object: serde_json::Value, key: T) -> serde_json::Value
where
    T: Into<String>,
{
    let map = object
        .as_object_mut()
        .expect("Expected value to be an JSON object");
    map.remove(&key.into());

    object
}

/// Removes a key value pair in serde_json object, returns the modified object
pub fn serde_remove_ref<T>(object: &mut Value, key: T)
where
    T: Into<String>,
{
    object
        .as_object_mut()
        .expect("Expected value to be an JSON object")
        .remove(&key.into());
}

pub fn get_data_from_file(file_path: &str) -> Result<String> {
    let current_dir = std::env::current_dir()?;
    let file_path = format!("{}/{}", current_dir.display(), file_path);
    let d = std::fs::read_to_string(file_path)?;
    Ok(d)
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub trait SerdeTestExtension {
    fn remove_key(&mut self, key: impl Into<String>);
    fn set_key_value<T, S>(&mut self, key: T, value: S) where
        T: Into<String>,
        S: Into<serde_json::Value>,
        serde_json::Value: From<S>;
    fn get_value(&self, key: impl Into<String>) -> &Value;
    fn get_value_mut(&mut self, key: impl Into<String>) -> &mut Value;
}

impl SerdeTestExtension for serde_json::Value {
    fn remove_key(&mut self, key: impl Into<String>) {
        self
            .as_object_mut()
            .expect("Expected value to be an JSON object")
            .remove(&key.into());
    }

    fn set_key_value<T, S>(&mut self, key: T, value: S) where T: Into<String>, S: Into<Value>, Value: From<S> {
        let map = self
            .as_object_mut()
            .expect("Expected value to be an JSON object");
        map.insert(key.into(), serde_json::Value::from(value));
    }

    fn get_value(&self, key: impl Into<String>) -> &Value {
        self.as_object().expect("Expected key to exist").get(&key.into()).expect("Expected key to exist")
    }

    fn get_value_mut(&mut self, key: impl Into<String>) -> &mut Value {
        self.as_object_mut().expect("Expected key to exist").get_mut(&key.into()).expect("Expected key to exist")
    }
}

#[derive(Debug)]
pub enum DecodeError {
    ParseIntError(ParseIntError),
    InvalidVectorSizeError(InvalidVectorSizeError)
}

impl From<InvalidVectorSizeError> for DecodeError {
    fn from(err: InvalidVectorSizeError) -> Self {
        Self::InvalidVectorSizeError(err)
    }
}

impl From<ParseIntError> for DecodeError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

pub fn decode_hex_bls_sig(s: &str) -> Result<[u8; 96], DecodeError> {
    Ok(hex_to_array::<96>(s)?)
}

pub fn decode_hex_sha256(s: &str) -> Result<[u8; 32], DecodeError> {
    Ok(hex_to_array::<32>(s)?)
}

pub fn hex_to_array<const N: usize>(s: &str) -> Result<[u8; N], DecodeError> {
    let vec = decode_hex(s)?;
    Ok(vec_to_array::<N>(&vec)?)
}

pub fn vec_to_array<const N: usize>(vec: &[u8]) -> Result<[u8; N], InvalidVectorSizeError> {
    let mut v: [u8; N] = [0; N];
    // let mut v: T = T::default();
    if v.len() != vec.len() {
        return Err(InvalidVectorSizeError::new(v.len(), vec.len()));
    }
    for i in 0..vec.len() {
        if let Some(n) = vec.get(i) {
            v[i] = *n;
        } else {
            return Err(InvalidVectorSizeError::new(v.len(), vec.len()));
        }
    }
    Ok(v)
}

// fn byte_to_hex(byte: &u8) -> String {
//     format!("{:02x}", byte)
// }
//
// /// Serializes bytes into a hex string
// pub fn encode_hex<T: Clone + Into<Vec<u8>>>(bytes: &T) -> String {
//     let hex_vec: Vec<String> = bytes.clone().into().iter().map(byte_to_hex).collect();
//
//     hex_vec.join("")
// }

/// Assert that all validation error belong to a certain enum variant and
/// extracts all the errors from enum to a vector
#[macro_export]
macro_rules! assert_consensus_errors {
    ($validation_result: expr, $variant:path, $expected_errors_count: expr) => {{
        if $validation_result.errors().len() != $expected_errors_count {
            for error in $validation_result.errors().iter() {
                println!("{:?}", error);
            }
        }

        assert_eq!($validation_result.errors().len(), $expected_errors_count);

        let mut errors = Vec::new();

        for error in $validation_result.errors() {
            match error {
                $variant(err) => errors.push(err),
                _ => panic!("Expected $variant"),
            }
        }

        errors
    }};
}
