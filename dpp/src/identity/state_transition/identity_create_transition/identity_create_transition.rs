use std::convert::TryFrom;
use crate::identity::IdentityPublicKey;
use crate::identity::state_transition::asset_lock_proof::AssetLockProof;
use crate::prelude::Identifier;
use crate::SerdeParsingError;
use crate::state_transition::StateTransitionType;
use crate::util::json_value::JsonValueExt;

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

        let transition_map = raw_state_transition.as_object().ok_or_else(|| SerdeParsingError::new("Expected raw identity transition to be a map"))?;
        if let Some(keys) = transition_map.get(PUBLIC_KEYS_PROPERTY_NAME) {
            // this.setPublicKeys(
            // raw_state_transition.publicKeys
            // .map((rawPublicKey) => new IdentityPublicKey(rawPublicKey)),
            // );
        }

        if let Some(proof) = transition_map.get(ASSET_LOCK_PROOF_PROPERTY_NAME) {
            state_transition.setAssetLockProof(AssetLockProof::try_from(proof)?);
        }

        Ok(state_transition)
    }

    /// Get State Transition type
    pub fn getType() -> StateTransitionType {
        StateTransitionType::IdentityCreate
    }

    /// Set asset lock
    pub fn setAssetLockProof(&mut self, asset_lock_proof: AssetLockProof) {
        self.identity_id = asset_lock_proof.create_identifier();

        self.asset_lock_proof = Some(asset_lock_proof);
    }

    /// Get asset lock proof
    pub fn getAssetLockProof(&self) -> &Option<AssetLockProof> {
        &self.asset_lock_proof
    }

    /// Get identity public keys
    pub fn getPublicKeys(&self) -> &[IdentityPublicKey] {
        &self.public_keys
    }

    /// Replaces existing set of public keys with a new one
    pub fn setPublicKeys(mut self, public_keys: Vec<IdentityPublicKey>) -> Self {
        self.public_keys = public_keys;

        self
    }

    /// Adds public keys to the existing public keys array
    pub fn addPublicKeys(mut self, public_keys: &mut Vec<IdentityPublicKey>) -> Self {
        self.public_keys.append(public_keys);

        self
    }

    /// Returns identity id
    pub fn getIdentityId(&self) -> &Identifier {
       &self.identity_id
    }

    /// Returns Owner ID
    pub fn getOwnerId(&self) -> &Identifier {
        &self.identity_id
    }

    ///
    /// Get raw state transition
    ///
    /// @param {Object} [options]
    /// @param {boolean} [options.skipSignature=false]
    /// @param {boolean} [options.skipIdentifiersConversion=false]
    pub fn toObject(options: ()) {
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
    pub fn toJSON(&self) -> serde_json::Value {
    // return {
    // super.toJSON(),
    // assetLockProof: this.getAssetLockProof().toJSON(),
    // publicKeys: this.getPublicKeys().map((publicKey) => publicKey.toJSON()),
    // };
        let mut json = serde_json::Value::Object(Default::default());

        if let Some(proof) = &self.asset_lock_proof {
            let proof_val: serde_json::Value = proof.into();
            json.insert(ASSET_LOCK_PROOF_PROPERTY_NAME.to_string(), proof_val);
        }

        json
    }

    /// Returns ids of created identities
    pub fn getModifiedDataIds(&self) -> Vec<&Identifier> {
        vec![self.getIdentityId()]
    }
}
// @typedef {RawStateTransition & Object} RawIdentityCreateTransition
// @property {RawInstantAssetLockProof|RawChainAssetLockProof} assetLockProof
// @property {RawIdentityPublicKey[]} publicKeys
//
// @typedef {JsonStateTransition & Object} JsonIdentityCreateTransition
// @property {JsonInstantAssetLockProof|JsonChainAssetLockProof} assetLockProof
// @property {JsonIdentityPublicKey[]} publicKeys

