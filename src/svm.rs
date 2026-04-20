use serde::{Deserialize, Serialize};

use crate::Network;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
    pub token_mint: String,
    pub signature: String,
    pub slot: u64,
    pub block_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_index: Option<u32>,
    pub is_buy: bool,
    pub size_sol: String,
    pub size_usd: String,
    pub price_usd: String,
    pub price_sol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_decimals: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_decimals: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<SwapFees>,
    pub network: Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapFees {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_units_consumed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fee_lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mev_fee_lamports: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPoolEvent {
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimals: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
    pub block_index: u64,
    pub timestamp_updated_ms: u64,
    pub network: Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BondingCurveCreationEvent {
    pub token_address: String,
    pub curve_address: String,
    pub curve_dex_type: String,
    pub creator: String,
    pub network: Network,
    pub creation_timestamp: i64,
    pub block_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BondingCurveGraduationEvent {
    pub token_address: String,
    pub curve_address: String,
    pub curve_dex_type: String,
    pub dest_pool_address: String,
    pub dest_pool_dex_type: String,
    pub network: Network,
    pub graduation_timestamp: i64,
    pub block_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceEvent {
    pub mint: String,
    pub owner: String,
    pub slot: u64,
    pub block_time: i64,
    pub decimals: u32,
    pub pre_balance: u64,
    pub post_balance: u64,
    pub network: Network,
}
