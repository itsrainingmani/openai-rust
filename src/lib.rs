use reqwest::{
    self,
    header::{self, HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    ClientBuilder,
};
use std::{collections::HashMap, fmt::Debug};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug)]
pub struct Config {
    pub openai_secret_key: String,
    pub openai_org: Option<String>,
}

#[derive(Debug)]
pub struct Client {
    pub config: Config,
    http_client: reqwest::Client,
}

impl Client {
    /// Creates a new Client given a secret API key
    pub fn new(key: String) -> Self {
        let mut headers = HeaderMap::new();
        let auth_token = format!("Bearer {}", key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(auth_token.as_str()).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        let config = Config {
            openai_secret_key: key,
            openai_org: None,
        };

        Client {
            config,
            http_client: client,
        }
    }

    /// Creates a new Client given an organization and the secret API key
    pub fn new_with_org(key: String, organization: String) -> Self {
        let mut cl = Client::new(key);
        cl.config.openai_org = Some(organization);
        cl
    }

    #[tokio::main]
    pub async fn get_models(&self) -> Result<String, Box<dyn std::error::Error>> {
        let model_url = String::from("https://api.openai.com/v1/models");
        Ok(self.http_client.get(model_url).send().await?.text().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let key = String::from("keystring");
        let client = Client::new(key);

        println!("{:?}", client);
    }

    #[test]
    fn test_auth_headers() {
        let client = Client::new(String::from("test-key"));
        println!("{:?}", client.http_client);
    }
}
