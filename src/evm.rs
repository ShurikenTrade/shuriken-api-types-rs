use serde::{Deserialize, Serialize};

use crate::Network;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
  pub token_address: String,
  pub tx_hash: String,
  pub chain_id: u64,
  pub block_number: u64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub block_hash: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tx_index: Option<u64>,
  pub timestamp: i64,
  pub is_buy: bool,
  pub amount_native: String,
  pub amount_usd: String,
  pub price_native: String,
  pub price_usd: String,
  pub token_decimals: u32,
  pub token_in_address: String,
  pub token_out_address: String,
  pub amount_in: String,
  pub amount_out: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pool_address: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub recipient: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub maker: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub inferred_dex_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub platform_name: Option<String>,
  pub network: Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPoolEvent {
  pub token_address: String,
  pub chain_id: u64,
  pub token_name: String,
  pub token_symbol: String,
  pub token_decimals: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub total_supply: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub price_usd: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub price_native: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub liquidity_usd: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub market_cap_usd: Option<String>,
  pub block_number: u64,
  pub timestamp_updated_ms: u64,
  pub network: Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceEvent {
  pub token_address: String,
  pub owner: String,
  pub chain_id: u64,
  pub block_number: u64,
  pub block_time: i64,
  pub balance: String,
  pub decimals: u32,
  pub network: Network,
}
