use serde::{Deserialize, Deserializer, Serialize, Serializer};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn serialize<S>(data: &[Pubkey], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let v = data
        .iter()
        .map(|pubkey| pubkey.to_string())
        .collect::<Vec<String>>();
    v.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Pubkey>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<String> = Vec::deserialize(deserializer)?;
    v.into_iter()
        .map(|s| Pubkey::from_str(&s).map_err(serde::de::Error::custom))
        .collect()
}
