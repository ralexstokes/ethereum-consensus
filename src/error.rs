use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    #[serde(with = "crate::serde::as_u16")]
    pub code: StatusCode,
    pub message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ApiError {}

impl<'a> TryFrom<(u16, &'a str)> for ApiError {
    type Error = http::status::InvalidStatusCode;

    fn try_from((code, message): (u16, &'a str)) -> Result<Self, Self::Error> {
        let code = StatusCode::from_u16(code)?;
        Ok(Self {
            code,
            message: message.to_string(),
        })
    }
}
