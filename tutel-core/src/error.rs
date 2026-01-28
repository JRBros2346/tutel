use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("unsupported codec")]
    Unsupported,
    #[error("invalid codec data")]
    InvalidData,
    #[error("decoder initialization failed")]
    InitFailed,
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("unsupported container format")]
    Unsupported,
    #[error("invalid or corrupted container")]
    InvalidData,
    #[error("missing required stream")]
    MissingStream,
    #[error("{0}")]
    Other(String),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("codec error: {0}")]
    Codec(#[from] CodecError),
    #[error("format error: {0}")]
    Format(#[from] FormatError),
    #[error("internal error: {0}")]
    Internal(String),
}
