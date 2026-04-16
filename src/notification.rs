use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum NotificationEvent {
  #[serde(rename = "swap")]
  Swap {
    task_id: String,
    user_id: String,
    chain_id: i32,
  },
  #[serde(rename = "transfer")]
  Transfer {
    task_id: String,
    user_id: String,
  },
  #[serde(rename = "approval")]
  Approval {
    task_id: String,
    user_id: String,
  },
  #[serde(rename = "automation")]
  Automation {
    task_id: String,
    user_id: String,
  },
  #[serde(rename = "strategy")]
  Strategy {
    root_task_id: String,
    user_id: String,
  },
  #[serde(rename = "claim")]
  Claim {
    claim_id: String,
    user_id: String,
  },
  #[serde(rename = "cleanup")]
  Cleanup {
    task_id: String,
    user_id: String,
  },
  #[serde(rename = "svmNonce")]
  SvmNonce {
    task_id: String,
    user_id: String,
  },
  #[serde(rename = "crosschainSwap")]
  CrosschainSwap {
    task_id: String,
    user_id: String,
  },
  #[serde(rename = "perpsOrder")]
  PerpsOrder {
    task_id: String,
    user_id: String,
  },
}
