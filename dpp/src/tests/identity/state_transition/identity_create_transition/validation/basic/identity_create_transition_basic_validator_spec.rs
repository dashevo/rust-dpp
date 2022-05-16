// let { getRE2Class } = require("@dashevo/wasm-re2");
//
// let createAjv = require("../../../../../../../lib/ajv/createAjv");
//
// let JsonSchemaValidator = require("../../../../../../../lib/validation/JsonSchemaValidator");
//
// let getIdentityCreateTransitionFixture = require("../../../../../../../lib/test/fixtures/getIdentityCreateTransitionFixture");
//
// let validator.validateFactory = require(
//     "../../../../../../../lib/identity/stateTransition/IdentityCreateTransition/validation/basic/validator.validateFactory",
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

use crate::identity::state_transition::identity_create_transition::validation::basic::IdentityCreateTransitionBasicValidator;
use crate::identity::validation::{PublicKeysInIdentityCreateTransitionValidator, PublicKeysValidator, TPublicKeysValidator};
use crate::version::ProtocolVersionValidator;
use serde_json::Value;
use std::sync::Arc;

fn setup_test(public_keys_validator_mock: Option<impl TPublicKeysValidator>, public_keys_transition_validator_mock: Option<impl TPublicKeysValidator>) -> (
    Value,
    IdentityCreateTransitionBasicValidator<
        PublicKeysValidator,
        PublicKeysInIdentityCreateTransitionValidator,
    >,
) {
    let protocol_version_validator = ProtocolVersionValidator::default();
    let public_keys_validator = match public_keys_validator_mock {
        None => { PublicKeysValidator::new().unwrap() }
        Some(validator) => { validator }
    };
    let other_public_keys_validator = match public_keys_transition_validator_mock {
        None => { PublicKeysInIdentityCreateTransitionValidator::default() }
        Some(validator) => { validator }
    };
    (
        // TODO: should it really be None?
        crate::tests::fixtures::identity_create_transition_fixture_json(None),
        IdentityCreateTransitionBasicValidator::new(
            Arc::new(protocol_version_validator),
            Arc::new(public_keys_validator),
            Arc::new(other_public_keys_validator),
        )
        .unwrap(),
    )
}

mod validate_identity_create_transition_basic_factory {
    use std::option::Option::None;
    use super::setup_test;
    // let validator.validate;
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
    // validator.validate = validator.validateFactory(
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
        use std::option::Option::None;
        use crate::assert_consensus_errors;
        use crate::consensus::basic::TestConsensusError;
        use crate::consensus::ConsensusError;
        use crate::tests::identity::state_transition::identity_create_transition::validation::basic::identity_create_transition_basic_validator_spec::setup_test;
        use crate::tests::utils::{SerdeTestExtension};
        use crate::validation::ValidationResult;

