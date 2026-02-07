use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(io::Error),
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Invalid codec")]
    InvalidCodec,
    #[error("Unsupported operation")]
    UnsupportedOperation,
    #[error("Decode error: {0}")]
    DecodeError(String),
    #[error("Encode error: {0}")]
    EncodeError(String),
    #[error("End of stream")]
    EndOfStream,
    #[error("Custom error: {0}")]
    Custom(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = std::result::Result<T, Error>;
