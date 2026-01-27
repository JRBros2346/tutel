#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(
        #[source]
        #[from]
        std::io::Error,
    ),
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Invalid codec")]
    InvalidCodec,
    #[error("Decode error: {0}")]
    DecodeError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Encode error: {0}")]
    EncodeError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Custom error: {0}")]
    Custom(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = std::result::Result<T, Error>;
