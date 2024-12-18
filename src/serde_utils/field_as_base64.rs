use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{de, Deserialize, Deserializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    BASE64_STANDARD
        .decode(s)
        .map_err(|err| de::Error::custom(format!("Base64 decode error: {err:?}")))
}
