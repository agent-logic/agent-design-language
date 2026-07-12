use serde::Serialize;
use strum::{AsRefStr, Display, EnumIter, EnumString};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, V2Error>;

#[derive(Debug, Clone, Copy, Serialize, Display, EnumString, AsRefStr, EnumIter)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorCode {
    Io,
    InvalidInput,
    InvalidTransition,
    StaleGeneration,
    StaleDigest,
    MissingClaim,
    ExpiredClaim,
    FieldOwnership,
    CardInvalid,
    CorruptRecord,
    InterruptedTransaction,
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct V2Error {
    pub code: ErrorCode,
    pub message: String,
}

impl V2Error {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for V2Error {
    fn from(value: std::io::Error) -> Self {
        Self::new(ErrorCode::Io, value.to_string())
    }
}

impl From<serde_json::Error> for V2Error {
    fn from(value: serde_json::Error) -> Self {
        Self::new(ErrorCode::CorruptRecord, value.to_string())
    }
}
