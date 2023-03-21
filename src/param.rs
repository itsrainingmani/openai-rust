use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionParams {
    pub model: String,
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    #[serde(flatten)]
    pub opts: OptParams,
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
    pub user: String,
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
            user: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_serialization() {
        let completion_params: CompletionParams = CompletionParams {
            model: String::from("text-davinci-003"),
            prompt: String::from("Say this is a test"),
            max_tokens: 7,
            temperature: 0.0,
            opts: OptParams::default(),
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

        let opt_json = r#"{"suffix":null,"top_p":1.0,"n":1,"stream":false,"logprobs":null,"echo":false,"stop":null,"presence_penalty":0.0,"frequency_penalty":0.0,"best_of":1,"user":""}"#;

        assert_eq!(opt_serialized, opt_json);
    }
}
