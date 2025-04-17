use std::sync::Arc;

use anyhow::Result;
use fury_sdk::{
    sdk::{
        AnalyticsPnlOptions, BuyTokenRequest, FuryError, FurySDK, Protocol, SellRequest, TokenCreation, TokenCreationConfig, TokenCreationMetadata, TokensCreateRequest, TransactionSendRequest, WalletsDistributeRecipient, WalletsDistributeRequest
    },
    utils::sign_transactions,
};
use solana_sdk::{native_token::sol_to_lamports, signature::Keypair, signer::Signer};

/**
 * Please note that this file is only intended to be used for testing purposes.
 */
#[tokio::main]
async fn main() -> Result<()> {
    let http_client = reqwest::Client::new();
    let client = FurySDK::new(http_client);

    // ask user for wallet private key as input and then convert it to a keypair
    let mut input = String::new();
    println!("Enter wallet private key: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let wallet = Arc::new(Keypair::from_base58_string(input.trim()));

    let wallets = vec![wallet.clone()];

    // --------------------------------------------
    // Buy token
    // --------------------------------------------
    // let buy_response = match client
    //     .buy_token(&BuyTokenRequest {
    //         wallet_addresses: wallets.iter().map(|w| w.pubkey().to_string()).collect(),
    //         token_address: "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
    //         sol_amount: 0.001,
    //         protocol: Protocol::Pumpfun,
    //         jito_tip_lamports: Some(sol_to_lamports(0.001)),
    //         amounts: None,
    //         use_rpc: false,
    //         affiliate_address: None,
    //         affiliate_fee: None,
    //         slippage_bps: None,
    //     })
    //     .await
    // {
    //     Ok(response) => response,
    //     Err(err) => {
    //         println!("Error: {:#?}", err);
    //         return Err(anyhow::anyhow!("Failed to buy token: {:?}", err));
    //     }
    // };

    // println!("Buy response:");
    // println!("{:#?}", buy_response);

    // let signed_txs = match sign_transactions(&buy_response.transactions, &wallets) {
    //     Ok(signed_txs) => signed_txs,
    //     Err(err) => {
    //         println!("{:#?}", err);
    //         return Err(anyhow::anyhow!("Failed to sign transactions: {:?}", err));
    //     }
    // };

    // println!("Signed txs:");
    // println!("{:#?}", signed_txs);
    
    // --------------------------------------------
    // Sell token
    // --------------------------------------------
    // let sell_response = match client.sell_token(&SellRequest {
    //     wallet_addresses: vec![wallet.pubkey().to_string()],
    //     token_address: "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
    //     percentage: 100,
    //     protocol: Protocol::Pumpfun,
    //     affiliate_address: None,
    //     affiliate_fee: None,
    //     jito_tip_lamports: None,
    //     slippage_bps: None,
    // }).await {
    //     Ok(response) => response,
    //     Err(err) => {
    //         println!("Error: {:#?}", err);
    //         return Err(anyhow::anyhow!("Failed to sell token: {:?}", err));
    //     }
    // };

    // println!("Sell response:");
    // println!("{:#?}", sell_response);

    // let signed_txs = match sign_transactions(&sell_response.transactions, &wallets) {
    //     Ok(signed_txs) => signed_txs,
    //     Err(err) => {
    //         println!("{:#?}", err);
    //         return Err(anyhow::anyhow!("Failed to sign transactions: {:?}", err));
    //     }
    // };

    // println!("Signed txs:");
    // println!("{:#?}", signed_txs);

    // let mint = client.generate_mint().await;
    // println!("{:#?}", mint);

    // let pnl_wallets = vec![
    //     "Bp7RdpR7HHJwWdQ8TXmGpFEkP1pBFHdviNQn45mrFSCo".to_string(),
    //     "Bp7RdpR7HHJwWdQ8TXmGpFEkP1pBFHdviNQn45mrFSCo".to_string(),
    // ];
    // let pnl = client
    //     .analytics_pnl(
    //         pnl_wallets,
    //         "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
    //         AnalyticsPnlOptions {
    //             include_timestamp: true,
    //         },
    //     )
    //     .await;
    // println!("{:#?}", pnl);

    // --------------------------------------------
    // Create token
    // --------------------------------------------
    // let create_token_response = client.tokens_create(&TokensCreateRequest {
    //     wallet_addresses: vec![wallet.pubkey().to_string()],
    //     mint_pubkey: "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
    //     config: TokenCreationConfig {
    //         token_creation: TokenCreation {
    //             metadata: TokenCreationMetadata {
    //                 name: "Test Token".to_string(),
    //                 symbol: "TEST".to_string(),
    //                 description: Some("A test token created with FURY SDK".to_string()),
    //                 telegram: None,
    //                 twitter: None,
    //                 website: None,
    //                 file: "https://example.com/logo.png".to_string(),
    //             },
    //             default_sol_amount: 0.001,
    //         },
    //     },
    //     amounts: vec![0.001],
    // }).await;

    // println!("{:#?}", create_token_response);

    // --------------------------------------------
    // Wallets distribute
    // --------------------------------------------
    // let wallets_distribute_response = client.wallets_distribute(&WalletsDistributeRequest {
    //     sender: wallet.pubkey().to_string(),
    //     recipients: vec![WalletsDistributeRecipient {
    //         address: "5tqe3S1zsfAmT7L2Ru5gVJDaq4wUB7AbCpTLPaxaM6eG".to_string(),
    //         amount: 0.001,
    //     }],
    // }).await;

    // println!("{:#?}", wallets_distribute_response);

    // // Generate a new mint key
    // let mint_key = match client.generate_mint().await {
    //     Ok(mint_key) => mint_key,
    //     Err(err) => {
    //         println!("Error: {:#?}", err);
    //         return Err(anyhow::anyhow!("API returned error: {:?}", error));
    //     }
    //     Err(FuryError::RequestError(err)) => {
    //         println!("Error: {:#?}", err);
    //         return Err(anyhow::anyhow!("Request error: {:?}", err));
    //     }
    //     Err(FuryError::Other(err)) => {
    //         println!("Other error: {:#?}", err);
    //         return Err(anyhow::anyhow!("Other error: {:?}", err));
    //     }
    // };
    // println!("Generated mint key: {}", mint_key.pubkey);
    
    

    // match client
    //     .jito_transaction_send(&TransactionSendRequest {
    //         transactions: signed_txs,
    //         use_rpc: false,
    //     })
    //     .await
    // {
    //     Ok(response) => println!("{:#?}", response),
    //     Err(err) => {
    //         println!("{:#?}", err);
    //         return Err(anyhow::anyhow!("Failed to send transactions: {:?}", err));
    //     }
    // };



    Ok(())
}
