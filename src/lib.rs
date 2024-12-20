pub mod constants;
pub mod request;
pub mod response;
mod serde_utils;

use constants::*;
use request::*;
use response::*;

use anyhow::Result;
use reqwest::{Client, Response, StatusCode};
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone)]
pub struct ParclV3ApiClient {
    client: Client,
    base_url: String,
    exchange_id: ExchangeIdentifier,
    priority_fee_percentile: Option<u16>,
}

impl Default for ParclV3ApiClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            base_url: DEFAULT_V3_API_URL.to_string(),
            exchange_id: ExchangeIdentifier::default(),
            priority_fee_percentile: None,
        }
    }
}

#[derive(Clone)]
pub struct ParclV3ApiClientConfig {
    pub base_url: String,
    pub exchange_id: Option<ExchangeIdentifier>,
    pub priority_fee_percentile: Option<u16>,
}

impl ParclV3ApiClient {
    pub fn new(config: ParclV3ApiClientConfig) -> Self {
        Self {
            client: Client::new(),
            base_url: config.base_url,
            exchange_id: config.exchange_id.unwrap_or_default(),
            priority_fee_percentile: config.priority_fee_percentile,
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    pub async fn get_exchange(&self) -> Result<ExchangeInfo> {
        let response = self
            .client
            .get(self.build_url("/exchange"))
            .query(&[("exchange_id", self.exchange_id.to_string())])
            .send()
            .await?;
        validate_and_deserialize_response::<ExchangeInfo>(response).await
    }

    pub async fn get_exponents(&self) -> Result<HashMap<String, i32>> {
        let response = self
            .client
            .get(self.build_url("/exponents"))
            .query(&[("exchange_id", self.exchange_id.to_string())])
            .send()
            .await?;
        validate_and_deserialize_response::<HashMap<String, i32>>(response).await
    }

    async fn get_market_ids_internal(
        &self,
        response_kind: MarketIdentifiersResponseKind,
    ) -> Result<MarketIdentifiersResponse> {
        let response = self
            .client
            .get(self.build_url("/market-ids"))
            .query(&[("response_kind", response_kind)])
            .query(&[("exchange_id", self.exchange_id.to_string())])
            .send()
            .await?;
        validate_and_deserialize_response::<MarketIdentifiersResponse>(response).await
    }

    pub async fn get_market_ids(&self) -> Result<Vec<MarketId>> {
        let response = self
            .get_market_ids_internal(MarketIdentifiersResponseKind::Ids)
            .await?;
        match response {
            MarketIdentifiersResponse::Ids(ids) => Ok(ids),
            _ => Err(
                ParclV3ApiClientError::MarketIdsResponse(MarketIdentifiersResponseKind::Ids).into(),
            ),
        }
    }

    pub async fn get_market_ids_map(&self) -> Result<HashMap<MarketId, Pubkey>> {
        let response = self
            .get_market_ids_internal(MarketIdentifiersResponseKind::Map)
            .await?;
        match response {
            MarketIdentifiersResponse::Map(map) => Ok(map),
            _ => Err(
                ParclV3ApiClientError::MarketIdsResponse(MarketIdentifiersResponseKind::Map).into(),
            ),
        }
    }

    pub async fn get_market_addresses(&self) -> Result<Vec<Pubkey>> {
        let response = self
            .get_market_ids_internal(MarketIdentifiersResponseKind::Addresses)
            .await?;
        match response {
            MarketIdentifiersResponse::Addresses(addresses) => Ok(addresses),
            _ => Err(ParclV3ApiClientError::MarketIdsResponse(
                MarketIdentifiersResponseKind::Addresses,
            )
            .into()),
        }
    }

    pub async fn get_margin_account(
        &self,
        margin_account_id: MarginAccountIdentifier,
        owner: Option<Pubkey>,
    ) -> Result<MarginAccountInfo> {
        let response = self
            .client
            .get(self.build_url("/margin-account"))
            .query(&[("margin_account_id", margin_account_id.to_string())])
            .query(&[("owner", owner.map(|owner| owner.to_string()))])
            .query(&[("exchange_id", self.exchange_id.to_string())])
            .send()
            .await?;
        validate_and_deserialize_response::<MarginAccountInfo>(response).await
    }

    pub async fn get_margin_account_from_id(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountId,
    ) -> Result<MarginAccountInfo> {
        self.get_margin_account(MarginAccountIdentifier::Id(margin_account_id), Some(owner))
            .await
    }

    pub async fn get_margin_account_from_address(
        &self,
        address: Pubkey,
    ) -> Result<MarginAccountInfo> {
        self.get_margin_account(MarginAccountIdentifier::Address(address), None)
            .await
    }

    pub async fn get_margin_accounts(
        &self,
        margin_accounts: &[Pubkey],
    ) -> Result<Vec<Option<MarginAccountInfo>>> {
        let response = self
            .client
            .post(self.build_url("/margin-accounts"))
            .json(&MarginAccountsPayload {
                margin_accounts: margin_accounts.to_vec(),
                exchange_id: Some(self.exchange_id),
            })
            .send()
            .await?;
        validate_and_deserialize_response::<Vec<Option<MarginAccountInfo>>>(response).await
    }

    pub async fn get_unhealthy_margin_accounts(&self) -> Result<Vec<Pubkey>> {
        let response = self
            .client
            .get(self.build_url("/unhealthy-margin-accounts"))
            .query(&[("exchange_id", self.exchange_id.to_string())])
            .send()
            .await?;
        let unhealthy_margin_accounts =
            validate_and_deserialize_response::<Vec<String>>(response).await?;
        Ok(unhealthy_margin_accounts
            .into_iter()
            .flat_map(|s| Pubkey::from_str(&s).ok())
            .collect::<Vec<Pubkey>>())
    }

    pub async fn get_market(&self, market_id: MarketIdentifier) -> Result<MarketInfo> {
        let response = self
            .client
            .get(self.build_url("/market"))
            .query(&[("market_id", market_id.to_string())])
            .query(&[("exchange_id", self.exchange_id.to_string())])
            .send()
            .await?;
        validate_and_deserialize_response::<MarketInfo>(response).await
    }

    pub async fn get_market_from_id(&self, market_id: MarketId) -> Result<MarketInfo> {
        self.get_market(MarketIdentifier::Id(market_id)).await
    }

    pub async fn get_market_from_address(&self, address: Pubkey) -> Result<MarketInfo> {
        self.get_market(MarketIdentifier::Address(address)).await
    }

    pub async fn get_markets(&self, market_ids: &[MarketIdentifier]) -> Result<Vec<MarketInfo>> {
        let response = self
            .client
            .post(self.build_url("/markets"))
            .json(&MarketsPayload {
                market_ids: market_ids.to_vec(),
                exchange_id: Some(self.exchange_id),
            })
            .send()
            .await?;
        validate_and_deserialize_response::<Vec<MarketInfo>>(response).await
    }

    pub async fn get_markets_from_addresses(
        &self,
        addresses: &[Pubkey],
    ) -> Result<Vec<MarketInfo>> {
        let addresses = addresses
            .iter()
            .map(|address| MarketIdentifier::Address(*address))
            .collect::<Vec<MarketIdentifier>>();
        self.get_markets(&addresses).await
    }

    pub async fn get_markets_from_ids(&self, ids: &[MarketId]) -> Result<Vec<MarketInfo>> {
        let ids = ids
            .iter()
            .map(|id| MarketIdentifier::Id(*id))
            .collect::<Vec<MarketIdentifier>>();
        self.get_markets(&ids).await
    }

    pub async fn get_create_margin_account_transaction(
        &self,
        owner: Pubkey,
        margin_account_id: Option<MarginAccountId>,
    ) -> Result<CreateMarginAccountTransactionResponse> {
        let response = self
            .client
            .post(self.build_url("/create-margin-account-transaction"))
            .json(&CreateMarginAccountPayload {
                owner,
                margin_account_id,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<CreateMarginAccountTransactionResponse>(response).await
    }

    pub async fn get_create_margin_account_instructions(
        &self,
        owner: Pubkey,
        margin_account_id: Option<MarginAccountId>,
    ) -> Result<CreateMarginAccountInstructionsResponse> {
        let response = self
            .client
            .post(self.build_url("/create-margin-account-instructions"))
            .json(&CreateMarginAccountPayload {
                owner,
                margin_account_id,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<CreateMarginAccountInstructionsResponseInternal>(
            response,
        )
        .await
        .map(Into::into)
    }

    pub async fn get_close_margin_account_transaction(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
    ) -> Result<TransactionInfo> {
        let response = self
            .client
            .post(self.build_url("/close-margin-account-transaction"))
            .json(&CloseMarginAccountPayload {
                owner,
                margin_account_id,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<TransactionInfo>(response).await
    }

    pub async fn get_close_margin_account_instructions(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
    ) -> Result<InstructionInfo> {
        let response = self
            .client
            .post(self.build_url("/close-margin-account-instructions"))
            .json(&CloseMarginAccountPayload {
                owner,
                margin_account_id,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<InstructionInfoInternal>(response)
            .await
            .map(Into::into)
    }

    pub async fn get_deposit_margin_transaction(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        margin: u64,
    ) -> Result<TransactionInfo> {
        let response = self
            .client
            .post(self.build_url("/deposit-margin-transaction"))
            .json(&DepositMarginPayload {
                owner,
                margin_account_id,
                margin,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<TransactionInfo>(response).await
    }

    pub async fn get_deposit_margin_instructions(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        margin: u64,
    ) -> Result<InstructionInfo> {
        let response = self
            .client
            .post(self.build_url("/deposit-margin-instructions"))
            .json(&DepositMarginPayload {
                owner,
                margin_account_id,
                margin,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<InstructionInfoInternal>(response)
            .await
            .map(Into::into)
    }

    pub async fn get_withdraw_margin_transaction(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        margin: u64,
        settlement_request_id: Option<SettlementRequestId>,
        keeper_tip: Option<u64>,
    ) -> Result<TransactionInfo> {
        let response = self
            .client
            .post(self.build_url("/withdraw-margin-transaction"))
            .json(&WithdrawMarginPayload {
                owner,
                margin_account_id,
                margin,
                settlement_request_id,
                keeper_tip,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<TransactionInfo>(response).await
    }

    pub async fn get_withdraw_margin_instructions(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        margin: u64,
        settlement_request_id: Option<SettlementRequestId>,
        keeper_tip: Option<u64>,
    ) -> Result<InstructionInfo> {
        let response = self
            .client
            .post(self.build_url("/withdraw-margin-instructions"))
            .json(&WithdrawMarginPayload {
                owner,
                margin_account_id,
                margin,
                settlement_request_id,
                keeper_tip,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<InstructionInfoInternal>(response)
            .await
            .map(Into::into)
    }

    pub async fn get_modify_position_transaction(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        market_id: MarketId,
        size_delta: i128,
        slippage_setting: SlippageSetting,
    ) -> Result<TransactionInfo> {
        let (maybe_acceptable_price, maybe_slippage_tolerance_bps) =
            slippage_setting.as_request_fields();
        let response = self
            .client
            .post(self.build_url("/modify-position-transaction"))
            .json(&ModifyPositionPayload {
                owner,
                margin_account_id,
                market_id,
                size_delta,
                acceptable_price: maybe_acceptable_price,
                slippage_tolerance_bps: maybe_slippage_tolerance_bps,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<TransactionInfo>(response).await
    }

    pub async fn get_modify_position_instructions(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        market_id: MarketId,
        size_delta: i128,
        slippage_setting: SlippageSetting,
    ) -> Result<InstructionInfo> {
        let (maybe_acceptable_price, maybe_slippage_tolerance_bps) =
            slippage_setting.as_request_fields();
        let response = self
            .client
            .post(self.build_url("/modify-position-instructions"))
            .json(&ModifyPositionPayload {
                owner,
                margin_account_id,
                market_id,
                size_delta,
                acceptable_price: maybe_acceptable_price,
                slippage_tolerance_bps: maybe_slippage_tolerance_bps,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<InstructionInfoInternal>(response)
            .await
            .map(Into::into)
    }

    pub async fn get_close_position_transaction(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        market_id: MarketId,
        slippage_setting: SlippageSetting,
    ) -> Result<TransactionInfo> {
        let (maybe_acceptable_price, maybe_slippage_tolerance_bps) =
            slippage_setting.as_request_fields();
        let response = self
            .client
            .post(self.build_url("/close-position-transaction"))
            .json(&ClosePositionPayload {
                owner,
                margin_account_id,
                market_id,
                acceptable_price: maybe_acceptable_price,
                slippage_tolerance_bps: maybe_slippage_tolerance_bps,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<TransactionInfo>(response).await
    }

    pub async fn get_close_position_instructions(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        market_id: MarketId,
        slippage_setting: SlippageSetting,
    ) -> Result<InstructionInfo> {
        let (maybe_acceptable_price, maybe_slippage_tolerance_bps) =
            slippage_setting.as_request_fields();
        let response = self
            .client
            .post(self.build_url("/close-position-instructions"))
            .json(&ClosePositionPayload {
                owner,
                margin_account_id,
                market_id,
                acceptable_price: maybe_acceptable_price,
                slippage_tolerance_bps: maybe_slippage_tolerance_bps,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<InstructionInfoInternal>(response)
            .await
            .map(Into::into)
    }

    pub async fn get_liquidate_transaction(
        &self,
        margin_account_to_liquidate: Pubkey,
        liquidator: Pubkey,
        liquidator_margin_account_id: MarginAccountIdentifier,
    ) -> Result<TransactionInfo> {
        let response = self
            .client
            .post(self.build_url("/liquidate-transaction"))
            .json(&LiquidatePayload {
                margin_account_to_liquidate,
                liquidator,
                liquidator_margin_account_id,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<TransactionInfo>(response).await
    }

    pub async fn get_liquidate_instructions(
        &self,
        margin_account_to_liquidate: Pubkey,
        liquidator: Pubkey,
        liquidator_margin_account_id: MarginAccountIdentifier,
    ) -> Result<InstructionInfo> {
        let response = self
            .client
            .post(self.build_url("/liquidate-instructions"))
            .json(&LiquidatePayload {
                margin_account_to_liquidate,
                liquidator,
                liquidator_margin_account_id,
                exchange_id: Some(self.exchange_id),
                priority_fee_percentile: self.priority_fee_percentile,
            })
            .send()
            .await?;
        validate_and_deserialize_response::<InstructionInfoInternal>(response)
            .await
            .map(Into::into)
    }

    pub async fn get_modify_position_quote(
        &self,
        owner: Pubkey,
        margin_account_id: MarginAccountIdentifier,
        market_id: MarketId,
        size_delta: i128,
        slippage_setting: SlippageSetting,
    ) -> Result<ModifyPositionQuote> {
        let (maybe_acceptable_price, maybe_slippage_tolerance_bps) =
            slippage_setting.as_request_fields();
        let response = self
            .client
            .post(self.build_url("/modify-position-quote"))
            .json(&ModifyPositionQuotePayload {
                owner,
                margin_account_id,
                market_id,
                size_delta,
                acceptable_price: maybe_acceptable_price,
                slippage_tolerance_bps: maybe_slippage_tolerance_bps,
                exchange_id: Some(self.exchange_id),
            })
            .send()
            .await?;
        validate_and_deserialize_response::<ModifyPositionQuote>(response)
            .await
            .map(Into::into)
    }
}

async fn validate_response(response: Response) -> Result<Response> {
    if !response.status().is_success() {
        return Err(ParclV3ApiClientError::Request(
            response.status(),
            response.text().await.unwrap_or_default(),
        )
        .into());
    }
    Ok(response)
}

async fn validate_and_deserialize_response<T: serde::de::DeserializeOwned>(
    response: Response,
) -> Result<T> {
    validate_response(response)
        .await?
        .json::<T>()
        .await
        .map_err(Into::into)
}

#[derive(thiserror::Error, Debug)]
pub enum ParclV3ApiClientError {
    #[error("Invalid market ids response. Used {0} as ids response_kind.")]
    MarketIdsResponse(MarketIdentifiersResponseKind),
    #[error(
        r#"
        Error Status: {0}
        Body: {1:#?}
        "#
    )]
    Request(StatusCode, String),
}
