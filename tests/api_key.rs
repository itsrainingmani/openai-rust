use openai_rust;

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
    common::setup();
    let key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let client = openai_rust::Client::new(key);
    assert_ne!(client.config.openai_secret_key.len(), 0);
}

#[test]
fn test_models() {
    common::setup();

    let key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = openai_rust::Client::new(key);

    println!("{:?}", client.get_models().unwrap());
}
