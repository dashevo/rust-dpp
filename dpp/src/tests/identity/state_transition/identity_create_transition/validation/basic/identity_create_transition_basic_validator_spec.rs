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

use std::sync::Arc;

use crate::identity::state_transition::asset_lock_proof::{
    AssetLockProofValidator, AssetLockTransactionValidator, InstantAssetLockProofStructureValidator,
};
use serde_json::Value;

use crate::identity::state_transition::identity_create_transition::validation::basic::IdentityCreateTransitionBasicValidator;
use crate::identity::validation::TPublicKeysValidator;
use crate::state_repository::MockStateRepositoryLike;
use crate::version::ProtocolVersionValidator;

pub fn setup_test(
    public_keys_validator: Arc<impl TPublicKeysValidator>,
    public_keys_transition_validator: Arc<impl TPublicKeysValidator>,
    state_repository_mock: MockStateRepositoryLike
) -> (
    Value,
    IdentityCreateTransitionBasicValidator<
        impl TPublicKeysValidator,
        impl TPublicKeysValidator,
        MockStateRepositoryLike,
    >,
) {
    let state_repository = Arc::new(state_repository_mock);
    let asset_lock_transaction_validator =
        AssetLockTransactionValidator::new(state_repository.clone());
    let instant_asset_lock_validator = InstantAssetLockProofStructureValidator::new(
        state_repository,
        asset_lock_transaction_validator,
    )
    .unwrap();
    let asset_lock_proof_validator =
        Arc::new(AssetLockProofValidator::new(instant_asset_lock_validator));

    let protocol_version_validator = ProtocolVersionValidator::default();
    (
        // TODO: should it really be None?
        crate::tests::fixtures::identity_create_transition_fixture_json(None),
        IdentityCreateTransitionBasicValidator::new(
            Arc::new(protocol_version_validator),
            public_keys_validator,
            public_keys_transition_validator,
            asset_lock_proof_validator,
        )
        .unwrap(),
    )
}

mod validate_identity_create_transition_basic_factory {
    use std::sync::Arc;

    use crate::identity::validation::PublicKeysInIdentityCreateTransitionValidator;
    use crate::tests::fixtures::PublicKeysValidatorMock;
    use crate::tests::utils::SerdeTestExtension;
    use crate::validation::ValidationResult;
    use crate::state_repository::MockStateRepositoryLike;

    pub use super::setup_test;

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
        use std::sync::Arc;

        use jsonschema::error::ValidationErrorKind;

        use super::setup_test;
        use crate::consensus::ConsensusError;
        use crate::identity::validation::{
            PublicKeysInIdentityCreateTransitionValidator, PublicKeysValidator,
        };
        use crate::tests::utils::SerdeTestExtension;
        use crate::{assert_consensus_errors, NonConsensusError};
        use crate::state_repository::MockStateRepositoryLike;

