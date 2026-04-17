use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Discord,
    Telegram,
    X,
    Unknown,
}

impl Platform {
    pub fn from_proto_i32(value: i32) -> Self {
        match value {
            1 => Self::Discord,
            2 => Self::Telegram,
            3 => Self::X,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Discord => "discord",
            Self::Telegram => "telegram",
            Self::X => "x",
            Self::Unknown => "unknown",
        }
    }
}
