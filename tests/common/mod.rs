use crate::openai_rust;
use dotenv::dotenv;
use openai_rust::Config;

pub fn setup() -> openai_rust::Client {
    // This line loads the environment variables from the ".env" file.
    dotenv().ok();

    // Construct a simple client using the key
    let key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    openai_rust::Client::new(Config {
        openai_secret_key: key.clone(),
        ..Default::default()
    })
}
