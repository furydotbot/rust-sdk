# FURY SDK for Rust

A Rust client library for interacting with the FURY API service - a comprehensive toolkit for Solana token operations.

## Features

- **Complete API Coverage** - Access to all FURY API endpoints
- **Type Safety** - Strong typing with Rust's type system
- **Async Support** - Built on Tokio for efficient async operations
- **Error Handling** - Comprehensive error handling with anyhow
- **Validation** - Built-in parameter validation
- **Solana Integration** - Native support for Solana transactions and signatures

## Installation

### Requirements

- Rust 1.70+
- Cargo package manager

### Add to Your Project

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fury-sdk = "0.1.0"
```

## Quick Start

```rust
use fury_sdk::sdk::FurySDK;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_client = reqwest::Client::new();
    let client = FurySDK::new(http_client);

    // Generate a new mint key
    let mint_key = client.generate_mint().await?;
    println!("Generated mint key: {}", mint_key.pubkey);

    Ok(())
}
```

## Usage Examples

### Buy Tokens

```rust
use fury_sdk::sdk::{FurySDK, BuyTokenRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_client = reqwest::Client::new();
    let fury = FurySDK::new(http_client);

    // Buy tokens
    let result = fury.buy_token(&BuyTokenRequest {
        wallet_addresses: vec!["FuRytmqsoo4mKQAhNXoB64JD4SsiVqxYkUKC6i1VaBot".to_string()],
        token_address: "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
        sol_amount: 0.001,
        protocol: fury::Protocol::Pumpfun,
        jito_tip_lamports: Some(sol_to_lamports(0.001)),
        amounts: None,
        use_rpc: false,
        affiliate_address: None,
        affiliate_fee: None,
        slippage_bps: None,
    }).await?;

    println!("Transaction data: {:?}", result.transactions);
    Ok(())
}
```

### Complete Buy Flow with Transaction Signing

```rust
use fury_sdk::sdk::{FurySDK, BuyTokenRequest, TransactionSendRequest};
use fury_sdk::utils::sign_transactions;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize SDK
    let http_client = reqwest::Client::new();
    let fury = FurySDK::new(http_client);

    // Load wallet
    let wallet = Keypair::from_base58_string("YOUR_PRIVATE_KEY_HERE");

    // Generate buy transaction
    let buy_result = fury.buy_token(&fury::BuyTokenRequest {
        wallet_addresses: vec![wallet.pubkey().to_string()],
        token_address: "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
        sol_amount: 0.001,
        protocol: fury::Protocol::Pumpfun,
        jito_tip_lamports: Some(sol_to_lamports(0.001)),
        amounts: None,
        use_rpc: false,
        affiliate_address: None,
        affiliate_fee: None,
        slippage_bps: None,
    }).await?;

    let signed_txs = sign_transactions(buy_result.transactions, vec![wallet]);

    let send_result = fury
        .jito_transaction_send(&TransactionSendRequest {
            transactions: signed_txs,
            use_rpc: false,
        })
        .await?

    println!("Transaction signatures: {:?}", send_result.results);
    Ok(())
}
```

### Create a New Token

```rust
use fury_sdk::sdk::{FurySDK, TokenCreationConfig, TokenCreation, TokenCreationMetadata, TokensCreateRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_client = reqwest::Client::new();
    let fury = FurySDK::new(http_client);

    let result = match client.tokens_create(&TokensCreateRequest {
        wallet_addresses: vec!["5tqe3S1zsfAmT7L2Ru5gVJDaq4wUB7AbCpTLPaxaM6eG".to_string()],
        mint_pubkey: "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
        config: TokenCreationConfig {
            token_creation: TokenCreation {
                metadata: TokenCreationMetadata {
                    name: "Test Token".to_string(),
                    symbol: "TEST".to_string(),
                    description: Some("A test token created with FURY SDK".to_string()),
                    telegram: None,
                    twitter: None,
                    website: None,
                    file: "https://example.com/logo.png".to_string(),
                },
                default_sol_amount: 0.001,
            },
        },
        amounts: vec![0.001],
    }).await?;

    println!("Token creation transactions: {:?}", result);
    Ok(())
}
```

### Distribute Tokens to Multiple Wallets

```rust
use fury_sdk::sdk::{FurySDK, WalletsDistributeRequest, WalletsDistributeRecipient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_client = reqwest::Client::new();
    let fury = FurySDK::new(http_client);

    // Define recipients
    let recipients = vec![
        WalletsDistributeRecipient { address: "8fwjXcyQrCCkG5k3vHUioVLNbPr72otA59mmR1w6CwpS".to_string(), amount: 0.01 },
        WalletsDistributeRecipient { address: "68qzyqvqX3eEGEfwa2ajsDKmEjhmU9XRj1VjcUPJNwpq".to_string(), amount: 0.01 },
    ];

    // Distribute tokens
    let result = fury.wallets_distribute(&WalletsDistributeRequest {
        sender: wallet.pubkey().to_string(),
        recipients,
    }).await?;

    println!("Distribution transactions: {:?}", result.transactions);
    Ok(())
}
```

## API Reference

### Main SDK Struct

- `FurySDK::new(client: Client) -> Self` - Initialize the SDK with a reqwest client
- `FurySDK::new_with_base_url(client: Client, base_url: &str) -> Self` - Initialize the SDK with custom base URL
- `health_check() -> Result<HealthCheckResponse, FuryError>` - Check API health

### Token Operations

- `buy_token(data: &BuyTokenRequest) -> Result<BuyTokenResponse, FuryError>` - Buy tokens
- `sell_token(data: &SellRequest) -> Result<SellResponse, FuryError>` - Sell tokens
- `token_transfer(data: &TokenTransferRequest) -> Result<TokenTransferResponse, FuryError>` - Transfer tokens
- `tokens_create(data: &TokensCreateRequest) -> Result<TokensCreateResponse, FuryError>` - Create a new token
- `token_burn(data: &TokenBurnRequest) -> Result<TokenBurnResponse, FuryError>` - Burn tokens
- `token_cleaner(data: &TokenCleanerRequest) -> Result<TokenCleanerResponse, FuryError>` - Execute buy/sell operations

### Transaction Operations

- `jito_transaction_send(data: &TransactionSendRequest) -> Result<JitoTransactionSendResponse, FuryError>` - Submit transactions via Jito
- `rpc_transaction_send(data: &TransactionSendRequest) -> Result<RpcTransactionSendResponse, FuryError>` - Submit transactions via RPC

### Analytics Operations

- `analytics_pnl(addresses: Vec<String>, token_address: String, options: AnalyticsPnlOptions) -> Result<AnalyticsPnlResponse, FuryError>` - Calculate profit and loss

### Utility Operations

- `generate_mint() -> Result<GenerateMintResponse, FuryError>` - Generate a new mint public key

### Wallet Operations

- `wallets_distribute(data: &WalletsDistributeRequest) -> Result<WalletsDistributeResponse, FuryError>` - Distribute tokens to multiple wallets
- `wallets_consolidate(data: &WalletsConsolidateRequest) -> Result<WalletsConsolidateResponse, FuryError>` - Consolidate tokens from multiple wallets

### Error Handling

The SDK uses a custom `FuryError` enum for error handling:

```rust
pub enum FuryError {
    ApiError(ErrorResponse),
    RequestError(reqwest::Error),
    Other(anyhow::Error),
}
```

Example: 

```rust
use fury_sdk::sdk::{FurySDK, WalletsDistributeRequest, WalletsDistributeRecipient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_client = reqwest::Client::new();
    let fury = FurySDK::new(http_client);

    // You can handle specific errors if necessary 
    let mint = match client.generate_mint().await {
        Ok(key) => key,
        Err(FuryError::ApiError(error)) => {
            println!("Error: {:#?}", error);
            return Err(anyhow::anyhow!("API returned error: {:?}", error));
        }
        Err(FuryError::RequestError(err)) => {
            println!("Error: {:#?}", err);
            return Err(anyhow::anyhow!("Request error: {:?}", err));
        }
        Err(FuryError::Other(err)) => {
            println!("Other error: {:#?}", err);
            return Err(anyhow::anyhow!("Other error: {:?}", err));
        }
    };
    
    // or handle them in a more generic way (uncomment below to see)
    // let mint = match client.generate_mint().await {
    //     Ok(key) => key,
    //     Err(err) => {
    //         println!("Error: {:#?}", err);
    //         return Err(anyhow::anyhow!("Error occurred: {:?}", err));
    //     }
    // };

    println!("Mint: {mint}");
    Ok(())
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
