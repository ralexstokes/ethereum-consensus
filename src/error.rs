use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "status {}: {}", self.code, self.message)
    }
}

impl Error for ApiError {}

impl<'a> From<(u16, &'a str)> for ApiError {
    fn from((code, message): (u16, &'a str)) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}
