use serde::{Deserialize, Serialize};

/// Network identifier for API payloads.
///
/// Discriminant values match the proto `NetworkId` enum from `schemas-rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[repr(i32)]
pub enum Network {
    Eth = 0,
    Base = 1,
    Blast = 2,
    Avax = 3,
    Bsc = 4,
    Trx = 5,
    Sol = 6,
    Monad = 7,
    Hype = 8,
}

impl Network {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eth => "eth",
            Self::Base => "base",
            Self::Blast => "blast",
            Self::Avax => "avax",
            Self::Bsc => "bsc",
            Self::Trx => "trx",
            Self::Sol => "sol",
            Self::Monad => "monad",
            Self::Hype => "hype",
        }
    }
}

impl TryFrom<i32> for Network {
    type Error = UnknownNetworkError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Eth),
            1 => Ok(Self::Base),
            2 => Ok(Self::Blast),
            3 => Ok(Self::Avax),
            4 => Ok(Self::Bsc),
            5 => Ok(Self::Trx),
            6 => Ok(Self::Sol),
            7 => Ok(Self::Monad),
            8 => Ok(Self::Hype),
            _ => Err(UnknownNetworkError(value)),
        }
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct UnknownNetworkError(pub i32);

impl std::fmt::Display for UnknownNetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown network id: {}", self.0)
    }
}

impl std::error::Error for UnknownNetworkError {}
