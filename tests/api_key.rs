use openai_rust::{self, open_ai_api};

mod common;

#[test]
fn test_config_org_and_key_validation() {
    let org = String::from("orgstring");
    let key = String::from("keystring");

    let client = open_ai_api::Client::new_with_org(key, org);
    assert_eq!(client.config.key, String::from("keystring"));
    assert_eq!(client.config.organization, Some(String::from("orgstring")));
}

#[test]
fn test_config_from_env() {
    common::setup();
    let key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let client = open_ai_api::Client::new(key);
    assert_ne!(client.config.key.len(), 0);
}

#[test]
fn test_authenticate() {
    common::setup();

    let key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = open_ai_api::Client::new(key);

    println!("{:?}", client.authenticate().unwrap());
}
