use openai_rust::{self, APIClient, APIMethods};

mod common;

#[test]
fn test_config_org_and_key_validation() {
    let org = String::from("orgstring");
    let key = String::from("keystring");

    let client = openai_rust::Client::new_with_org(key, org);
    assert_eq!(client.config.openai_secret_key, String::from("keystring"));
    assert_eq!(client.config.openai_org, Some(String::from("orgstring")));
}

#[test]
fn test_config_from_env() {
    let client = common::setup();
    assert_ne!(client.config.openai_secret_key.len(), 0);
}

#[test]
fn test_list_of_models() {
    let client = common::setup();
    let resp = client.get_models();

    // println!("{:?}", resp.unwrap());

    assert!(resp.is_ok());
}

#[test]
fn test_get_existing_model() {
    let client = common::setup();
    let resp = client.get_model_info(String::from("text-davinci-003"));

    assert!(resp.is_ok());
}

#[test]
fn test_invalid_model() {
    let client = common::setup();
    let resp = client.get_model_info(String::from("chatgpt"));

    // println!("{}", resp.unwrap_err().to_string());

    assert!(resp.is_err());
    assert_eq!("Internal API Error: [404 Not Found] | [The model 'chatgpt' does not exist] \n [invalid_request_error]", resp.unwrap_err().to_string())
}

#[test]
fn test_create_completion() {
    let client = common::setup();
    let completion_params: openai_rust::param::CompletionParams =
        openai_rust::param::CompletionParams {
            model: String::from("text-davinci-003"),
            prompt: String::from("Say this is a test"),
            opts: openai_rust::param::OptParams::default(),
        };

    let resp = client.create_completion(completion_params);
    assert!(resp.is_ok());
}
