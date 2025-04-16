use std::{collections::HashMap, fmt::{self, Display}, str::FromStr};
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
// Analytics PNL
// --------------------------------------------

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsPnlRequest {
    pub wallet_address: String,
    pub token_address: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize, Debug)]
pub struct AnalyticsPnlResponse {
    pub success: bool,
    pub pnl: f64,
    pub details: String,
}


// --------------------------------------------
// Analytics Usage Stats
// --------------------------------------------
pub enum AnalyticsUsagePeriod {
    Day,
    Week,
    Month,
    Year,
}

impl ToString for AnalyticsUsagePeriod {
    fn to_string(&self) -> String {
        match self {
            AnalyticsUsagePeriod::Day => "day".to_string(),
            AnalyticsUsagePeriod::Week => "week".to_string(),
            AnalyticsUsagePeriod::Month => "month".to_string(),
            AnalyticsUsagePeriod::Year => "year".to_string(),
        }
    }
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsUsageStatsResponse {
    pub success: bool,
    pub data: AnalyticsUsageStatsData,
}

#[derive(Deserialize, Debug)]
pub struct AnalyticsUsageStatsData {
    pub total_requests: u64,
    pub avg_response_time: f64,
    pub successful_requests: u64,
    pub client_errors: u64,
    pub server_errors: u64,
}


// --------------------------------------------
// Analytics Usage Endpoints
// --------------------------------------------
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsUsageEndpointsResponse {
    pub success: bool,
    pub data: Vec<AnalyticsUsageEndpointsData>,
}

#[derive(Deserialize, Debug)]
pub struct AnalyticsUsageEndpointsData {
    pub endpoint: String,
    pub request_count: u64,
    pub avg_response_time: f64,
    pub successful_requests: u64,
    pub error_requests: u64,
}


// --------------------------------------------
// Analytics Usage Services
// --------------------------------------------
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsUsageServicesResponse {
    pub success: bool,
    pub data: Vec<AnalyticsUsageServicesData>,
}

#[derive(Deserialize, Debug)]
pub struct AnalyticsUsageServicesData {
    pub service_id: String,
    pub service_type: String,
    pub usage_count: u64,
    pub avg_response_time: f64,
    pub successful_calls: u64,
    pub failed_calls: u64,
}

// -------------------------------------------- 
// Analytics Usage Daily
// --------------------------------------------
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsUsageDailyResponse {
    pub success: bool,
    pub data: Vec<AnalyticsUsageDailyData>,
}

#[derive(Deserialize, Debug)]
pub struct AnalyticsUsageDailyData {
    pub date: String,
    pub request_count: u64,
    pub successful_requests: u64,
    pub error_requests: u64,
}

// --------------------------------------------
// Health check 
// --------------------------------------------
#[derive(Deserialize, Debug)]
pub struct HealthCheckResponse {
    pub status: String,
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


#[derive(Debug, Default)]
struct RequestOptions {
    base_url: Option<String>,
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

    pub async fn health_check(&self) -> Result<HealthCheckResponse, FuryError> {
        let base_url = self.base_url.clone().replace("api/", "");
        self.send_get_request( "health", None,RequestOptions { base_url: Some(base_url) }).await
    }

    pub async fn buy_token(&self, data: &BuyTokenRequest) -> Result<BuyTokenResponse, FuryError> {
        self.send_post_request("tokens/buy", data, RequestOptions { base_url: None }).await
    }

    pub async fn sell_token(&self, data: &SellRequest) -> Result<SellResponse, FuryError> {
        self.send_post_request("tokens/sell", data, RequestOptions { base_url: None }).await
    }

    pub async fn jito_transaction_send(&self, data: &TransactionSendRequest) -> Result<JitoTransactionSendResponse, FuryError> {
        self.send_post_request("transactions/send", data, RequestOptions { base_url: None }).await
    }

    pub async fn rpc_transaction_send(&self, data: &TransactionSendRequest) -> Result<RpcTransactionSendResponse, FuryError> {
        self.send_post_request("transactions/send", data, RequestOptions { base_url: None }).await
    }

