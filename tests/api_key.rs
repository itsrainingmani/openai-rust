use openai_rust::{self, open_ai_api};

mod common;

#[test]
fn test_config_org_and_key_validation() {
    common::setup();
    let org = String::from("orgstring");
    let key = String::from("keystring");

    let client = open_ai_api::Client::new_with_org(key, org);
    assert_eq!(client.config.key, String::from("keystring"));
    assert_eq!(client.config.organization, Some(String::from("orgstring")));
}
