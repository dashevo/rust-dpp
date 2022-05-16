use dashcore::PrivateKey;
use crate::util::string_encoding::{decode, Encoding};
use serde_json::{json, Value};
use crate::identity::{KeyType, Purpose, SecurityLevel};
use crate::tests::fixtures::{identity_create_transition_fixture, instant_asset_lock_proof_json};
use crate::version;

//3bufpwQjL5qsvuP4fmCKgXJrKG852DDMYfi9J6XKqPAT
//[198, 23, 40, 120, 58, 93, 0, 165, 27, 49, 4, 117, 107, 204,  67, 46, 164, 216, 230, 135, 201, 92, 31, 155, 62, 131, 211, 177, 139, 175, 163, 237]

pub fn identity_create__transition_fixture_json(one_time_private_key: Option<PrivateKey>) -> serde_json::Value {
    json!({
        "protocolVersion": 1,
        "id": [198, 23, 40, 120, 58, 93, 0, 165, 27, 49, 4, 117, 107, 204,  67, 46, 164, 216, 230, 135, 201, 92, 31, 155, 62, 131, 211, 177, 139, 175, 163, 237],
        "publicKeys": [
            {
                "id": 0,
                "type": 0,
                "purpose": 0,
                "securityLevel": 0,
                "data": decode("AuryIuMtRrl/VviQuyLD1l4nmxi9ogPzC9LT7tdpo0di", Encoding::Base64).unwrap(),
                "readOnly": false
            },
            {
                "id": 1,
                "type": 0,
                "purpose": 1,
                "securityLevel": 3,
                "data": decode("A8AK95PYMVX5VQKzOhcVQRCUbc9pyg3RiL7jttEMDU+L", Encoding::Base64).unwrap(),
                "readOnly": false
            }
        ],
        "balance": 10,
        "revision": 0
    });

    let asset_lock_proof = instant_asset_lock_proof_json(one_time_private_key);
    let asset_lock_proof_json: Value = asset_lock_proof.into();

    json!({
        "protocolVersion": version::LATEST_VERSION,
        // TODO: change to a const
        "type": 2,
        "assetLockProof": asset_lock_proof_json,
        "publicKeys": [
            {
                "id": 0,
                "type": KeyType::ECDSA_SECP256K1,
                "data": decode("AuryIuMtRrl/VviQuyLD1l4nmxi9ogPzC9LT7tdpo0di", Encoding::Base64).unwrap(),
                "purpose": Purpose::AUTHENTICATION,
                "securityLevel": SecurityLevel::MASTER,
                "readOnly": false,
            },
        ]
    })
}