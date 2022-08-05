use dashcore::consensus;
use dashcore::InstantLock;
use lazy_static::lazy_static;
use serde_json::Value;
use std::sync::Arc;

use crate::consensus::basic::identity::{
    IdentityAssetLockProofLockedTransactionMismatchError, InvalidInstantAssetLockProofError,
    InvalidInstantAssetLockProofSignatureError,
};
use crate::identity::state_transition::asset_lock_proof::AssetLockTransactionValidator;
use crate::state_repository::StateRepositoryLike;
use crate::util::json_value::JsonValueExt;
use crate::validation::{JsonSchemaValidator, ValidationResult};
use crate::{DashPlatformProtocolInitError, NonConsensusError, SerdeParsingError};

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
    state_repository: Arc<SR>,
    asset_lock_transaction_validator: AssetLockTransactionValidator<SR>,
}

impl<SR> InstantAssetLockProofStructureValidator<SR>
where
    SR: StateRepositoryLike,
{
    pub fn new(
        state_repository: Arc<SR>,
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

    pub async fn validate(
        &self,
        raw_asset_lock_proof: &Value,
    ) -> Result<ValidationResult<PublicKeyHash>, NonConsensusError> {
        // let result = jsonSchemaValidator.validate(
        // instantAssetLockProofSchema,
        // convertBuffersToArrays(raw_asset_lock_proof),
        // );
        //
        let mut result = ValidationResult::default();
        result.merge(self.json_schema_validator.validate(raw_asset_lock_proof)?);

        // if !result.isValid() {
        // return result;
        // }

        if !result.is_valid() {
            return Ok(result);
        }

        // let asset_lock_proof: InstantAssetLockProof = serde_json::from_value(raw_asset_lock_proof.clone())?;

        // Is lock should go there
        let raw_is_lock: Vec<u8> = raw_asset_lock_proof
            .as_object()
            .ok_or_else(|| SerdeParsingError::new("Expected raw asset lock proof to be an object"))?
            .get("instantLock")
            .ok_or_else(|| {
                SerdeParsingError::new("Expected raw asset lock to have property 'instantLock'")
            })?
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

        let instant_lock = instant_lock_validation_result.data().unwrap().clone();

        result.merge(instant_lock_validation_result);

        if !result.is_valid() {
            return Ok(result);
        }

        // let instantLock;
        // try {
        // instantLock = InstantLock.fromBuffer(raw_asset_lock_proof.instantLock);
        // } catch (e) {
        // let error = new InvalidInstantAssetLockProofError(e.message);
        //
        // error.setValidationError(e);
        //
        // result.addError(error);
        //
        // return result;
        // }

        let is_signature_verified = self
            .state_repository
            .verify_instant_lock(&instant_lock)
            .await
            .map_err(|err| NonConsensusError::StateRepositoryFetchError(err.to_string()))?;

        if !is_signature_verified {
            result.add_error(InvalidInstantAssetLockProofSignatureError::new());
            return Ok(result);
        }

        // if (!await stateRepository.verifyInstantLock(instantLock)) {
        // result.addError(new InvalidInstantAssetLockProofSignatureError());
        //
        // return result;
        // }

        let tx_json_uint_array = raw_asset_lock_proof
            .get_bytes("transaction")
            .map_err(|err| SerdeParsingError::new(err.to_string()))?;

        let output_index = raw_asset_lock_proof
            .as_object()
            .ok_or_else(|| SerdeParsingError::new("Expected asset lock to be an object"))?
            .get("outputIndex")
            .ok_or_else(|| {
                SerdeParsingError::new("Expect asset lock to have a 'transaction field'")
            })?
            .as_u64()
            .ok_or_else(|| SerdeParsingError::new("Expect outputIndex to be a number"))?;

        // TODO: get transaction bytes and pass them as the first argument
        let validate_asset_lock_transaction_result = self
            .asset_lock_transaction_validator
            .validate(&tx_json_uint_array, output_index as usize)
            .await?;

        // TODO: remove unwrap
        let validation_result_data = validate_asset_lock_transaction_result
            .data()
            .unwrap()
            .clone();
        result.merge(validate_asset_lock_transaction_result);

        if !result.is_valid() {
            return Ok(result);
        }

        // let validate_asset_lock_transaction_result = await validateAssetLockTransaction(
        // raw_asset_lock_proof.transaction,
        // raw_asset_lock_proof.outputIndex,
        // );
        //
        // result.merge(validate_asset_lock_transaction_result);
        //
        // if (!result.isValid()) {
        // return result;
        // }

        let public_key_hash = validation_result_data.public_key_hash;
        let transaction = &validation_result_data.transaction;

        // /**
        //  * @typedef {Transaction} transaction
        //  * @typedef {Buffer} publicKeyHash
        //  */
        // let { publicKeyHash, transaction } = validate_asset_lock_transaction_result.getData();
        //

        if instant_lock.txid != transaction.txid() {
            result.add_error(IdentityAssetLockProofLockedTransactionMismatchError::new(
                instant_lock.txid,
                transaction.txid(),
            ));

            return Ok(result);
        }

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

        result.set_data(public_key_hash);

        Ok(result)

        // result.setData(publicKeyHash);
        //
        // return result;
    }
}
