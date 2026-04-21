use serde::{Deserialize, Serialize};

use crate::Network;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedTokenMeta {
    pub address: String,
    pub network: Network,
    pub symbol: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordSignalSource {
    pub message_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelegramSignalSource {
    pub message_id: String,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_title: Option<String>,
    pub sender_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XSignalSource {
    pub tweet_id: String,
    pub author_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeSignalSource {
    pub wallet_address: String,
    pub tx_signature: String,
    pub is_buy: bool,
    pub amount_usd: String,
    pub amount_native: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "sourceType", content = "source")]
pub enum SignalSource {
    #[serde(rename = "discord")]
    Discord(DiscordSignalSource),
    #[serde(rename = "telegram")]
    Telegram(TelegramSignalSource),
    #[serde(rename = "x")]
    X(XSignalSource),
    #[serde(rename = "trade")]
    Trade(TradeSignalSource),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedTokenSignal {
    pub id: String,
    pub timestamp_ms: i64,
    pub source: SignalSource,
    pub is_bot: bool,
    pub price_usd: f64,
    pub price_native: f64,
    pub marketcap_usd: f64,
    pub liquidity_usd: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidity_native: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex_name: Option<String>,
}
