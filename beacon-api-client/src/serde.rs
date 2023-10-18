pub(crate) use ethereum_consensus::serde::{as_hex, as_str, seq_of_str};

pub(crate) mod as_u16 {
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