        #[test]
        pub fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.remove_key("protocolVersion");

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);
            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instance_path(), "");
            assert_eq!(error.keyword(), "required");
            assert_eq!(error.getParams().missingProperty, "protocolVersion");
        }

        #[test]
        pub fn should_be_an_integer() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.set_key_value("protocolVersion", "1");

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instance_path(), "/protocolVersion");
            assert_eq!(error.keyword(), "type");
        }

        #[test]
        pub fn should_be_valid() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.set_key_value("protocolVersion", -1);

            let protocol_version_error = ConsensusError::from(TestConsensusError::new("test"));
            let protocol_version_result = ValidationResult::new(Some(vec![protocol_version_error]));

            validateProtocolVersionMock.returns(protocol_version_result);

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::TestConsensusError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error, protocol_version_error);

            assert_eq!(
                validateProtocolVersionMock,
                raw_state_transition.protocolVersion
            );
        }
    }

    mod type_a {
        use std::option::Option::None;
        use super::super::setup_test;
        use crate::assert_consensus_errors;
        use crate::consensus::ConsensusError;
        use crate::tests::utils::SerdeTestExtension;

        #[test]
        pub fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.remove_key("type");
            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instance_path(), "");
            assert_eq!(error.keyword(), "required");
            assert_eq!(error.getParams().missingProperty, "type");
        }

        #[test]
        pub fn should_be_equal_to_2() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.set_key_value("type", 666);

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instance_path(), "/type");
            assert_eq!(error.keyword(), "let");
            assert_eq!(error.getParams().allowedValue, 2);
        }
    }

    mod asset_lock_proof {
        use std::option::Option::None;
        use super::super::setup_test;
        use crate::assert_consensus_errors;
        use crate::consensus::basic::TestConsensusError;
        use crate::consensus::ConsensusError;
        use crate::tests::utils::SerdeTestExtension;
        use crate::validation::ValidationResult;

        #[test]
        pub fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.remove_key("assetLockProof");

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instance_path(), "");
            assert_eq!(error.getParams().missingProperty, "assetLockProof");
            assert_eq!(error.keyword(), "required");
        }

        #[test]
        pub fn should_be_an_object() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.set_key_value("assetLockProof", 1);

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instance_path(), "/assetLockProof");
            assert_eq!(error.keyword(), "type");
        }

        #[test]
        pub fn should_be_valid() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            let asset_lock_error = ConsensusError::from(TestConsensusError::new("test"));
            let asset_lock_result = ValidationResult::new(Some(vec![asset_lock_error]));

            proofValidationFunctionsByTypeMock[InstantAssetLockProof.type_]
                .resolves(asset_lock_result);

            let result = validator.validate(&raw_state_transition);

            assert_eq!(result.errors().len(), 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error, asset_lock_error);

            assert_eq!(
                proofValidationFunctionsByTypeMock[InstantAssetLockProof.type_],
                raw_state_transition.assetLockProof
            );
        }
    }

    mod public_keys {
        use std::option::Option::None;
        use super::super::setup_test;
        use crate::assert_consensus_errors;
        use crate::consensus::basic::TestConsensusError;
        use crate::consensus::ConsensusError;
        use crate::tests::fixtures::PublicKeysValidatorMock;
        use crate::tests::utils::SerdeTestExtension;
        use crate::validation::ValidationResult;

        #[test]
        pub fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.remove_key("publicKeys");

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instance_path(), "");
            assert_eq!(error.getParams().missingProperty, "publicKeys");
            assert_eq!(error.keyword(), "required");
        }

        #[test]
        pub fn should_not_be_empty() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.set_key_value("publicKeys", vec![]);

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.keyword(), "minItems");
            assert_eq!(error.instance_path(), "/publicKeys");
        }

        #[test]
        pub fn should_not_have_more_than_10_items() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            let [key] = raw_state_transition.publicKeys;

            // for (let i = 0; i < 10; i++) {
            //     raw_state_transition.publicKeys.push(key);
            // }

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.keyword(), "maxItems");
            assert_eq!(error.instance_path(), "/publicKeys");
        }

        #[test]
        pub fn should_be_unique() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition
                .publicKeys
                .push(raw_state_transition.publicKeys[0]);

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.keyword(), "uniqueItems");
            assert_eq!(error.instance_path(), "/publicKeys");
        }

        #[test]
        pub fn should_be_valid() {
            let mut pk_validator_mock = PublicKeysValidatorMock::new();
            let public_keys_error = ConsensusError::from(TestConsensusError::new("test"));
            let public_keys_result = ValidationResult::new(Some(vec![public_keys_error]));
            pk_validator_mock.returns(Ok(public_keys_result));

            let (mut raw_state_transition, validator) = setup_test(pk_validator_mock, None);

            let result = validator.validate(&raw_state_transition);

            assert_eq!(result.errors().len(), 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error, public_keys_error);

            assert_eq!(validatePublicKeysMock, raw_state_transition.publicKeys);
        }

        #[test]
        pub fn should_have_at_least_1_master_key() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            let public_keys_error = ConsensusError::from(TestConsensusError::new("test"));
            let public_keys_result = ValidationResult::new(Some(vec![public_keys_error]));

            validatePublicKeysInIdentityCreateTransition.returns(public_keys_result);

            let result = validator.validate(&raw_state_transition);

            assert_eq!(result.errors().len(), 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error, public_keys_error);

            assert_eq!(
                validatePublicKeysInIdentityCreateTransition,
                raw_state_transition.publicKeys
            );
        }
    }

    mod signature {
        use std::option::Option::None;
        use super::super::setup_test;
        use crate::assert_consensus_errors;
        use crate::consensus::ConsensusError;
        use crate::tests::utils::SerdeTestExtension;

        #[test]
        pub fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.remove_key("signature");

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instancePath, "");
            assert_eq!(error.keyword(), "required");
            assert_eq!(error.getParams().missingProperty, "signature");
        }

        #[test]
        pub fn should_be_a_byte_array() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.set_key_value("signature", vec![65; "string"]);

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 2);

            let [error, byteArrayError] = result.errors();

            assert_eq!(error.instancePath, "/signature/0");
            assert_eq!(error.keyword(), "type");

            assert_eq!(byteArrayError.keyword(), "byteArray");
        }

        #[test]
        pub fn should_be_not_shorter_than_65_bytes() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.set_key_value("signature", vec![64; 0]);

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instancePath, "/signature");
            assert_eq!(error.keyword(), "minItems");
        }

        #[test]
        pub fn should_be_not_longer_than_65_bytes() {
            let (mut raw_state_transition, validator) = setup_test(None, None);
            raw_state_transition.set_key_value("signature", vec![66; 0]);

            let result = validator.validate(&raw_state_transition);

            assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = result.errors().first().unwrap();

            assert_eq!(error.instancePath, "/signature");
            assert_eq!(error.keyword(), "maxItems");
        }
    }

    #[test]
    pub fn should_return_valid_result() {
        let (mut raw_state_transition, validator) = setup_test(None, None);
        let result = validator.validate(&raw_state_transition);

        assert!(result.isValid());

        assert_eq!(validatePublicKeysMock, raw_state_transition.publicKeys);
    }
}
