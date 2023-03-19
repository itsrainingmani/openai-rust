use openai_rust::{
    self,
    construct::{CompletionParams, OptParams, Usage},
};

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
fn test_completion_serialization() {
    let completion_params: CompletionParams = CompletionParams {
        model: String::from("text-davinci-003"),
        prompt: String::from("Say this is a test"),
        max_tokens: 7,
        temperature: 0.0,
        // opts: OptParams::default(),
    };

    let completion_serialized = serde_json::to_string(&completion_params).unwrap();

    let param_json = r#"{"model":"text-davinci-003","prompt":"Say this is a test","max_tokens":7,"temperature":0.0,"suffix":null,"top_p":1.0,"n":1,"stream":false,"logprobs":null,"echo":false,"stop":null,"presence_penalty":0.0,"frequency_penalty":0.0,"best_of":1,"user":null}"#;

    // println!("{}", params);
    assert_eq!(completion_serialized, param_json);
}

#[test]
fn test_optional_param_default_serialization() {
    let opt_params: OptParams = OptParams::default();

    let opt_serialized = serde_json::to_string(&opt_params).unwrap();

    let opt_json = r#"{"suffix":null,"top_p":1.0,"n":1,"stream":false,"logprobs":null,"echo":false,"stop":null,"presence_penalty":0.0,"frequency_penalty":0.0,"best_of":1,"user":null}"#;

    assert_eq!(opt_serialized, opt_json);
}
