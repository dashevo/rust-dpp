use crate::consensus::basic::identity::{
    DuplicatedIdentityPublicKeyError, DuplicatedIdentityPublicKeyIdError,
    InvalidIdentityPublicKeyDataError, InvalidIdentityPublicKeySecurityLevelError,
    MissingMasterPublicKeyError,
};
use crate::errors::consensus::basic::{
    IncompatibleProtocolVersionError, JsonSchemaError, UnsupportedProtocolVersionError,
};
use jsonschema::ValidationError;
use thiserror::Error;

#[cfg(test)]
use crate::errors::consensus::basic::TestConsensusError;

#[derive(Error, Debug)]
#[cfg_attr(test, derive(Clone))]
#[error("{0}")]
pub enum ConsensusError {
    #[error("{0}")]
    JsonSchemaError(JsonSchemaError),
    #[error("{0}")]
    UnsupportedProtocolVersionError(UnsupportedProtocolVersionError),
    #[error("{0}")]
    IncompatibleProtocolVersionError(IncompatibleProtocolVersionError),
    #[error("{0}")]
    DuplicatedIdentityPublicKeyIdError(DuplicatedIdentityPublicKeyIdError),
    #[error("{0}")]
    InvalidIdentityPublicKeyDataError(InvalidIdentityPublicKeyDataError),
    #[error("{0}")]
    InvalidIdentityPublicKeySecurityLevelError(InvalidIdentityPublicKeySecurityLevelError),
    #[error("{0}")]
    DuplicatedIdentityPublicKeyError(DuplicatedIdentityPublicKeyError),
    #[error("{0}")]
    MissingMasterPublicKeyError(MissingMasterPublicKeyError),
    #[cfg(test)]
    TestConsensusError(TestConsensusError),
}

impl ConsensusError {
    pub fn json_schema_error(&self) -> Option<&JsonSchemaError> {
        match self {
            ConsensusError::JsonSchemaError(err) => Some(err),
            _ => None,
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            ConsensusError::JsonSchemaError(_) => 1005,
            ConsensusError::UnsupportedProtocolVersionError(_) => 1002,
            ConsensusError::IncompatibleProtocolVersionError(_) => 1003,

            // Identity
            ConsensusError::DuplicatedIdentityPublicKeyError(_) => 1029,
            ConsensusError::DuplicatedIdentityPublicKeyIdError(_) => 1030,
            ConsensusError::InvalidIdentityPublicKeyDataError(_) => 1040,
            ConsensusError::MissingMasterPublicKeyError(_) => 1046,
            ConsensusError::InvalidIdentityPublicKeySecurityLevelError(_) => 1047,

            // Custom error for tests
            #[cfg(test)]
            ConsensusError::TestConsensusError(_) => 1000,
        }
    }
}

impl<'a> From<ValidationError<'a>> for ConsensusError {
    fn from(validation_error: ValidationError<'a>) -> Self {
        Self::JsonSchemaError(JsonSchemaError::from(validation_error))
    }
}

impl From<JsonSchemaError> for ConsensusError {
    fn from(json_schema_error: JsonSchemaError) -> Self {
        Self::JsonSchemaError(json_schema_error)
    }
}

impl From<UnsupportedProtocolVersionError> for ConsensusError {
    fn from(error: UnsupportedProtocolVersionError) -> Self {
        Self::UnsupportedProtocolVersionError(error)
    }
}

impl From<IncompatibleProtocolVersionError> for ConsensusError {
    fn from(error: IncompatibleProtocolVersionError) -> Self {
        Self::IncompatibleProtocolVersionError(error)
    }
}

impl From<DuplicatedIdentityPublicKeyIdError> for ConsensusError {
    fn from(error: DuplicatedIdentityPublicKeyIdError) -> Self {
        Self::DuplicatedIdentityPublicKeyIdError(error)
    }
}

impl From<InvalidIdentityPublicKeyDataError> for ConsensusError {
    fn from(error: InvalidIdentityPublicKeyDataError) -> Self {
        Self::InvalidIdentityPublicKeyDataError(error)
    }
}

impl From<InvalidIdentityPublicKeySecurityLevelError> for ConsensusError {
    fn from(error: InvalidIdentityPublicKeySecurityLevelError) -> Self {
        Self::InvalidIdentityPublicKeySecurityLevelError(error)
    }
}

impl From<DuplicatedIdentityPublicKeyError> for ConsensusError {
    fn from(error: DuplicatedIdentityPublicKeyError) -> Self {
        Self::DuplicatedIdentityPublicKeyError(error)
    }
}

impl From<MissingMasterPublicKeyError> for ConsensusError {
    fn from(error: MissingMasterPublicKeyError) -> Self {
        Self::MissingMasterPublicKeyError(error)
    }
}

#[cfg(test)]
impl From<TestConsensusError> for ConsensusError {
    fn from(error: TestConsensusError) -> Self {
        Self::TestConsensusError(error)
    }
}
