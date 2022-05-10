use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    code: u16,
    message: String,
}

impl<'a> From<(u16, &'a str)> for ApiError {
    fn from((code, message): (u16, &'a str)) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}
