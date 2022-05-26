use dashcore::consensus::Decodable;
use dashcore::consensus::encode::Error;
use crate::validation::{JsonSchemaValidator, ValidationResult};
use crate::{DashPlatformProtocolInitError, NonConsensusError};
use dashcore::InstantLock;
use lazy_static::lazy_static;
use serde_json::Value;
use crate::identity::state_transition::asset_lock_proof::{AssetLockTransactionResultData, AssetLockTransactionValidator, InstantAssetLockProof};

lazy_static! {
    static ref INSTANT_ASSET_LOCK_PROOF_SCHEMA: Value = serde_json::from_str(include_str!(
        "../../../../../schema/identity/stateTransition/assetLockProof/instantAssetLockProof.json"
    ))
    .unwrap();
}

pub type PublicKeyHash = [u8; 20];

pub struct InstantAssetLockProofStructureValidator<SR> {
    json_schema_validator: JsonSchemaValidator,
    state_repository: SR,
    asset_lock_transaction_validator: AssetLockTransactionValidator<AssetLockTransactionResultData>,
}

impl<SR> InstantAssetLockProofStructureValidator<SR> {
    pub fn new(
        state_repository: SR,
        asset_lock_transaction_validator: AssetLockTransactionValidator<AssetLockTransactionResultData>,
    ) -> Result<Self, DashPlatformProtocolInitError> {
        let json_schema_validator =
            JsonSchemaValidator::new(INSTANT_ASSET_LOCK_PROOF_SCHEMA.clone())?;

        Ok(Self {
            json_schema_validator,
            state_repository,
            asset_lock_transaction_validator,
        })
    }

    pub fn validate(&self, rawAssetLockProof: &Value) -> Result<ValidationResult<PublicKeyHash>, NonConsensusError> {
        // let result = jsonSchemaValidator.validate(
        // instantAssetLockProofSchema,
        // convertBuffersToArrays(rawAssetLockProof),
        // );
        //
        let mut result = ValidationResult::default();
        result.merge(self.json_schema_validator.validate(rawAssetLockProof)?);

        // if !result.isValid() {
        // return result;
        // }

        if !result.is_valid() {
            return Ok(result);
        }

        let asset_lock_proof: InstantAssetLockProof = serde_json::from_value(rawAssetLockProof.clone())?;

        let instant_lock_validation_result = match InstantLock::consensus_decode(asset_lock_proof.instant_lock()) {
            Ok(instant_lock) => {
                let mut res = ValidationResult::default();
                res.set_data(instant_lock);
                res
            }
            Err(error) => {

            }
        };

        result.merge(instant_lock_validation_result);

        if !result.is_valid() {
            return Ok(result);
        }

        // let instantLock;
        // try {
        // instantLock = InstantLock.fromBuffer(rawAssetLockProof.instantLock);
        // } catch (e) {
        // let error = new InvalidInstantAssetLockProofError(e.message);
        //
        // error.setValidationError(e);
        //
        // result.addError(error);
        //
        // return result;
        // }
        //
        // if (!await stateRepository.verifyInstantLock(instantLock)) {
        // result.addError(new InvalidInstantAssetLockProofSignatureError());
        //
        // return result;
        // }
        //
        // let validateAssetLockTransactionResult = await validateAssetLockTransaction(
        // rawAssetLockProof.transaction,
        // rawAssetLockProof.outputIndex,
        // );
        //
        // result.merge(validateAssetLockTransactionResult);
        //
        // if (!result.isValid()) {
        // return result;
        // }
        //
        // /**
        //  * @typedef {Transaction} transaction
        //  * @typedef {Buffer} publicKeyHash
        //  */
        // let { publicKeyHash, transaction } = validateAssetLockTransactionResult.getData();
        //
        // if (instantLock.txid !== transaction.id) {
        // result.addError(
        // new IdentityAssetLockProofLockedTransactionMismatchError(
        // Buffer.from(instantLock.txid, 'hex'),
        // Buffer.from(transaction.id, 'hex'),
        // ),
        // );
        //
        // return result;
        // }
        //
        // result.setData(publicKeyHash);
        //
        // return result;

        Ok(result)
    }
}
