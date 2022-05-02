use std::str::FromStr;
use crate::util::string_encoding::{decode, Encoding};
use serde_json::json;
use dashcore::{Transaction, PrivateKey, secp256k1::SecretKey, Network, };
use dashcore::secp256k1::Secp256k1;
use rand::thread_rng;

//3bufpwQjL5qsvuP4fmCKgXJrKG852DDMYfi9J6XKqPAT
//[198, 23, 40, 120, 58, 93, 0, 165, 27, 49, 4, 117, 107, 204,  67, 46, 164, 216, 230, 135, 201, 92, 31, 155, 62, 131, 211, 177, 139, 175, 163, 237]

pub fn instant_asset_lock_proof_json(one_time_private_key: Option<PrivateKey>) -> serde_json::Value {
    let private_key_hex = "cSBnVM4xvxarwGQuAfQFwqDg9k5tErHUHzgWsEfD4zdwUasvqRVY";
    let private_key = PrivateKey::from_str(private_key_hex);
    let from_address = private_key.toAddress();
    let mut rng = thread_rng();

    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rng);
    let one_time_private_key = one_time_private_key.unwrap_or_else(|| PrivateKey::new(secret_key, Network::Testnet));
    let one_time_public_key = one_time_private_key.public_key(&secp);

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
    })
}