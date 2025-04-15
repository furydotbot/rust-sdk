use std::{fmt::{self, Display}, str::FromStr};
use anyhow::Result;

use reqwest::Client;
use serde::{Serialize, Deserialize};

impl FromStr for Protocol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raydium" => Ok(Protocol::Raydium),
            "jupiter" => Ok(Protocol::Jupiter),
            "pumpfun" => Ok(Protocol::Pumpfun),
            "moonshot" => Ok(Protocol::Moonshot),
            "pumpswap" => Ok(Protocol::Pumpswap),
            "auto" => Ok(Protocol::Auto),
            _ => Err(anyhow::anyhow!("Invalid protocol: {}", s)),
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::Raydium => write!(f, "raydium"),
            Protocol::Jupiter => write!(f, "jupiter"),
            Protocol::Pumpfun => write!(f, "pumpfun"),
            Protocol::Moonshot => write!(f, "moonshot"),
            Protocol::Pumpswap => write!(f, "pumpswap"),
            Protocol::Auto => write!(f, "auto"),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Raydium,
    Jupiter,
    Pumpfun,
    Moonshot,
    Pumpswap,
    Auto,
}

// --------------------------------------------
// Token buy 
// --------------------------------------------
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuyTokenRequest {
    pub wallet_addresses: Vec<String>,
    pub token_address: String,
    pub sol_amount: f64,
    pub protocol: Protocol,
    pub affiliate_address: Option<String>,
    pub affiliate_fee: Option<String>,
    pub jito_tip_lamports: Option<u64>,
    pub slippage_bps: Option<u64>,
    pub amounts: Option<Vec<f64>>,
    pub use_rpc: bool,
}

#[derive(Deserialize, Debug)]
pub struct BuyTokenResponse {
    pub success: bool,
    pub transactions: Vec<String>,
}


