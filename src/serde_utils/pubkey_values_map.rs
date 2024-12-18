use crate::request::MarketId;
use serde::{Deserialize, Deserializer};
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashMap, str::FromStr};

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<MarketId, Pubkey>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::new();
    for (id, address) in map {
        let market_id = id.parse::<MarketId>().map_err(serde::de::Error::custom)?;
        let pubkey = Pubkey::from_str(&address).map_err(serde::de::Error::custom)?;
        result.insert(market_id, pubkey);
    }
    Ok(result)
}
