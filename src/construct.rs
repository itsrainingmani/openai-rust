// use serde_json::json;

use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelList {
    pub data: Vec<Model>,
    pub object: String,
}

/// Model
/// {
/// "id": "model-id-0",
/// "object": "model",
/// "owned_by": "organization-owner",
/// "permission": [...]
/// }
#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub id: String,
    pub object: String,
    #[serde(with = "ts_seconds_option")]
    pub created: Option<DateTime<Utc>>,
    pub owned_by: String,
    pub permission: Vec<Permission>,
    root: String,
    parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    pub id: String,
    pub object: String,
    #[serde(with = "ts_seconds_option")]
    pub created: Option<DateTime<Utc>>,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(transparent)]
// pub struct PermissionList {
//     pub list: Vec<Permission>,
// }
