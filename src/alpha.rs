use serde::{Deserialize, Serialize};

use crate::signal::{FeedTokenMeta, FeedTokenSignal};
use crate::{Network, Platform};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEvent {
    pub message_id: String,
    pub channel_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// Channel name at ingest time. Frozen — does not track renames.
    /// For live names, resolve channel_id via the alpha-ui registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_channel_name: Option<String>,
    /// Guild name at ingest time (Discord only). Frozen — does not track renames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_guild_name: Option<String>,
    pub platform: Platform,
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
    /// Telegram forum topic title (megagroups with forum=true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_title: Option<String>,
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
