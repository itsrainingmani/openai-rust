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

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u16,
    pub completion_tokens: u16,
    pub total_tokens: u16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionParams {
    pub model: String,
    pub prompt: String,
    pub max_tokens: usize,
    // #[serde(serialize_with = "float_to_usize")]
    pub temperature: f32,
    // #[serde(flatten)]
    // pub opts: OptParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptParams {
    pub suffix: Option<String>,
    pub top_p: f32,
    pub n: usize,
    pub stream: bool,
    pub logprobs: Option<usize>,
    pub echo: bool,
    pub stop: Option<String>,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub best_of: usize,
    pub user: Option<String>,
}

impl Default for OptParams {
    fn default() -> Self {
        Self {
            suffix: None,
            top_p: 1.0,
            n: 1,
            stream: false,
            logprobs: None,
            echo: false,
            stop: None,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            best_of: 1,
            user: None,
        }
    }
}
