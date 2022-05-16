// let { getRE2Class } = require("@dashevo/wasm-re2");
//
// let createAjv = require("../../../../../../../lib/ajv/createAjv");
//
// let JsonSchemaValidator = require("../../../../../../../lib/validation/JsonSchemaValidator");
//
// let getIdentityCreateTransitionFixture = require("../../../../../../../lib/test/fixtures/getIdentityCreateTransitionFixture");
//
// let validateIdentityCreateTransitionBasicFactory = require(
//     "../../../../../../../lib/identity/stateTransition/IdentityCreateTransition/validation/basic/validateIdentityCreateTransitionBasicFactory",
// );
//
// let {
// expectJsonSchemaError,
// expectValidationError,
// } = require("../../../../../../../lib/test/expect/expectError");
//
// let ValidationResult = require("../../../../../../../lib/validation/ValidationResult");
// let InstantAssetLockProof = require("../../../../../../../lib/identity/stateTransition/assetLockProof/instant/InstantAssetLockProof");
// let ChainAssetLockProof = require("../../../../../../../lib/identity/stateTransition/assetLockProof/chain/ChainAssetLockProof");
// let TestConsensusError = require("../../../../../../../lib/test/mocks/TestConsensusError");
// let IdentityPublicKey = require("../../../../../../../lib/identity/IdentityPublicKey");

use std::sync::Arc;
use serde_json::Value;
use crate::identity::state_transition::identity_create_transition::validation::basic::IdentityCreateTransitionBasicValidator;
use crate::identity::validation::{PublicKeysInIdentityCreateTransitionValidator, PublicKeysValidator};
use crate::version::ProtocolVersionValidator;

fn setup_test() -> (Value, IdentityCreateTransitionBasicValidator<PublicKeysValidator, PublicKeysInIdentityCreateTransitionValidator>) {
    let protocol_version_validator = ProtocolVersionValidator::default();
    let public_keys_validator = PublicKeysValidator::new().unwrap();
    let other_public_keys_validator = PublicKeysInIdentityCreateTransitionValidator::default();
    (
        crate::tests::fixtures::identity_fixture_json(),
        IdentityCreateTransitionBasicValidator::new(
            Arc::new(protocol_version_validator),
            Arc::new(public_keys_validator),
            Arc::new(other_public_keys_validator)
        )
            .unwrap(),
    )
}

mod validate_identity_create_transition_basic_factory {
    // let validateIdentityCreateTransitionBasic;
    // let rawStateTransition;
    // let stateTransition;
    // let validatePublicKeysMock;
    // let validatePublicKeysInIdentityCreateTransition;
    // let assetLockPublicKeyHash;
    // let proofValidationFunctionsByTypeMock;
    // let validateProtocolVersionMock;

    // beforeEach(async function beforeEach() {
    // validatePublicKeysMock = this.sinonSandbox.stub()
    // .returns(new ValidationResult());
    //
    // validatePublicKeysInIdentityCreateTransition = this.sinonSandbox.stub()
    // .returns(new ValidationResult());
    //
    // assetLockPublicKeyHash = vec![20, 1);
    //
    // let assetLockValidationResult = new ValidationResult();
    //
    // assetLockValidationResult.setData(assetLockPublicKeyHash);
    //
    // let RE2 =  getRE2Class();
    // let ajv = createAjv(RE2);
    //
    // let jsonSchemaValidator = new JsonSchemaValidator(ajv);
    //
    // let proofValidationResult = new ValidationResult();
    // proofValidationResult.setData(assetLockPublicKeyHash);
    //
    // proofValidationFunctionsByTypeMock = {
    // [InstantAssetLockProof.type]: this.sinonSandbox.stub().resolves(proofValidationResult),
    // [ChainAssetLockProof.type]: this.sinonSandbox.stub().resolves(proofValidationResult),
    // };
    //
    // validateProtocolVersionMock = this.sinonSandbox.stub().returns(new ValidationResult());
    //
    // validateIdentityCreateTransitionBasic = validateIdentityCreateTransitionBasicFactory(
    // jsonSchemaValidator,
    // validatePublicKeysMock,
    // validatePublicKeysInIdentityCreateTransition,
    // proofValidationFunctionsByTypeMock,
    // validateProtocolVersionMock,
    // );
    //
    // stateTransition = getIdentityCreateTransitionFixture();
    //
    // let privateKey = "9b67f852093bc61cea0eeca38599dbfba0de28574d2ed9b99d10d33dc1bde7b2";
    //
    //  stateTransition.signByPrivateKey(privateKey, IdentityPublicKey.TYPES.ECDSA_SECP256K1);
    //
    // rawStateTransition = stateTransition.toObject();
    // });

    mod protocol_version {
        use crate::consensus::ConsensusError;
        use crate::consensus::ConsensusError::TestConsensusError;
        use crate::validation::ValidationResult;

        #[test]
        pub fn should_be_present() {
            delete_rawStateTransition.protocolVersion;

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instance_path(),"");
            assert_eq!(error.keyword(),"required");
            assert_eq!(error.getParams().missingProperty,"protocolVersion");
        }

