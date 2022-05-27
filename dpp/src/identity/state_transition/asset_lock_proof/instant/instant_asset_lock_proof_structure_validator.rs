use crate::consensus::basic::identity::InvalidInstantAssetLockProofError;
use crate::identity::state_transition::asset_lock_proof::{
    AssetLockTransactionValidator,
};
use crate::state_repository::StateRepositoryLike;
use crate::validation::{JsonSchemaValidator, ValidationResult};
use crate::{DashPlatformProtocolInitError, NonConsensusError, SerdeParsingError};
use dashcore::consensus;
use dashcore::InstantLock;
use lazy_static::lazy_static;
use serde_json::Value;

lazy_static! {
    static ref INSTANT_ASSET_LOCK_PROOF_SCHEMA: Value = serde_json::from_str(include_str!(
        "../../../../schema/identity/stateTransition/assetLockProof/instantAssetLockProof.json"
    ))
    .unwrap();
}

pub type PublicKeyHash = [u8; 20];

pub struct InstantAssetLockProofStructureValidator<SR>
where
    SR: StateRepositoryLike,
{
    json_schema_validator: JsonSchemaValidator,
    state_repository: SR,
    asset_lock_transaction_validator: AssetLockTransactionValidator<SR>,
}

impl<SR> InstantAssetLockProofStructureValidator<SR>
where
    SR: StateRepositoryLike,
{
    pub fn new(
        state_repository: SR,
        asset_lock_transaction_validator: AssetLockTransactionValidator<SR>,
    ) -> Result<Self, DashPlatformProtocolInitError> {
        let json_schema_validator =
            JsonSchemaValidator::new(INSTANT_ASSET_LOCK_PROOF_SCHEMA.clone())?;

        Ok(Self {
            json_schema_validator,
            state_repository,
            asset_lock_transaction_validator,
        })
    }

    pub fn validate(
        &self,
        rawAssetLockProof: &Value,
    ) -> Result<ValidationResult<PublicKeyHash>, NonConsensusError> {
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

        // let asset_lock_proof: InstantAssetLockProof = serde_json::from_value(rawAssetLockProof.clone())?;

        // Is lock should go there
        let raw_is_lock: Vec<u8> = rawAssetLockProof
            .as_object()
            .ok_or_else(|| SerdeParsingError::new("Expected raw asset lock proof to be an object"))?
            .get("instantLock")
            .ok_or_else(|| SerdeParsingError::new("Expected raw asset lock to have property 'instantLock'"))?
            .as_array()
            .ok_or_else(|| SerdeParsingError::new("Expected 'instantLock' to be an array"))?
            .iter()
            // TODO: remove unwrap
            .map(|val| val.as_u64().unwrap() as u8)
            .collect();
        //let is_lock = consensus::deserialize::<InstantLock>(&raw_is_lock);

        let instant_lock_validation_result =
            match consensus::deserialize::<InstantLock>(&raw_is_lock) {
                Ok(instant_lock) => {
                    let mut res = ValidationResult::default();
                    res.set_data(instant_lock);
                    res
                }
                Err(error) => {
                    let mut res = ValidationResult::default();
                    let err = InvalidInstantAssetLockProofError::new(error.to_string());
                    res.add_error(err);
                    res
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
