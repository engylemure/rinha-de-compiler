use crate::{ast::Location, parse::ParseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error when parsing json")]
    ParseJsonError(serde_json::Error),
    #[error("IO Error")]
    IOError(std::io::Error),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseJsonError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}
