use crate::{
    request::*,
    serde_utils::{field_as_base64, field_as_string, pubkey_values_map, pubkey_vec},
};
use serde::Deserialize;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct TransactionInfo {
    #[serde(with = "field_as_base64")]
    pub transaction: Vec<u8>,
    pub total_required_lamports: u64,
    pub required_compute_lamports: u64,
    pub required_rent_lamports: u64,
    pub cu_limit: u32,
}

#[derive(Deserialize, Debug)]
pub struct InstructionInfo {
    pub instructions: Instructions,
    pub total_required_lamports: u64,
    pub required_compute_lamports: u64,
    pub required_rent_lamports: u64,
    pub cu_limit: u32,
}

impl From<InstructionInfoInternal> for InstructionInfo {
    fn from(ixs: InstructionInfoInternal) -> Self {
        InstructionInfo {
            instructions: ixs.instructions.into(),
            total_required_lamports: ixs.total_required_lamports,
            required_compute_lamports: ixs.required_compute_lamports,
            required_rent_lamports: ixs.required_rent_lamports,
            cu_limit: ixs.cu_limit,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct InstructionInfoInternal {
    pub instructions: InstructionsInternal,
    pub total_required_lamports: u64,
    pub required_compute_lamports: u64,
    pub required_rent_lamports: u64,
    pub cu_limit: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Instructions {
    pub v3_instructions: Vec<Instruction>,
    pub compute_budget_instructions: Vec<Instruction>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct InstructionsInternal {
    pub v3_instructions: Vec<InstructionInternal>,
    pub compute_budget_instructions: Vec<InstructionInternal>,
}

impl From<InstructionsInternal> for Instructions {
    fn from(ixs: InstructionsInternal) -> Self {
        Instructions {
            v3_instructions: ixs
                .v3_instructions
                .into_iter()
                .map(Instruction::from)
                .collect(),
            compute_budget_instructions: ixs
                .compute_budget_instructions
                .into_iter()
                .map(Instruction::from)
                .collect(),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct InstructionInternal {
    #[serde(with = "field_as_string")]
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMetaInternal>,
    #[serde(with = "field_as_base64")]
    pub data: Vec<u8>,
}

impl From<InstructionInternal> for Instruction {
    fn from(ix: InstructionInternal) -> Self {
        Instruction {
            program_id: ix.program_id,
            accounts: ix.accounts.into_iter().map(AccountMeta::from).collect(),
            data: ix.data,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountMetaInternal {
    #[serde(with = "field_as_string")]
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<AccountMetaInternal> for AccountMeta {
    fn from(account_meta: AccountMetaInternal) -> Self {
        AccountMeta {
            pubkey: account_meta.pubkey,
            is_signer: account_meta.is_signer,
            is_writable: account_meta.is_writable,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateMarginAccountTransactionResponse {
    #[serde(with = "field_as_base64")]
    pub transaction: Vec<u8>,
    pub total_required_lamports: u64,
    pub required_compute_lamports: u64,
    pub required_rent_lamports: u64,
    #[serde(with = "field_as_string")]
    pub margin_account_address: Pubkey,
    pub margin_account_id: MarginAccountId,
}

#[derive(Deserialize, Debug)]
pub struct CreateMarginAccountInstructionsResponse {
    pub instructions: Instructions,
    pub total_required_lamports: u64,
    pub required_compute_lamports: u64,
    pub required_rent_lamports: u64,
    #[serde(with = "field_as_string")]
    pub margin_account_address: Pubkey,
    pub margin_account_id: MarginAccountId,
}

impl From<CreateMarginAccountInstructionsResponseInternal>
    for CreateMarginAccountInstructionsResponse
{
    fn from(ixs: CreateMarginAccountInstructionsResponseInternal) -> Self {
        CreateMarginAccountInstructionsResponse {
            instructions: ixs.instructions.into(),
            total_required_lamports: ixs.total_required_lamports,
            required_compute_lamports: ixs.required_compute_lamports,
            required_rent_lamports: ixs.required_rent_lamports,
            margin_account_address: ixs.margin_account_address,
            margin_account_id: ixs.margin_account_id,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateMarginAccountInstructionsResponseInternal {
    pub instructions: InstructionsInternal,
    pub total_required_lamports: u64,
    pub required_compute_lamports: u64,
    pub required_rent_lamports: u64,
    #[serde(with = "field_as_string")]
    pub margin_account_address: Pubkey,
    pub margin_account_id: MarginAccountId,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExchangeInfo {
    #[serde(with = "field_as_string")]
    pub address: Pubkey,
    pub accounting: ExchangeInfoAccounting,
    pub settings: ExchangeInfoSettings,
    #[serde(with = "field_as_string")]
    pub id: ExchangeId,
    pub market_ids: Vec<MarketId>,
    pub oracle_configs: Vec<OracleConfig>,
    pub status: u16,
    pub collateral_expo: i16,
    #[serde(with = "field_as_string")]
    pub collateral_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub collateral_vault: Pubkey,
    #[serde(with = "field_as_string")]
    pub admin: Pubkey,
    #[serde(with = "field_as_string")]
    pub nominated_admin: Pubkey,
    #[serde(with = "field_as_string")]
    pub authorized_settler: Pubkey,
    #[serde(with = "field_as_string")]
    pub authorized_protocol_fees_collector: Pubkey,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExchangeInfoAccounting {
    #[serde(with = "field_as_string")]
    pub notional_open_interest: u128,
    #[serde(with = "field_as_string")]
    pub last_time_locked_open_interest_accounting_refreshed: u64,
    #[serde(with = "field_as_string")]
    pub balance: u64,
    #[serde(with = "field_as_string")]
    pub margin_balance: u64,
    #[serde(with = "field_as_string")]
    pub lp_balance: u64,
    #[serde(with = "field_as_string")]
    pub lp_shares: u64,
    #[serde(with = "field_as_string")]
    pub protocol_fees: u64,
    #[serde(with = "field_as_string")]
    pub unsettled_collateral_amount: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExchangeInfoSettings {
    pub min_lp_duration: u64,
    pub settlement_delay: u64,
    #[serde(with = "field_as_string")]
    pub min_liquidation_fee: u64,
    #[serde(with = "field_as_string")]
    pub max_liquidation_fee: u64,
    pub locked_open_interest_staleness_threshold: u64,
    pub protocol_fee_rate: u16,
    pub locked_open_interest_ratio: u16,
    pub max_keeper_tip_rate: u16,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OracleConfig {
    pub kind: OracleKind,
    #[serde(with = "field_as_string")]
    pub program_id: Pubkey,
}

#[derive(Deserialize, Clone, Debug)]
pub enum OracleKind {
    Pyth,
    Parcl,
    PythV2,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum MarketIdentifiersResponse {
    Ids(Vec<MarketId>),
    Addresses(#[serde(with = "pubkey_vec")] Vec<Pubkey>),
    #[serde(with = "pubkey_values_map")]
    Map(HashMap<MarketId, Pubkey>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct MarginAccountInfo {
    #[serde(with = "field_as_string")]
    pub address: Pubkey,
    pub id: MarginAccountId,
    pub active_market_ids: Vec<MarketId>,
    pub positions: Vec<PositionInfo>,
    pub margins: Margins,
    #[serde(with = "field_as_string")]
    pub margin: u64,
    #[serde(with = "field_as_string")]
    pub excess_margin: u64,
    #[serde(with = "field_as_string")]
    pub exchange: Pubkey,
    #[serde(with = "field_as_string")]
    pub owner: Pubkey,
    #[serde(with = "field_as_string")]
    pub delegate: Pubkey,
    pub can_close: bool,
    pub can_liquidate: bool,
    pub in_liquidation: bool,
}

#[derive(Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Margins {
    #[serde(with = "field_as_string")]
    pub available_margin: i128,
    #[serde(with = "field_as_string")]
    pub total_required_margin: u64,
    #[serde(with = "field_as_string")]
    pub required_initial_margin: u64,
    #[serde(with = "field_as_string")]
    pub required_maintenance_margin: u64,
    #[serde(with = "field_as_string")]
    pub required_liquidation_fee_margin: u64,
    #[serde(with = "field_as_string")]
    pub accumulated_liquidation_fees: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PositionInfo {
    #[serde(with = "field_as_string")]
    pub size: i128,
    #[serde(with = "field_as_string")]
    pub last_interaction_price: u128,
    pub last_interaction_funding_per_unit: String,
    pub market_id: MarketId,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MarketInfo {
    #[serde(with = "field_as_string")]
    pub address: Pubkey,
    pub price_feed_info: PriceFeedInfo,
    pub accounting: MarketInfoAccounting,
    pub settings: MarketInfoSettings,
    pub id: MarketId,
    #[serde(with = "field_as_string")]
    pub exchange: Pubkey,
    #[serde(with = "field_as_string")]
    pub price_feed: Pubkey,
    pub status: u8,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PriceFeedInfo {
    #[serde(with = "field_as_string")]
    pub price: u64,
    pub expo: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MarketInfoAccounting {
    #[serde(with = "field_as_string")]
    pub last_utilized_liquidation_capacity: u128,
    #[serde(with = "field_as_string")]
    pub size: u128,
    #[serde(with = "field_as_string")]
    pub skew: i128,
    pub last_funding_rate: String,
    pub last_funding_per_unit: String,
    pub last_time_funding_updated: u64,
    pub first_liquidation_epoch_start_time: u64,
    pub last_liquidation_epoch_index: u64,
    pub last_time_liquidation_capacity_updated: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MarketInfoSettings {
    #[serde(with = "field_as_string")]
    pub min_position_margin: u128,
    #[serde(with = "field_as_string")]
    pub skew_scale: u128,
    #[serde(with = "field_as_string")]
    pub max_side_size: u128,
    pub max_liquidation_limit_accumulation_multiplier: u64, // bps
    pub max_seconds_in_liquidation_epoch: u64,
    pub initial_margin_ratio: u32,
    pub maker_fee_rate: u16,
    pub taker_fee_rate: u16,
    pub max_funding_velocity: u16,
    pub liquidation_fee_rate: u16,
    pub min_initial_margin_ratio: u16,
    pub maintenance_margin_proportion: u16,
    pub max_liquidation_pd: u16,
    #[serde(with = "field_as_string")]
    pub authorized_liquidator: Pubkey,
}
