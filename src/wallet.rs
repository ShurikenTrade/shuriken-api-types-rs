use serde::{Deserialize, Serialize};

// ── SVM ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvmNativeBalanceEvent {
  pub owner: String,
  pub slot: u64,
  pub block_time: i64,
  pub pre_balance: u64,
  pub post_balance: u64,
  pub network: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvmTokenBalanceEvent {
  pub mint: String,
  pub owner: String,
  pub slot: u64,
  pub block_time: i64,
  pub decimals: u32,
  pub pre_balance: u64,
  pub post_balance: u64,
  pub network: String,
}

// ── EVM ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmNativeBalanceEvent {
  pub owner: String,
  pub chain_id: u64,
  pub block_number: u64,
  pub block_time: i64,
  pub balance: String,
  pub network: String,
}
