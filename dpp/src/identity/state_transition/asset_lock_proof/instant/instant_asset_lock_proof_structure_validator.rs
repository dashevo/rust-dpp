use crate::validation::JsonSchemaValidator;
use crate::DashPlatformProtocolInitError;
use dashcore::InstantLock;
use lazy_static::lazy_static;
use serde_json::Value;

lazy_static! {
    static ref INSTANT_ASSET_LOCK_PROOF_SCHEMA: Value = serde_json::from_str(include_str!(
        "../../../../../schema/identity/stateTransition/assetLockProof/instantAssetLockProof.json"
    ))
    .unwrap();
}

pub struct InstantAssetLockProofStructureValidator<SR> {
    json_schema_validator: JsonSchemaValidator,
    state_repository: SR,
    asset_lock_transaction_validator: Something,
}

impl<SR> InstantAssetLockProofStructureValidator<SR> {
    pub fn new(
        state_repository: SR,
        asset_lock_transaction_validator: Something,
    ) -> Result<Self, DashPlatformProtocolInitError> {
        let json_schema_validator =
            JsonSchemaValidator::new(INSTANT_ASSET_LOCK_PROOF_SCHEMA.clone())?;

        Ok(Self {
            json_schema_validator,
            state_repository,
            asset_lock_transaction_validator,
        })
    }

    pub fn validate(rawAssetLockProof: &Value) {
        // let result = jsonSchemaValidator.validate(
        // instantAssetLockProofSchema,
        // convertBuffersToArrays(rawAssetLockProof),
        // );
        //
        // if !result.isValid() {
        // return result;
        // }
        //
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
    }
}
