use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u16,
    pub completion_tokens: u16,
    pub total_tokens: u16,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<usize>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct Completion {
    pub id: String,
    pub object: String,
    #[serde(with = "ts_seconds_option")]
    pub created: Option<DateTime<Utc>>,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

// TODO
#[derive(Deserialize, Debug)]
pub struct ChatCompletion {}

// TODO
#[derive(Deserialize, Debug)]
pub struct EditedPrompt {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_deserialization() {
        let usage_data = r#"
    {
        "prompt_tokens": 5,
        "completion_tokens": 7,
        "total_tokens": 12
    }
        "#;

        let deserialized_usage_data: Usage = serde_json::from_str(usage_data).unwrap();

        assert_eq!(
            vec![
                deserialized_usage_data.prompt_tokens,
                deserialized_usage_data.completion_tokens,
                deserialized_usage_data.total_tokens
            ],
            vec![5, 7, 12]
        );
    }

    #[test]
    fn test_completion_deserialization() {
        let completion_data = r#"
        {
            "id": "cmpl-6wNHLLAa4l0GkmSZHb3Y9wM9G6IWS",
            "object": "text_completion",
            "created": 1679370527,
            "model": "text-davinci-003",
            "choices": [
                {
                    "text": "\n\nThis is indeed a test",
                    "index": 0,
                    "logprobs": null,
                    "finish_reason": "length"
                }
            ],
            "usage": {
                "prompt_tokens": 5,
                "completion_tokens": 7,
                "total_tokens": 12
            }
        }"#;

        let deserialized_completion_data: Completion =
            serde_json::from_str(completion_data).unwrap();

        assert_eq!(
            deserialized_completion_data.choices.get(0).unwrap().text,
            "\n\nThis is indeed a test"
        );
    }
}
