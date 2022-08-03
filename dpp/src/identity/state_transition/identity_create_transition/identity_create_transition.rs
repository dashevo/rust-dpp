use crate::identity::state_transition::asset_lock_proof::AssetLockProof;
use crate::identity::{IdentityPublicKey, JsonIdentityPublicKey};
use crate::prelude::Identifier;
use crate::state_transition::StateTransitionType;
use crate::util::json_value::JsonValueExt;
use crate::SerdeParsingError;
use serde_json::Value;
use std::convert::{TryFrom, TryInto};

const PUBLIC_KEYS_PROPERTY_NAME: &str = "publicKeys";
const ASSET_LOCK_PROOF_PROPERTY_NAME: &str = "assetLockProof";

#[derive(Default, Debug, Clone)]
pub struct IdentityCreateTransition {
    public_keys: Vec<IdentityPublicKey>,
    asset_lock_proof: Option<AssetLockProof>,
    identity_id: Identifier,
}

impl IdentityCreateTransition {
    pub fn new(raw_state_transition: serde_json::Value) -> Result<Self, SerdeParsingError> {
        // TODO
        //super(raw_state_transition);

        let mut state_transition = Self::default();

        let transition_map = raw_state_transition.as_object().ok_or_else(|| {
            SerdeParsingError::new("Expected raw identity transition to be a map")
        })?;
        if let Some(keys_value) = transition_map.get(PUBLIC_KEYS_PROPERTY_NAME) {
            let keys_value_arr = keys_value
                .as_array()
                .ok_or_else(|| SerdeParsingError::new("Expected public keys to be an array"))?;
            let keys = keys_value_arr
                .iter()
                .map(|val| serde_json::from_value(val.clone()))
                .collect::<Result<Vec<IdentityPublicKey>, serde_json::Error>>()?;
            state_transition = state_transition.set_public_keys(keys);
        }

        if let Some(proof) = transition_map.get(ASSET_LOCK_PROOF_PROPERTY_NAME) {
            state_transition.set_asset_lock_proof(AssetLockProof::try_from(proof)?);
        }

        Ok(state_transition)
    }

    /// Get State Transition type
    pub fn get_type() -> StateTransitionType {
        StateTransitionType::IdentityCreate
    }

    /// Set asset lock
    pub fn set_asset_lock_proof(&mut self, asset_lock_proof: AssetLockProof) {
        self.identity_id = asset_lock_proof.create_identifier();

        self.asset_lock_proof = Some(asset_lock_proof);
    }

    /// Get asset lock proof
    pub fn get_asset_lock_proof(&self) -> &Option<AssetLockProof> {
        &self.asset_lock_proof
    }

    /// Get identity public keys
    pub fn get_public_keys(&self) -> &[IdentityPublicKey] {
        &self.public_keys
    }

    /// Replaces existing set of public keys with a new one
    pub fn set_public_keys(mut self, public_keys: Vec<IdentityPublicKey>) -> Self {
        self.public_keys = public_keys;

        self
    }

    /// Adds public keys to the existing public keys array
    pub fn add_public_keys(mut self, public_keys: &mut Vec<IdentityPublicKey>) -> Self {
        self.public_keys.append(public_keys);

        self
    }

    /// Returns identity id
    pub fn get_identity_id(&self) -> &Identifier {
        &self.identity_id
    }

    /// Returns Owner ID
    pub fn get_owner_id(&self) -> &Identifier {
        &self.identity_id
    }

    ///
    /// Get raw state transition
    ///
    /// @param {Object} [options]
    /// @param {boolean} [options.skipSignature=false]
    /// @param {boolean} [options.skipIdentifiersConversion=false]
    pub fn to_object(&self, options: ()) {
        // Object.assign(
        // options,
        // {
        // skipIdentifiersConversion: false,
        // ...options,
        // },
        // );
        //
        // return {
        // ...super.toObject(options),
        // assetLockProof: this.getAssetLockProof().toObject(),
        // publicKeys: this.getPublicKeys()
        // .map((publicKey) => publicKey.toObject(options)),
        // };
    }

    /// Get state transition as JSON
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        let mut json = serde_json::Value::Object(Default::default());

        // TODO: super.toJSON()

        if let Some(proof) = &self.asset_lock_proof {
            let proof_val: serde_json::Value = proof.try_into()?;
            json.insert(ASSET_LOCK_PROOF_PROPERTY_NAME.to_string(), proof_val);
        }

        let public_keys = self
            .public_keys
            .iter()
            .map(|pk| {
                let json_key: JsonIdentityPublicKey = pk.into();

                serde_json::to_value(json_key)
            })
            .collect::<Result<Vec<Value>, serde_json::Error>>()?;

        json.insert(
            PUBLIC_KEYS_PROPERTY_NAME.to_string(),
            serde_json::Value::Array(public_keys),
        );

        Ok(json)
    }

    /// Returns ids of created identities
    pub fn get_modified_data_ids(&self) -> Vec<&Identifier> {
        vec![self.get_identity_id()]
    }
}
// @typedef {RawStateTransition & Object} RawIdentityCreateTransition
// @property {RawInstantAssetLockProof|RawChainAssetLockProof} assetLockProof
// @property {RawIdentityPublicKey[]} publicKeys
//
// @typedef {JsonStateTransition & Object} JsonIdentityCreateTransition
// @property {JsonInstantAssetLockProof|JsonChainAssetLockProof} assetLockProof
// @property {JsonIdentityPublicKey[]} publicKeys
