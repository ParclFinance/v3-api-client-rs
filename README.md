<div align="center">
<img height="180" src="https://avatars.githubusercontent.com/u/84755822?s=200&v=4"/>
<h1 style="margin-top:-15px;">v3-api-client-rs</h1>
</div>

## Quick Start

Rust client library for interacting with parcl-v3 via an api. Provides easy ways to trade and manage margin accounts.

Api and client library are in alpha. Code may change.

## Getting Started

To use the `parcl-v3-api-client` library in your rust project, add the library to your dependencies in your `Cargo.toml`:

```
[dependencies]
parcl-v3-api-client = { git = "https://github.com/ParclFinance/v3-api-client-rs.git", package = "parcl-v3-api-client" }
```

You can also specify the revision to lock the library to a specific commit:

```
[dependencies]
parcl-v3-api-client = { git = "https://github.com/ParclFinance/v3-api-client-rs.git", rev = <REV>, package = "parcl-v3-api-client" }
```

## Usage

```rust
use parcl_v3_api_client::ParclV3ApiClient;

let client = ParclV3ApiClient::default();
```

## Examples

For the full code and other examples please see [examples repo](https://github.com/ParclFinance/v3-api-examples).

Simplified trade:

```rust
use anyhow::Result;
use parcl_v3_api_client::{request::*, ParclV3ApiClient};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use std::sync::Arc;
use utils::{deserialize_tx_set_recent_blockhash_and_sign_message, send_transaction};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup wallet to use for trading, api client, and rpc client.
    let wallet = Keypair::from_base58_string(PRIVATE_KEY);
    let v3_api_client = ParclV3ApiClient::default();
    let rpc_client = Arc::new(RpcClient::new(RPC_URL));

    // Setup trade inputs
    let margin_account_id = MarginAccountIdentifier::Id(0); // Margin account with id 0
    let market_id = 23; // SOL market
    let size_delta = -20; // 0.00002 SOL short
    let slippage_setting = SlippageSetting::SlippageToleranceBps(200); // 2%

    // Fetch trade transaction and latest blockhash.
    let (api_response, latest_blockhash) = {
        let (api_response, latest_blockhash) = tokio::join!(
            v3_api_client.get_modify_position_transaction(
                wallet.pubkey(),
                margin_account_id,
                market_id,
                size_delta,
                slippage_setting,
            ),
            rpc_client.get_latest_blockhash(),
        );
        (api_response?, latest_blockhash?)
    };

    // Deserialize trade transaction into a versioned transaction. Set blockhash and sign transaction.
    let tx = deserialize_tx_set_recent_blockhash_and_sign_message(
        api_response.transaction,
        &wallet,
        latest_blockhash,
    )?;

    // Send trade transaction
    let signature = send_transaction(&tx, rpc_client.clone()).await?;
    println!("Transaction successful: {signature:?}");
    Ok(())
}
```

## Additional Resources

- [OpenAPI Documentation](https://v3.parcl-api.com/docs): See available routes.
- [Examples Repo](https://github.com/ParclFinance/v3-api-examples): Examples of common use cases.
- [Parcl Website](https://parcl.co): Check out the official Parcl website for additional information and resources.
