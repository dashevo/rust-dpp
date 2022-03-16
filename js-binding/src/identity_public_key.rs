pub use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use dpp::identity::{IdentityPublicKey, KeyType, Purpose, SecurityLevel};

#[wasm_bindgen(js_name=IdentityPublicKey)]
pub struct IdentityPublicKeyWasm(IdentityPublicKey);

// TODO Error Handling

#[wasm_bindgen(js_class = IdentityPublicKey)]
impl IdentityPublicKeyWasm {
    #[wasm_bindgen(js_name=getId)]
    pub fn get_id(&self) -> i64 {
        self.0.id
    }

    #[wasm_bindgen(js_name=setId)]
    pub fn set_id(&mut self, id: i64) {
        self.0.id = id;
    }

    #[wasm_bindgen(js_name=getType)]
    pub fn get_type(&self) -> u8 {
        unimplemented!()
    }

    #[wasm_bindgen(js_name=setType)]
    pub fn set_type(&mut self, key_type: KeyType) {
        self.0.key_type = key_type
    }

    #[wasm_bindgen(js_name=getData)]
    pub fn get_data(&self) -> Vec<u8> {
        self.0.data.clone()
    }

    #[wasm_bindgen(js_name=setData)]
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.0.data = data;
    }

    #[wasm_bindgen(js_name=getPurpose)]
    pub fn get_purpose(&self) -> Purpose {
        self.0.purpose
    }

    #[wasm_bindgen(js_name=setPurpose)]
    pub fn set_purpose(&mut self, purpose: Purpose) {
        self.0.purpose = purpose;
    }

    #[wasm_bindgen(js_name=getSecurityLevel)]
    pub fn get_security_level(&self) -> SecurityLevel {
        self.0.security_level
    }

    #[wasm_bindgen(js_name=setSecurityLevel)]
    pub fn set_security_level(&mut self, security_level: SecurityLevel) {
        self.0.security_level = security_level;
    }

    #[wasm_bindgen(js_name=getReadOnly)]
    pub fn get_readonly(&self) -> bool {
        self.0.read_only
    }

    #[wasm_bindgen(js_name=setReadOnly)]
    pub fn set_readonly(&mut self, ro: bool) {
        self.0.read_only = ro;
    }

    #[wasm_bindgen(js_name=hash)]
    pub fn hash(&self) -> Vec<u8> {
        self.0
            .hash()
            .expect("unable to generate hash for identity public key")
    }

    #[wasm_bindgen(js_name=toObject)]
    pub fn to_object(&self) -> JsValue {
        JsValue::from_serde(
            &self
                .0
                .to_object()
                .expect("unable to to convert to JSON Value"),
        )
        .expect("unable to convert to JS Value")
    }

    #[wasm_bindgen(js_name=toJSON)]
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.0.to_json().expect("unable to convert into JSON value"))
            .expect("unable to convert Identity Public Key to string")
    }
}

impl std::convert::From<IdentityPublicKey> for IdentityPublicKeyWasm {
    fn from(v: IdentityPublicKey) -> Self {
        IdentityPublicKeyWasm(v)
    }
}