        #[test]
        pub fn should_be_an_integer() {
            rawStateTransition.protocolVersion = "1";

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instance_path(),"/protocolVersion");
            assert_eq!(error.keyword(),"type");
        }

        #[test]
        pub fn should_be_valid() {
            rawStateTransition.protocolVersion = -1;

            let protocol_version_error = ConsensusError::from(TestConsensusError::new("test"));
            let protocol_version_result = ValidationResult::new(Some(vec![protocol_version_error]));

            validateProtocolVersionMock.returns(protocol_version_result);

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectValidationError(result, TestConsensusError);

            let [error] = result.errors();

            assert_eq!(error, protocol_version_error);

            assert_eq!(validateProtocolVersionMock,rawStateTransition.protocolVersion);
        }
    }

    mod type_a {
        #[test]
        pub fn should_be_present() {
            delete_rawStateTransition.type_;

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instance_path(),"");
            assert_eq!(error.keyword(),"required");
            assert_eq!(error.getParams().missingProperty,"type");
        }

        #[test]
        pub fn should_be_equal_to_2() {
            rawStateTransition.type_ = 666;

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instance_path(),"/type");
            assert_eq!(error.keyword(),"let");
            assert_eq!(error.getParams().allowedValue,2);
        }
    }

    mod asset_lock_proof {
        use crate::consensus::basic::TestConsensusError;
        use crate::consensus::ConsensusError;
        use crate::validation::ValidationResult;

        #[test]
        pub fn should_be_present() {
            delete_rawStateTransition.assetLockProof;

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instance_path(),"");
            assert_eq!(error.getParams().missingProperty,"assetLockProof");
            assert_eq!(error.keyword(),"required");
        }

        #[test]
        pub fn should_be_an_object() {
            rawStateTransition.assetLockProof = 1;

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result, 1);

            let [error] = result.errors();

            assert_eq!(error.instance_path(),"/assetLockProof");
            assert_eq!(error.keyword(),"type");
        }

        #[test]
        pub fn should_be_valid() {
            let asset_lock_error = ConsensusError::from(TestConsensusError::new("test"));
            let asset_lock_result = ValidationResult::new(Some(vec![asset_lock_error]));

            proofValidationFunctionsByTypeMock[InstantAssetLockProof.type_]
                .resolves(asset_lock_result);

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectValidationError(result);

            let [error] = result.errors();

            assert_eq!(error, asset_lock_error);

            assert_eq!(proofValidationFunctionsByTypeMock[InstantAssetLockProof.type_],rawStateTransition.assetLockProof);
        }
    }

    mod public_keys {
        use crate::consensus::basic::TestConsensusError;
        use crate::consensus::ConsensusError;
        use crate::validation::ValidationResult;

        #[test]
        pub fn should_be_present() {
            rawStateTransition.publicKeys = undefined;

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instance_path(),"");
            assert_eq!(error.getParams().missingProperty,"publicKeys");
            assert_eq!(error.keyword(),"required");
        }

        #[test]
        pub fn should_not_be_empty() {
            rawStateTransition.publicKeys = [];

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.keyword(),"minItems");
            assert_eq!(error.instance_path(),"/publicKeys");
        }

        #[test]
        pub fn should_not_have_more_than_10_items() {
            let [key] = rawStateTransition.publicKeys;

            // for (let i = 0; i < 10; i++) {
            //     rawStateTransition.publicKeys.push(key);
            // }

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.keyword(),"maxItems");
            assert_eq!(error.instance_path(),"/publicKeys");
        }

        #[test]
        pub fn should_be_unique() {
            rawStateTransition
                .publicKeys
                .push(rawStateTransition.publicKeys[0]);

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.keyword(),"uniqueItems");
            assert_eq!(error.instance_path(),"/publicKeys");
        }

        #[test]
        pub fn should_be_valid() {
            let public_keys_error = ConsensusError::from(TestConsensusError::new("test"));
            let public_keys_result = ValidationResult::new(Some(vec![public_keys_error]));

            validatePublicKeysMock.returns(public_keys_result);

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectValidationError(result);

            let [error] = result.errors();

            assert_eq!(error, public_keys_error);

            assert_eq!(validatePublicKeysMock,rawStateTransition.publicKeys);
        }

        #[test]
        pub fn should_have_at_least_1_master_key() {
            let public_keys_error = ConsensusError::from(TestConsensusError::new("test"));
            let public_keys_result = ValidationResult::new(Some(vec![public_keys_error]));

            validatePublicKeysInIdentityCreateTransition.returns(public_keys_result);

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectValidationError(result);

            let [error] = result.errors();

            assert_eq!(error, public_keys_error);

            assert_eq!(validatePublicKeysInIdentityCreateTransition,rawStateTransition.publicKeys);
        }
    }

    mod signature {
        #[test]
        pub fn should_be_present() {
            delete_rawStateTransition.signature;

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instancePath,"");
            assert_eq!(error.keyword(),"required");
            assert_eq!(error.getParams().missingProperty,"signature");
        }

        #[test]
        pub fn should_be_a_byte_array() {
            rawStateTransition.signature = vec![65; "string"];

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result, 2);

            let [error, byteArrayError] = result.errors();

            assert_eq!(error.instancePath,"/signature/0");
            assert_eq!(error.keyword(),"type");

            assert_eq!(byteArrayError.keyword(),"byteArray");
        }

        #[test]
        pub fn should_be_not_shorter_than_65_bytes() {
            rawStateTransition.signature = vec![64; 0];

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instancePath,"/signature");
            assert_eq!(error.keyword(),"minItems");
        }

        #[test]
        pub fn should_be_not_longer_than_65_bytes() {
            rawStateTransition.signature = vec![66; 0];

            let result = validateIdentityCreateTransitionBasic(rawStateTransition);

            expectJsonSchemaError(result);

            let [error] = result.errors();

            assert_eq!(error.instancePath,"/signature");
            assert_eq!(error.keyword(),"maxItems");
        }
    }

    #[test]
    pub fn should_return_valid_result() {
        let result = validateIdentityCreateTransitionBasic(rawStateTransition);

        assert!(result.isValid());

        assert_eq!(validatePublicKeysMock,rawStateTransition.publicKeys);
    }
}
