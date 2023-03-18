// use serde::{Deserialize, Serialize};
// use serde_json::json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelList {
    data: Vec<Model>,
    object: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    id: String,
    object: String,
    owned_by: String,
    permission: PermissionList,
    root: String,
    parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    id: String,
    object: String,
    allow_create_engine: bool,
    allow_sampling: bool,
    allow_logprobs: bool,
    allow_search_indices: bool,
    allow_view: bool,
    allow_fine_tuning: bool,
    organization: String,
    group: Option<String>,
    is_blocking: bool,
}

/// #[serde(transparent)]
/// Serialize and deserialize a newtype struct or a braced struct with one field exactly the same as if its one field were serialized and deserialized by itself. Analogous to #[repr(transparent)].
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PermissionList {
    list: Vec<Permission>,
}
