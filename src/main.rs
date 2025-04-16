mod fury_sdk;
mod utils;

use anyhow::Result;
use solana_sdk::{native_token::sol_to_lamports, signature::Keypair, signer::Signer};

#[tokio::main]
async fn main() -> Result<()> {
    // ask user for wallet private key as input and then convert it to a keypair
    let mut input = String::new();
    println!("Enter wallet private key: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let wallet = Keypair::from_base58_string(input.trim());

    let wallets = vec![wallet];

    let client = reqwest::Client::new();
    let sdk = fury_sdk::FurySDK::new(client);
    let buy_response = match sdk.buy_token(&fury_sdk::BuyTokenRequest {
        wallet_addresses: wallets.iter().map(|w| w.pubkey().to_string()).collect(),
        token_address: "Bq5nFQ82jBYcFKRzUSximpCmCg5t8L8tVMqsn612pump".to_string(),
        sol_amount: 0.001,
        protocol: fury_sdk::Protocol::Pumpfun,
        jito_tip_lamports: Some(sol_to_lamports(0.001)),
        amounts: None,
        use_rpc: false,
        affiliate_address: None,
        affiliate_fee: None,
        slippage_bps: None,
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

    let mint = sdk.generate_mint().await;
    println!("{:#?}", mint);

    // match sdk.jito_transaction_send(&fury_sdk::TransactionSendRequest {
    //     transactions: signed_txs,
    //     use_rpc: false,
    // }).await {
    //     Ok(response) => println!("{:#?}", response),
    //     Err(err) => {
    //         println!("{:#?}", err);
    //         return Err(anyhow::anyhow!("Failed to send transactions: {:?}", err));
    //     }
    // };

    Ok(())
}
