use derive_more::From;
use pest::error::{Error as PestError, ErrorVariant};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // Internal Errors
    #[from]
    CustomError {
        message: String,
    },

    // TODO: Fix this -- Pest uses PestError<T>
    ParseError(),
}

impl Error {
    pub fn custom(msg: impl core::fmt::Display) -> Self {
        Self::CustomError {
            message: msg.to_string(),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::CustomError {
            message: value.to_string(),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for Error {}