        #[tokio::test]
        pub async fn should_be_present() {
            let state_repository = MockStateRepositoryLike::new();
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                state_repository
            );
            raw_state_transition.remove_key("protocolVersion");

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "");
            assert_eq!(error.keyword().unwrap(), "required");
            match error.kind() {
                ValidationErrorKind::Required { property } => {
                    assert_eq!(property.to_string(), "\"protocolVersion\"");
                }
                _ => panic!("Expected to be missing property"),
            }
        }

        #[tokio::test]
        pub async fn should_be_an_integer() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.set_key_value("protocolVersion", "1");

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "/protocolVersion");
            assert_eq!(error.keyword().unwrap(), "type");
        }

        #[tokio::test]
        pub async fn should_be_valid() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.set_key_value("protocolVersion", -1);

            let result = validator.validate(&raw_state_transition).await;

            match result {
                Ok(_) => {
                    panic!("Expected error");
                }
                Err(e) => match e {
                    NonConsensusError::SerdeParsingError(e) => {
                        assert_eq!(e.message(), "Expected protocolVersion to be a uint");
                    }
                    _ => {
                        panic!("Expected version parsing error");
                    }
                },
            }
        }
    }

    mod type_a {
        use std::sync::Arc;

        use jsonschema::error::ValidationErrorKind;

        use crate::assert_consensus_errors;
        use crate::consensus::ConsensusError;
        use crate::identity::validation::{
            PublicKeysInIdentityCreateTransitionValidator, PublicKeysValidator,
        };
        use crate::tests::utils::SerdeTestExtension;
        use crate::state_repository::MockStateRepositoryLike;

        use super::super::setup_test;

        #[tokio::test]
        pub async fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.remove_key("type");
            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "");
            assert_eq!(error.keyword().unwrap(), "required");

            match error.kind() {
                ValidationErrorKind::Required { property } => {
                    assert_eq!(property.to_string(), "\"type\"");
                }
                _ => panic!("Expected to be missing property"),
            }
        }

        #[tokio::test]
        pub async fn should_be_equal_to_2() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.set_key_value("type", 666);

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "/type");
            assert_eq!(error.keyword().unwrap(), "const");

            println!("{:?}", error.kind());
            match error.kind() {
                ValidationErrorKind::Constant { expected_value } => {
                    assert_eq!(expected_value.as_u64().unwrap(), 2u64);
                }
                _ => panic!("Expected to have a constant value"),
            }
        }
    }

    mod asset_lock_proof {
        use std::sync::Arc;

        use jsonschema::error::ValidationErrorKind;

        use crate::assert_consensus_errors;
        use crate::consensus::basic::TestConsensusError;
        use crate::consensus::ConsensusError;
        use crate::identity::validation::{
            PublicKeysInIdentityCreateTransitionValidator, PublicKeysValidator,
        };
        use crate::tests::utils::SerdeTestExtension;
        use crate::state_repository::MockStateRepositoryLike;

        use super::super::setup_test;

        #[tokio::test]
        pub async fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.remove_key("assetLockProof");

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "");
            assert_eq!(error.keyword().unwrap(), "required");

            match error.kind() {
                ValidationErrorKind::Required { property } => {
                    assert_eq!(property.to_string(), "\"assetLockProof\"");
                }
                _ => panic!("Expected to be missing property"),
            }
        }

        #[tokio::test]
        pub async fn should_be_an_object() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.set_key_value("assetLockProof", 1);

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "/assetLockProof");
            assert_eq!(error.keyword().unwrap(), "type");
        }

        #[tokio::test]
        pub async fn should_be_valid() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            let st_map = raw_state_transition
                .get_mut("assetLockProof")
                .unwrap()
                .as_object_mut()
                .unwrap();
            st_map.insert("version".into(), "totally not a valid type".into());
            let err = TestConsensusError::new("test");
            println!("1");
            //let asset_lock_error = ConsensusError::from(err.clone());
            //let asset_lock_result = ValidationResult::<()>::new(Some(vec![asset_lock_error]));

            // TODO: what to do about that?
            // proofValidationFunctionsByTypeMock[InstantAssetLockProof.type_]
            //     .resolves(asset_lock_result);

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::TestConsensusError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error, &&err);

            // assert_eq!(
            //     proofValidationFunctionsByTypeMock[InstantAssetLockProof.type_],
            //     raw_state_transition.assetLockProof
            // );
        }
    }

    mod public_keys {
        use std::sync::Arc;

        use jsonschema::error::ValidationErrorKind;
        use serde_json::Value;

        use crate::assert_consensus_errors;
        use crate::consensus::basic::TestConsensusError;
        use crate::consensus::ConsensusError;
        use crate::identity::validation::{
            PublicKeysInIdentityCreateTransitionValidator, PublicKeysValidator,
        };
        use crate::tests::fixtures::PublicKeysValidatorMock;
        use crate::tests::utils::SerdeTestExtension;
        use crate::validation::ValidationResult;
        use crate::state_repository::MockStateRepositoryLike;

        use super::super::setup_test;

        #[tokio::test]
        pub async fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.remove_key("publicKeys");

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "");
            assert_eq!(error.keyword().unwrap(), "required");

            match error.kind() {
                ValidationErrorKind::Required { property } => {
                    assert_eq!(property.to_string(), "\"publicKeys\"");
                }
                _ => panic!("Expected to be missing property"),
            }
        }

        #[tokio::test]
        pub async fn should_not_be_empty() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.set_key_value("publicKeys", Vec::<Value>::new());

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.keyword().unwrap(), "minItems");
            assert_eq!(error.instance_path().to_string(), "/publicKeys");
        }

        #[tokio::test]
        pub async fn should_not_have_more_than_10_items() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );

            let public_keys = raw_state_transition
                .get_value_mut("publicKeys")
                .as_array_mut()
                .unwrap();
            let key = public_keys.first().unwrap().clone();

            for _ in 0..10 {
                public_keys.push(key.clone());
            }

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 2);

            let error = errors.first().unwrap();

            assert_eq!(error.keyword().unwrap(), "maxItems");
            assert_eq!(error.instance_path().to_string(), "/publicKeys");
        }

        #[tokio::test]
        pub async fn should_be_unique() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );

            let public_keys = raw_state_transition
                .get_value_mut("publicKeys")
                .as_array_mut()
                .unwrap();
            let key = public_keys.first().unwrap().clone();
            public_keys.push(key.clone());

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.keyword().unwrap(), "uniqueItems");
            assert_eq!(error.instance_path().to_string(), "/publicKeys");
        }

        #[tokio::test]
        pub async fn should_be_valid() {
            let pk_validator_mock = Arc::new(PublicKeysValidatorMock::new());
            let pk_error = TestConsensusError::new("test");
            pk_validator_mock.returns_fun(move || {
                Ok(ValidationResult::new(Some(vec![ConsensusError::from(
                    TestConsensusError::new("test"),
                )])))
            });

            let (raw_state_transition, validator) = setup_test(
                pk_validator_mock.clone(),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::TestConsensusError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error, &&pk_error);

            assert_eq!(
                &pk_validator_mock.called_with(),
                raw_state_transition
                    .get_value("publicKeys")
                    .as_array()
                    .unwrap()
            );
        }

        #[tokio::test]
        pub async fn should_have_at_least_1_master_key() {
            let pk_validator_mock = Arc::new(PublicKeysValidatorMock::new());
            let pk_error = TestConsensusError::new("test");
            pk_validator_mock.returns_fun(move || {
                Ok(ValidationResult::new(Some(vec![ConsensusError::from(
                    TestConsensusError::new("test"),
                )])))
            });

            let (raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                pk_validator_mock.clone(),
                MockStateRepositoryLike::new()
            );

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::TestConsensusError, 1);
            let error = errors.first().unwrap();

            assert_eq!(error, &&pk_error);

            assert_eq!(
                &pk_validator_mock.called_with(),
                raw_state_transition
                    .get_value("publicKeys")
                    .as_array()
                    .unwrap()
            );
        }
    }

    mod signature {
        use std::sync::Arc;

        use jsonschema::error::ValidationErrorKind;

        use crate::assert_consensus_errors;
        use crate::consensus::ConsensusError;
        use crate::identity::validation::{
            PublicKeysInIdentityCreateTransitionValidator, PublicKeysValidator,
        };
        use crate::tests::utils::SerdeTestExtension;
        use crate::state_repository::MockStateRepositoryLike;

        use super::super::setup_test;

        #[tokio::test]
        pub async fn should_be_present() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.remove_key("signature");

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "");
            assert_eq!(error.keyword().unwrap(), "required");

            match error.kind() {
                ValidationErrorKind::Required { property } => {
                    assert_eq!(property.to_string(), "\"signature\"");
                }
                _ => panic!("Expected to be missing property"),
            }
        }

        #[tokio::test]
        pub async fn should_be_a_byte_array() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.set_key_value("signature", vec!["string"; 65]);

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 65);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "/signature/0");
            assert_eq!(error.keyword().unwrap(), "type");
        }

        #[tokio::test]
        pub async fn should_be_not_shorter_than_65_bytes() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.set_key_value("signature", vec![0; 64]);

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            assert_eq!(error.instance_path().to_string(), "/signature");
            assert_eq!(error.keyword().unwrap(), "minItems");
        }

        #[tokio::test]
        pub async fn should_be_not_longer_than_65_bytes() {
            let (mut raw_state_transition, validator) = setup_test(
                Arc::new(PublicKeysValidator::new().unwrap()),
                Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
                MockStateRepositoryLike::new()
            );
            raw_state_transition.set_key_value("signature", vec![0; 66]);

            let result = validator.validate(&raw_state_transition).await.unwrap();

            let errors = assert_consensus_errors!(result, ConsensusError::JsonSchemaError, 1);

            let error = errors.first().unwrap();

            println!("{:?}", error);
            assert_eq!(error.instance_path().to_string(), "/signature");
            assert_eq!(error.keyword().unwrap(), "maxItems");
        }
    }

    #[tokio::test]
    pub async fn should_return_valid_result() {
        let pk_validator_mock = Arc::new(PublicKeysValidatorMock::new());
        pk_validator_mock.returns_fun(move || Ok(ValidationResult::default()));

        let mut state_repository = MockStateRepositoryLike::new();
        state_repository.expect_verify_instant_lock().returning(|_asset_lock| Ok(true));
        state_repository.expect_is_asset_lock_transaction_out_point_already_used().returning(|_asset_lock| Ok(false));

        let (raw_state_transition, validator) = setup_test(
            pk_validator_mock.clone(),
            Arc::new(PublicKeysInIdentityCreateTransitionValidator::default()),
            state_repository
        );
        let result = validator.validate(&raw_state_transition).await.unwrap();

        println!("{:?}", result.errors);

        assert!(result.is_valid());
        assert_eq!(
            &pk_validator_mock.called_with(),
            raw_state_transition
                .get_value("publicKeys")
                .as_array()
                .unwrap()
        );
    }
}
