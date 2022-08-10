use crate::version::{COMPATIBILITY_MAP, LATEST_VERSION, ProtocolVersionValidator};

pub fn get_protocol_version_validator_fixture() -> ProtocolVersionValidator {
    ProtocolVersionValidator::new(LATEST_VERSION, LATEST_VERSION, COMPATIBILITY_MAP.clone())
}
