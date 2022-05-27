use super::{
    abstract_state_error::StateError, consensus::basic::BasicError, consensus::ConsensusError,
    DataTriggerError,
};
pub trait ErrorWithCode {
    // Returns the Error Code
    fn get_code(&self) -> u32;
}

impl ErrorWithCode for ConsensusError {
    fn get_code(&self) -> u32 {
        match self {
            // Decoding
            Self::ProtocolVersionParsingError { .. } => 1000,
            Self::SerializedObjectParsingError { .. } => 1001,

            Self::JsonSchemaError(_) => 1005,
            Self::UnsupportedProtocolVersionError(_) => 1002,
            Self::IncompatibleProtocolVersionError(_) => 1003,

            // Identity
            Self::DuplicatedIdentityPublicKeyError(_) => 1029,
            Self::DuplicatedIdentityPublicKeyIdError(_) => 1030,
            Self::IdentityAssetLockProofLockedTransactionMismatchError(_) => 1031,
            Self::IdentityAssetLockTransactionOutPointAlreadyExistsError(_) => 1033,
            Self::IdentityAssetLockTransactionOutputNotFoundError(_) => 1034,
            Self::InvalidAssetLockTransactionOutputReturnSize(_) => 1037,
            Self::InvalidIdentityAssetLockTransactionError(_) => 1038,
            Self::InvalidIdentityAssetLockTransactionOutputError(_) => 1039,
            Self::InvalidIdentityPublicKeyDataError(_) => 1040,
            Self::InvalidInstantAssetLockProofError(_) => 1041,
            Self::InvalidInstantAssetLockProofSignatureError(_) => 1042,
            Self::MissingMasterPublicKeyError(_) => 1046,
            Self::InvalidIdentityPublicKeySecurityLevelError(_) => 1047,

            Self::StateError(e) => e.get_code(),
            Self::BasicError(e) => e.get_code(),

            #[cfg(test)]
            ConsensusError::TestConsensusError(_) => 1000,
        }
    }
}

impl ErrorWithCode for StateError {
    fn get_code(&self) -> u32 {
        match *self {
            // Document
            Self::DocumentAlreadyPresentError { .. } => 4004,
            Self::DocumentNotFoundError { .. } => 4005,
            Self::DocumentOwnerMismatchError { .. } => 4006,
            Self::DocumentTimestampMismatchError { .. } => 4007,
            Self::DocumentTimestampWindowViolationError { .. } => 4008,
            Self::DuplicateUniqueIndexError { .. } => 4009,
            Self::InvalidDocumentRevisionError { .. } => 4010,
            // Data contract
            Self::DataContractAlreadyPresentError { .. } => 4000,
            Self::DataTriggerError(ref e) => e.get_code(),
        }
    }
}

impl ErrorWithCode for DataTriggerError {
    fn get_code(&self) -> u32 {
        match *self {
            // Data Contract - Data Trigger
            Self::DataTriggerConditionError { .. } => 4001,
            Self::DataTriggerExecutionError { .. } => 4002,
            Self::DataTriggerInvalidResultError { .. } => 4003,
        }
    }
}

impl ErrorWithCode for BasicError {
    fn get_code(&self) -> u32 {
        match *self {
            // Document
            Self::DataContractContPresent { .. } => 1018,
            // Data contract
            Self::InvalidDataContractVersionError { .. } => 4013,
        }
    }
}