// --------------------------------------------
// Token sell 
// --------------------------------------------
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SellRequest {
    pub wallet_addresses: Vec<String>,
    pub token_address: String,
    pub percentage: u64,
    pub protocol: Protocol,
    pub affiliate_address: Option<String>,
    pub affiliate_fee: Option<String>,
    pub jito_tip_lamports: Option<u64>,
    pub slippage_bps: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct SellResponse {
    pub success: bool,
    pub transactions: Vec<String>,
}

// --------------------------------------------
// Transaction send 
// --------------------------------------------
#[derive(Serialize)]
pub struct TransactionSendRequest {
    // Signed transactions
    pub transactions: Vec<String>,
    pub use_rpc: bool,
}

#[derive(Deserialize, Debug)]
pub struct JitoTxResult {
    pub jito: String,
}

#[derive(Deserialize, Debug)]
pub struct JitoTransactionSendResponse {
    pub success: bool,
    // Transaction signatures
    pub result: JitoTxResult,
}

#[derive(Deserialize, Debug)]
pub struct RpcTxResult {
    pub rpc: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RpcTransactionSendResponse {
    pub success: bool,
    pub result: RpcTxResult,
}

// --------------------------------------------
// Token transfer 
// --------------------------------------------
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransferRequest {
    pub from_wallet: String,
    pub to_wallet: String,
    pub token_address: String,
    pub amount: u64,
}
#[derive(Deserialize, Debug)]
pub struct TokenTransferResponse {
    pub success: bool,
    pub transaction: String,
}

// --------------------------------------------
// Token creation 
// --------------------------------------------
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCreationMetadata {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub file: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCreation {
    pub metadata: TokenCreationMetadata,
    pub default_sol_amount: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCreationConfig {
    pub token_creation: TokenCreation,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokensCreateRequest {
    pub wallet_addresses: Vec<String>,
    pub mint_pubkey: String,
    pub config: TokenCreationConfig,
    pub amounts: Vec<f64>,
}

#[derive(Deserialize, Debug)]
pub struct TokensCreateResponse {
    pub success: bool,
    pub token_address: String,
    pub transactions: Vec<String>,
}

// --------------------------------------------
// Token burn 
// --------------------------------------------
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBurnRequest {
    pub wallet: String,
    pub token_address: String,
    pub amount: f64,
}
#[derive(Deserialize, Debug)]
pub struct TokenBurnResponse {
    pub success: bool,
    pub transaction: String,
}

// --------------------------------------------
// Token cleaner 
// --------------------------------------------
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCleanerRequest {
    pub wallet_addresses: Vec<String>,
    pub token_address: String,
    pub buy_amount: f64,
    pub hold_time: u64,
    pub sell_percentage: u64,
}

#[derive(Deserialize, Debug)]
pub struct TokenCleanerResponse {
    pub success: bool,
    pub transactions: Vec<String>,
}

// --------------------------------------------
// Error handling 
// --------------------------------------------
#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum FuryError {
    ApiError(ErrorResponse),
    RequestError(reqwest::Error),
    Other(anyhow::Error),
}

impl std::fmt::Display for FuryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuryError::ApiError(error) => {
                if let Some(details) = &error.details {
                    write!(f, "API error: {}: {}", 
                        error.error.as_deref().unwrap_or("Unknown error"), 
                        details)
                } else {
                    write!(f, "API error: {}", 
                        error.error.as_deref().unwrap_or("Unknown error"))
                }
            },
            FuryError::RequestError(e) => write!(f, "Request error: {}", e),
            FuryError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for FuryError {}

pub struct FurySDK {
    pub client: Client,
    pub base_url: String,
}

impl FurySDK {
    pub fn new(client: Client) -> Self {
        Self { client, base_url: "https://solana.fury.bot/api/".to_string() }
    }
    
    pub fn new_with_base_url(client: Client, base_url: &str) -> Self {
        Self { client, base_url: base_url.to_string() }
    }

    pub async fn buy_token(&self, data: &BuyTokenRequest) -> Result<BuyTokenResponse, FuryError> {
        self.send_request("tokens/buy", data).await
    }

    pub async fn sell_token(&self, data: &SellRequest) -> Result<SellResponse, FuryError> {
        self.send_request("tokens/sell", data).await
    }

    pub async fn jito_transaction_send(&self, data: &TransactionSendRequest) -> Result<JitoTransactionSendResponse, FuryError> {
        self.send_request("transactions/send", data).await
    }

    pub async fn rpc_transaction_send(&self, data: &TransactionSendRequest) -> Result<RpcTransactionSendResponse, FuryError> {
        self.send_request("transactions/send", data).await
    }

    pub async fn token_transfer(&self, data: &TokenTransferRequest) -> Result<TokenTransferResponse, FuryError> {
        self.send_request("tokens/transfer", data).await
    }

    pub async fn tokens_create(&self, data: &TokensCreateRequest) -> Result<TokensCreateResponse, FuryError> {
        self.send_request("tokens/create", data).await
    }

    pub async fn token_burn(&self, data: &TokenBurnRequest) -> Result<TokenBurnResponse, FuryError> {
        self.send_request("tokens/burn", data).await
    }

    pub async fn token_cleaner(&self, data: &TokenCleanerRequest) -> Result<TokenCleanerResponse, FuryError> {
        self.send_request("tokens/cleaner", data).await
    }

    // pub async fn 

    /// Generic method to send a request and handle the response
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - The API endpoint to call
    /// * `data` - The data to send in the request body
    /// 
    /// # Returns
    /// 
    /// * `Ok(T)` - The successful response deserialized to type T
    /// * `Err(FuryError)` - An error occurred
    async fn send_request<T, D>(&self, endpoint: &str, data: &D) -> Result<T, FuryError> 
    where
        T: for<'de> Deserialize<'de>,
        D: Serialize,
    {
        let response = match self.client.post(&format!("{}{}", self.base_url, endpoint))
            .json(data)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(FuryError::RequestError(e)),
            };

        if response.status().is_success() {
            // Deserialize successful response
            match response.json().await {
                Ok(body) => Ok(body),
                Err(e) => Err(FuryError::RequestError(e)),
            }
        } else {
            // Try to deserialize error response
            match response.json().await {
                Ok(error) => Err(FuryError::ApiError(error)),
                Err(e) => Err(FuryError::RequestError(e)),
            }
        }
    }
}
