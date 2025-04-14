mod fury_sdk;
mod utils;

use anyhow::Result;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::main]
async fn main() -> Result<()> {
    let wallets = vec![Keypair::new()];

    let client = reqwest::Client::new();
    let sdk = fury_sdk::FurySDK::new(client);
    let buy_response = match sdk.buy_token(&fury_sdk::BuyTokenRequest {
        wallet_addresses: wallets.iter().map(|w| w.pubkey().to_string()).collect(),
        token_address: "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
        sol_amount: 0.001,
        protocol: fury_sdk::Protocol::Pumpfun,
        // affiliate_address: None,
        // affiliate_fee: None,
        // jito_tip_lamports: Some(5000000),
        // slippage_bps: Some(9990),
        amounts: None,
    }).await {
        Ok(response) => response,
        Err(err) => {
            println!("Error: {:#?}", err);
            return Err(anyhow::anyhow!("Failed to buy token: {:?}", err));
        }
    };

    println!("Buy response:");
    println!("{:#?}", buy_response);

    let signed_txs = match utils::wallets::sign_transactions(&buy_response.transactions, &wallets) {
        Ok(signed_txs) => signed_txs,
        Err(err) => {
            println!("{:#?}", err);
            return Err(anyhow::anyhow!("Failed to sign transactions: {:?}", err));
        }
    };

    println!("Signed txs:");
    println!("{:#?}", signed_txs);

    // TODO: Send transaction
    // let send_response = match sdk.transaction_send(&fury_sdk::TransactionSendRequest {
    //     transactions: signed_txs,
    //     use_rpc: false,
    // }).await {
    //     Ok(response) => response,
    //     Err(err) => {
    //         println!("{:#?}", err);
    //         return Err(anyhow::anyhow!("Failed to send transactions: {:?}", err));
    //     }
    // };

    // println!("{:#?}", send_response);

    Ok(())
}
