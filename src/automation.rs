use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum AutomationEvent {
    #[serde(rename = "taskUpdate")]
    TaskUpdate { task_id: String, user_id: String },
    #[serde(rename = "regionalStats")]
    RegionalStats {
        task_id: String,
        region_id: String,
        user_id: String,
    },
}
