use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

// NOTE: `IndexedError` must come before `ErrorMessage` so
// the `serde(untagged)` machinery does not greedily match it first.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiError {
    IndexedError {
        #[serde(with = "crate::serde::as_u16")]
        code: StatusCode,
        message: String,
        failures: Vec<IndexedError>,
    },
    ErrorMessage {
        #[serde(with = "crate::serde::as_u16")]
        code: StatusCode,
        message: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexedError {
    index: usize,
    message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorMessage { message, .. } => {
                write!(f, "{message}")
            }
            Self::IndexedError { message, failures, .. } => {
                write!(f, "{message}: ")?;
                for failure in failures {
                    write!(f, "{failure:?}, ")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for ApiError {}

impl<'a> TryFrom<(u16, &'a str)> for ApiError {
    type Error = http::status::InvalidStatusCode;

    fn try_from((code, message): (u16, &'a str)) -> Result<Self, Self::Error> {
        let code = StatusCode::from_u16(code)?;
        Ok(Self::ErrorMessage { code, message: message.to_string() })
    }
}