    pub async fn token_transfer(&self, data: &TokenTransferRequest) -> Result<TokenTransferResponse, FuryError> {
        self.send_post_request("tokens/transfer", data, RequestOptions { base_url: None }).await
    }

    pub async fn tokens_create(&self, data: &TokensCreateRequest) -> Result<TokensCreateResponse, FuryError> {
        self.send_post_request("tokens/create", data, RequestOptions { base_url: None }).await
    }

    pub async fn token_burn(&self, data: &TokenBurnRequest) -> Result<TokenBurnResponse, FuryError> {
        self.send_post_request("tokens/burn", data, RequestOptions { base_url: None }).await
    }

    pub async fn token_cleaner(&self, data: &TokenCleanerRequest) -> Result<TokenCleanerResponse, FuryError> {
        self.send_post_request("tokens/cleaner", data, RequestOptions { base_url: None }).await
    }

    pub async fn analytics_pnl(&self, data: &AnalyticsPnlRequest) -> Result<AnalyticsPnlResponse, FuryError> {
        self.send_post_request("analytics/pnl", data, RequestOptions { base_url: None }).await
    }

    pub async fn analytics_usage_stats(&self, period: &AnalyticsUsagePeriod) -> Result<AnalyticsUsageStatsResponse, FuryError> {
        let mut params = HashMap::new();
        params.insert("period".to_string(), period.to_string());
        self.send_get_request("analytics/usage/stats", Some(params), RequestOptions { base_url: None }).await
    }

    pub async fn analytics_usage_endpoints(&self, period: &AnalyticsUsagePeriod) -> Result<AnalyticsUsageEndpointsResponse, FuryError> {
        let mut params = HashMap::new();
        params.insert("period".to_string(), period.to_string());
        self.send_get_request("analytics/usage/endpoints", Some(params), RequestOptions { base_url: None }).await
    }

    pub async fn analytics_usage_services(&self, period: &AnalyticsUsagePeriod) -> Result<AnalyticsUsageServicesResponse, FuryError> {
        let mut params = HashMap::new();
        params.insert("period".to_string(), period.to_string());
        self.send_get_request("analytics/usage/services", Some(params), RequestOptions { base_url: None }).await
    }

    pub async fn analytics_usage_daily(&self, period: &AnalyticsUsagePeriod) -> Result<AnalyticsUsageDailyResponse, FuryError> {
        let mut params = HashMap::new();
        params.insert("period".to_string(), period.to_string());
        self.send_get_request("analytics/usage/daily", Some(params), RequestOptions { base_url: None }).await
    }

    /// Generic method to send a POST request and handle the response
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
    async fn send_post_request<T, D>(&self, endpoint: &str, data: &D, options: RequestOptions) -> Result<T, FuryError> 
    where
        T: for<'de> Deserialize<'de>,
        D: Serialize,
    {
        let base_url = options.base_url.unwrap_or(self.base_url.clone());
        let response = match self.client.post(&format!("{}{}", base_url, endpoint))
            .json(data)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(FuryError::RequestError(e)),
            };

        self.process_response(response).await
    }

    /// Generic method to send a POST request and handle the response
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
    async fn send_get_request<T>(&self, endpoint: &str, params: Option<HashMap<String, String>>, options: RequestOptions) -> Result<T, FuryError> 
    where
        T: for<'de> Deserialize<'de>,
    {
        let params = params.unwrap_or_default();
        let base_url = options.base_url.unwrap_or(self.base_url.clone());
        let response = match self.client.get(&format!("{}{}", base_url, endpoint))
            .query(&params)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(FuryError::RequestError(e)),
            };

        self.process_response(response).await
    }

    async fn process_response<T>(&self, response: reqwest::Response) -> Result<T, FuryError>
    where
        T: for<'de> Deserialize<'de>,
    {
        if response.status().is_success() {
            match response.json().await {
                Ok(body) => Ok(body),
                Err(e) => Err(FuryError::RequestError(e)),
            }
        } else {
            Err(FuryError::ApiError(response.json().await.unwrap()))
        }
    }
}
