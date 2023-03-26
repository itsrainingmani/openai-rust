use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionParams {
    pub model: String,
    pub prompt: String,
    #[serde(flatten)]
    pub opts: OptParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptParams {
    pub max_tokens: usize,
    pub temperature: f32,
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
    pub user: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatParams {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(flatten)]
    pub opt: OptChatParams,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    User,
    System,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptChatParams {
    pub temperature: f32,
    pub top_p: f32,
    pub n: usize,
    pub stream: bool,
    pub stop: Option<String>,
    pub max_tokens: Option<u64>,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub user: String,
}

/// Only use text-davinci-edit-001 or code-davinci-edit-001 models with this endpoint
#[derive(Serialize, Deserialize, Debug)]
pub struct EditParams {
    pub model: String,
    pub input: String,
    pub instruction: String,

    #[serde(flatten)]
    pub opts: OptEditParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptEditParams {
    #[serde(rename = "n")]
    pub num_edits: usize,
    pub temperature: f32,
    pub top_p: usize,
}

impl Default for OptEditParams {
    fn default() -> Self {
        Self {
            num_edits: 1,
            temperature: 1.0,
            top_p: 1,
        }
    }
}

impl Default for OptParams {
    fn default() -> Self {
        Self {
            max_tokens: 16,
            temperature: 1.0,
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
            user: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_serialization() {
        let opt_params = OptParams {
            max_tokens: 7,
            temperature: 0.0,
            ..Default::default()
        };
        let completion_params: CompletionParams = CompletionParams {
            model: String::from("text-davinci-003"),
            prompt: String::from("Say this is a test"),
            opts: opt_params,
        };

        let completion_serialized = serde_json::to_string(&completion_params).unwrap();

        let param_json = r#"{"model":"text-davinci-003","prompt":"Say this is a test","max_tokens":7,"temperature":0.0,"suffix":null,"top_p":1.0,"n":1,"stream":false,"logprobs":null,"echo":false,"stop":null,"presence_penalty":0.0,"frequency_penalty":0.0,"best_of":1,"user":""}"#;

        // println!("{}", params);
        assert_eq!(completion_serialized, param_json);
    }

    #[test]
    fn test_optional_param_default_serialization() {
        let opt_params: OptParams = OptParams::default();

        let opt_serialized = serde_json::to_string(&opt_params).unwrap();

        let opt_json = r#"{"max_tokens":16,"temperature":1.0,"suffix":null,"top_p":1.0,"n":1,"stream":false,"logprobs":null,"echo":false,"stop":null,"presence_penalty":0.0,"frequency_penalty":0.0,"best_of":1,"user":""}"#;

        assert_eq!(opt_serialized, opt_json);
    }

    #[test]
    fn test_edit_params() {
        let opt_params: OptEditParams = OptEditParams::default();
        let edit_params: EditParams = EditParams {
            model: String::from("text-davinci-edit-001"),
            input: String::from("What day of the wek is it?"),
            instruction: String::from("Fix the spelling mistakes"),
            opts: opt_params,
        };

        let params_serialized = serde_json::to_string(&edit_params).unwrap();

        let params_json = r#"{"model":"text-davinci-edit-001","input":"What day of the wek is it?","instruction":"Fix the spelling mistakes","n":1,"temperature":1.0,"top_p":1}"#;

        assert_eq!(params_serialized, params_json);
    }
}
