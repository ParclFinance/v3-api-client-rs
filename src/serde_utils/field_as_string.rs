use serde::{de, Deserialize, Deserializer, Serializer};
use std::str::FromStr;

pub fn serialize<T, S>(data: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    serializer.serialize_str(&data.to_string())
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    D: Deserializer<'de>,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let s: String = String::deserialize(deserializer)?;
    s.parse()
        .map_err(|err| de::Error::custom(format!("Parse error: {err:?}")))
}
