use crate::JceType;
use std::{error::Error, fmt::Display};

/// Results
pub type JceResult<T> = Result<T, JceError>;

/// Errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JceError {
    /// Jce tag error
    ReadTagError(u8, u8),
    /// Jce type error
    ReadTypeError(JceType, JceType),
    /// Jce len
    ReadLenError(JceType),
    /// Read Utf8 error
    Utf8Error,
    /// Tag not found
    TagNotFound(u8),
    /// Jce read error
    ReadError(&'static str),
    /// Jce write error
    WriteError(&'static str),
}

impl Display for JceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadTagError(etag, gtag) => write!(
                f,
                "Jce read tag error, expected tag: {}, get tag: {}",
                etag, gtag
            ),
            Self::ReadTypeError(ety, gty) => write!(
                f,
                "Jce read type error, expected type: {}, get type: {}",
                ety, gty
            ),
            Self::ReadLenError(ty) => write!(f, "Jce read len error, get type: {}", ty),
            Self::Utf8Error => write!(f, "Jce read utf8 error"),
            Self::TagNotFound(tag) => write!(f, "Jce tag not found, tag: {}", tag),
            Self::ReadError(s) => write!(f, "Jce read error: {}", s),
            Self::WriteError(s) => write!(f, "Jce write error: {}", s),
        }
    }
}

impl Error for JceError {}
