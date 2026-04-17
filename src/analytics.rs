use serde::{Deserialize, Serialize};

use crate::Network;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDistributionStatsEvent {
  pub token_address: String,
  pub network: Network,
  pub block_index: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stats: Option<TokenDistributionStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDistributionStats {
  pub holder_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub developer: Option<String>,
  pub developer_balance: String,
  pub top_10_holder_balances: String,
  pub top_10_holders: Vec<String>,
  pub sniper_balances: String,
  pub pro_trader_balances: String,
  pub sniper_count: i64,
  pub pro_trader_count: i64,
  pub freshie_balances: String,
  pub freshie_count: i64,
  pub insider_balances: String,
  pub insider_count: i64,
  pub bundler_balances: String,
  pub bundler_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderStatsEvent {
  pub token_address: String,
  pub network: Network,
  pub block_index: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stats: Option<HolderStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderStats {
  pub bought_usd: f64,
  pub bought_native: f64,
  pub bought_tokens: String,
  pub sold_usd: f64,
  pub sold_native: f64,
  pub sold_tokens: String,
  pub buys_count: i64,
  pub sells_count: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub first_transaction_ms: Option<i64>,
  pub is_token_dev: bool,
  pub token_balance: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub avg_buy_price_usd: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub avg_sell_price_usd: Option<f64>,
}
