use crate::consensus::basic::identity::{
    IdentityAssetLockTransactionOutputNotFoundError, InvalidAssetLockTransactionOutputReturnSize,
    InvalidIdentityAssetLockTransactionError, InvalidIdentityAssetLockTransactionOutputError,
};
use crate::tests::utils::vec_to_array;
use crate::validation::ValidationResult;
use crate::NonConsensusError;
use dashcore::consensus::encode::Error;
use dashcore::consensus::{Decodable, Encodable};
use dashcore::psbt::serialize::Deserialize;
use dashcore::{OutPoint, Transaction};

pub struct AssetLockTransactionResultData {
    public_key_hash: [u8; 20],
    transaction: Transaction,
}

pub struct AssetLockTransactionValidator<SR> {
    state_repository: SR,
}

impl<SR> AssetLockTransactionValidator<SR> {
    pub fn new(state_repository: SR) -> Self {
        Self { state_repository }
    }

    pub fn validate(
        &self,
        raw_tx: &[u8],
        output_index: usize,
    ) -> Result<ValidationResult<AssetLockTransactionResultData>, NonConsensusError> {
        let mut result = ValidationResult::default();

        match Transaction::consensus_decode(raw_tx) {
            Ok(transaction) => {
                let output = transaction.output.get(output_index);

                if let Some(output) = output {
                    if !output.script_pubkey.is_op_return() {
                        result.add_error(InvalidIdentityAssetLockTransactionOutputError::new(
                            output_index,
                        ));
                        return Ok(result);
                    }

                    // Slicing from 1 bytes, which is OP_RETURN, to the end of the script
                    let public_key_hash = output.script_pubkey.as_bytes()[1..];
                    // 20 bytes is the size of ripemd160, which should be stored after the OP_RETURN
                    if public_key_hash.len() != 20 {
                        result.add_error(InvalidAssetLockTransactionOutputReturnSize::new(
                            output_index,
                        ));
                        return Ok(result);
                    }

                    let out_point_buf =
                        OutPoint::new(transaction.txid(), output_index as u32).consensus_encode();

                    let is_out_point_used = self
                        .state_repository
                        .is_asset_lock_transaction_out_point_already_used(out_point_buf);
                    if is_out_point_used {
                        result.add_error(
                            IdentityAssetLockTransactionOutPointAlreadyExistsError::new(
                                transaction.txid(),
                                output_index,
                            ),
                        );
                        return Ok(result);
                    }

                    result.set_data(AssetLockTransactionResultData {
                        public_key_hash: vec_to_array(&output.script_pubkey.as_bytes()[1..21])?,
                        transaction,
                    });

                    Ok(result)
                } else {
                    result.add_error(IdentityAssetLockTransactionOutputNotFoundError::new(
                        output_index,
                    ));
                    return Ok(result);
                }
            }
            Err(err) => {
                let mut error = InvalidIdentityAssetLockTransactionError::new(err.to_string());
                err.set_validation_error(err);

                result.add_error(error);
                return Ok(result);
            }
        }
    }
}
