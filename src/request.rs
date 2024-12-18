use crate::serde_utils::{field_as_string, pubkey_vec};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

pub type ExchangeId = u64;
pub type MarginAccountId = u32;
pub type MarketId = u32;
pub type SettlementRequestId = u64;

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(untagged)]
pub enum ExchangeIdentifier {
    #[serde(with = "field_as_string")]
    Id(ExchangeId),
    #[serde(with = "field_as_string")]
    Address(Pubkey),
}

impl Default for ExchangeIdentifier {
    fn default() -> Self {
        Self::Id(ExchangeId::default())
    }
}

impl std::fmt::Display for ExchangeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Id(id) => id.to_string(),
                Self::Address(address) => address.to_string(),
            }
        )
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(untagged)]
pub enum MarginAccountIdentifier {
    #[serde(with = "field_as_string")]
    Id(MarginAccountId),
    #[serde(with = "field_as_string")]
    Address(Pubkey),
}

impl std::fmt::Display for MarginAccountIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Id(id) => id.to_string(),
                Self::Address(address) => address.to_string(),
            }
        )
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(untagged)]
pub enum MarketIdentifier {
    #[serde(with = "field_as_string")]
    Id(MarketId),
    #[serde(with = "field_as_string")]
    Address(Pubkey),
}

impl std::fmt::Display for MarketIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Id(id) => id.to_string(),
                Self::Address(address) => address.to_string(),
            }
        )
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Copy, Clone)]
pub enum MarketIdentifiersResponseKind {
    #[default]
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "addresses")]
    Addresses,
    #[serde(rename = "ids")]
    Ids,
}

impl std::fmt::Display for MarketIdentifiersResponseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ids => "ids",
                Self::Map => "map",
                Self::Addresses => "addresses",
            }
        )
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum SlippageSetting {
    #[serde(with = "field_as_string")]
    AcceptablePrice(u64),
    #[serde(with = "field_as_string")]
    SlippageToleranceBps(u64),
}

#[derive(Deserialize, Serialize)]
pub struct MarginAccountsPayload {
    #[serde(with = "pubkey_vec")]
    pub margin_accounts: Vec<Pubkey>,
    pub exchange_id: Option<ExchangeIdentifier>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketsPayload {
    pub market_ids: Vec<MarketIdentifier>,
    pub exchange_id: Option<ExchangeIdentifier>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CloseMarginAccountPayload {
    #[serde(with = "field_as_string")]
    pub owner: Pubkey,
    pub margin_account_id: MarginAccountIdentifier,
    pub exchange_id: Option<ExchangeIdentifier>,
    pub priority_fee_percentile: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClosePositionPayload {
    #[serde(with = "field_as_string")]
    pub owner: Pubkey,
    pub margin_account_id: MarginAccountIdentifier,
    pub market_id: MarketId,
    pub slippage_setting: SlippageSetting,
    pub exchange_id: Option<ExchangeIdentifier>,
    pub priority_fee_percentile: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateMarginAccountPayload {
    #[serde(with = "field_as_string")]
    pub owner: Pubkey,
    pub margin_account_id: Option<MarginAccountId>,
    pub exchange_id: Option<ExchangeIdentifier>,
    pub priority_fee_percentile: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepositMarginPayload {
    #[serde(with = "field_as_string")]
    pub owner: Pubkey,
    pub margin_account_id: MarginAccountIdentifier,
    pub margin: u64,
    pub exchange_id: Option<ExchangeIdentifier>,
    pub priority_fee_percentile: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModifyPositionPayload {
    #[serde(with = "field_as_string")]
    pub owner: Pubkey,
    pub margin_account_id: MarginAccountIdentifier,
    pub market_id: MarketId,
    pub size_delta: i128,
    pub slippage_setting: SlippageSetting,
    pub exchange_id: Option<ExchangeIdentifier>,
    pub priority_fee_percentile: Option<u16>,
}

impl ModifyPositionPayload {
    pub fn new_with_defaults(
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        market_id: MarketId,
        size_delta: i128,
        slippage_setting: SlippageSetting,
    ) -> Self {
        Self {
            owner,
            margin_account_id,
            market_id,
            size_delta,
            slippage_setting,
            exchange_id: None,
            priority_fee_percentile: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WithdrawMarginPayload {
    #[serde(with = "field_as_string")]
    pub owner: Pubkey,
    pub margin_account_id: MarginAccountIdentifier,
    pub margin: u64,
    pub settlement_request_id: Option<SettlementRequestId>,
    pub keeper_tip: Option<u64>,
    pub exchange_id: Option<ExchangeIdentifier>,
    pub priority_fee_percentile: Option<u16>,
}

#[derive(Deserialize, Serialize)]
pub struct LiquidatePayload {
    #[serde(with = "field_as_string")]
    pub margin_account_to_liquidate: Pubkey,
    #[serde(with = "field_as_string")]
    pub liquidator: Pubkey,
    pub liquidator_margin_account_id: MarginAccountIdentifier,
    pub exchange_id: Option<ExchangeIdentifier>,
    pub priority_fee_percentile: Option<u16>,
}
