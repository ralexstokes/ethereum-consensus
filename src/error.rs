use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

mod as_u16 {
    use http::StatusCode;
    use serde::{de::Deserializer, Deserialize, Serializer};

    pub fn serialize<S>(x: &StatusCode, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_u16(x.as_u16())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: u16 = Deserialize::deserialize(deserializer)?;
        StatusCode::from_u16(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    #[serde(with = "as_u16")]
    pub code: StatusCode,
    pub message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "status {}: {}", self.code, self.message)
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
