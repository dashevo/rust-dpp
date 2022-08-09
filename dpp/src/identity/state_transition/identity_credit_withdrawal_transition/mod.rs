use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    identity::KeyID,
    prelude::Identifier,
    state_transition::{
        StateTransitionConvert, StateTransitionIdentitySigned, StateTransitionLike,
        StateTransitionType,
    },
    util::json_value::{JsonValueExt, ReplaceWith},
    ProtocolError,
};

use super::properties::{
    PROPERTY_IDENTITY_ID, PROPERTY_OUTPUT, PROPERTY_OWNER_ID, PROPERTY_SIGNATURE,
    PROPERTY_SIGNATURE_PUBLIC_KEY_ID,
};

pub mod apply_identity_credit_withdrawal_transition_factory;
pub mod validation;

pub const IDENTIFIER_FIELDS: [&str; 2] = [PROPERTY_IDENTITY_ID, PROPERTY_OWNER_ID];

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Pooling {
    Never = 0,
    IfAvailable = 1,
    Standard = 2,
}

impl std::default::Default for Pooling {
    fn default() -> Self {
        Pooling::Never
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityCreditWithdrawalTransition {
    pub protocol_version: u32,
    #[serde(rename = "type")]
    pub transition_type: StateTransitionType,
    pub identity_id: Identifier,
    pub amount: u64,
    pub core_fee: u64,
    pub pooling: Pooling,
    pub output: Vec<u8>,
    pub signature_public_key_id: KeyID,
    pub signature: Vec<u8>,
}

impl std::default::Default for IdentityCreditWithdrawalTransition {
    fn default() -> Self {
        IdentityCreditWithdrawalTransition {
            protocol_version: Default::default(),
            transition_type: StateTransitionType::IdentityCreditWithdrawal,
            identity_id: Default::default(),
            amount: Default::default(),
            core_fee: Default::default(),
            pooling: Default::default(),
            output: Default::default(),
            signature_public_key_id: Default::default(),
            signature: Default::default(),
        }
    }
}

impl IdentityCreditWithdrawalTransition {
    pub fn from_raw_object(
        mut raw_object: JsonValue,
    ) -> Result<IdentityCreditWithdrawalTransition, ProtocolError> {
        raw_object.replace_identifier_paths(IDENTIFIER_FIELDS, ReplaceWith::Base58)?;

        let transition: IdentityCreditWithdrawalTransition = serde_json::from_value(raw_object)?;

        Ok(transition)
    }

    /// Get owner ID
    pub fn get_owner_id(&self) -> &Identifier {
        &self.identity_id
    }

    /// Returns ID of the created contract
    pub fn get_modified_data_ids(&self) -> Vec<&Identifier> {
        vec![&self.identity_id]
    }
}

impl StateTransitionIdentitySigned for IdentityCreditWithdrawalTransition {
    fn get_signature_public_key_id(&self) -> KeyID {
        self.signature_public_key_id
    }

    fn set_signature_public_key_id(&mut self, key_id: crate::identity::KeyID) {
        self.signature_public_key_id = key_id
    }
}

impl StateTransitionLike for IdentityCreditWithdrawalTransition {
    fn get_protocol_version(&self) -> u32 {
        self.protocol_version
    }

    /// returns the type of State Transition
    fn get_type(&self) -> StateTransitionType {
        self.transition_type
    }

    /// returns the signature as a byte-array
    fn get_signature(&self) -> &Vec<u8> {
        &self.signature
    }

    /// set a new signature
    fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature
    }

    fn calculate_fee(&self) -> Result<u64, ProtocolError> {
        unimplemented!()
    }
}

impl StateTransitionConvert for IdentityCreditWithdrawalTransition {
    fn signature_property_paths() -> Vec<&'static str> {
        vec![PROPERTY_SIGNATURE, PROPERTY_SIGNATURE_PUBLIC_KEY_ID]
    }

    fn identifiers_property_paths() -> Vec<&'static str> {
        vec![PROPERTY_IDENTITY_ID]
    }

    fn binary_property_paths() -> Vec<&'static str> {
        vec![PROPERTY_SIGNATURE, PROPERTY_OUTPUT]
    }
}
