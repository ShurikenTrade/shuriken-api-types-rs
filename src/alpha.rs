use serde::{Deserialize, Serialize};

use crate::signal::{FeedTokenMeta, FeedTokenSignal};
use crate::Network;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageToken {
    pub address: String,
    pub chain: String,
}

/// Platform-tagged message event. Each variant carries only the fields
/// meaningful to its platform — Discord callers don't see Telegram-only fields
/// and vice versa.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "platform", content = "data", rename_all = "camelCase")]
pub enum MessageEvent {
    #[serde(rename = "discord")]
    Discord(DiscordMessageEvent),
    #[serde(rename = "telegram")]
    Telegram(TelegramMessageEvent),
    #[serde(rename = "x")]
    X(XMessageEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordMessageEvent {
    pub message_id: String,
    pub guild_id: String,
    pub channel_id: String,
    /// Server name at ingest time. Frozen — does not track renames.
    /// For live names, resolve guild_id via the alpha-ui registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_guild_name: Option<String>,
    /// Channel name at ingest time. Frozen — does not track renames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_channel_name: Option<String>,
    pub content: String,
    #[serde(rename = "timestamp")]
    pub timestamp_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<MessageAuthor>,
    pub tokens: Vec<MessageToken>,
    pub is_edited: bool,
    pub is_deleted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelegramMessageEvent {
    pub message_id: String,
    pub chat_id: String,
    /// Chat title at ingest time. Frozen — does not track renames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_chat_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i32>,
    /// Forum topic title (megagroups with forum=true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_title: Option<String>,
    pub content: String,
    #[serde(rename = "timestamp")]
    pub timestamp_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<MessageAuthor>,
    pub tokens: Vec<MessageToken>,
    pub is_edited: bool,
    pub is_deleted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XMessageEvent {
    pub tweet_id: String,
    pub content: String,
    #[serde(rename = "timestamp")]
    pub timestamp_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<XAuthor>,
    pub tokens: Vec<MessageToken>,
    pub hashtags: Vec<String>,
    pub cashtags: Vec<String>,
    pub mentions: Vec<XMention>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<XPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_user_id: Option<String>,
    pub media_urls: Vec<String>,
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XAuthor {
    pub user_id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub verified: bool,
    pub followers_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XMention {
    pub username: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XPublicMetrics {
    pub like_count: u32,
    pub retweet_count: u32,
    pub reply_count: u32,
    pub quote_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalFeedUpdateEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub token_address: String,
    pub network: Network,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_meta: Option<FeedTokenMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_signal: Option<FeedTokenSignal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedMessageEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub feed_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MessageEvent>,
}
