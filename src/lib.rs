pub mod construct;
pub mod error;
pub mod param;

use construct::{Completion, Model, ModelList};
use error::{OpenAIError, OpenAIResult};
use param::CompletionParams;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    ClientBuilder, StatusCode,
};

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
    ///
    /// This function will panic if there isn't a valid TLS Backend / Resolver cannot load system config
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
            .expect("Expected a valid TLS Backend / Resolver");

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

    /// Lists the currently available models, and provides basic information about each one such as the owner and availability.
    ///
    /// The Model vector is accessible through the "data" field
    ///
    /// # Errors
    ///
    /// This function will return an error if -
    /// * _the requested endpoint is not available_
    /// * _deserialization of JSON response data fails_
    #[tokio::main]
    pub async fn get_models(&self) -> OpenAIResult<ModelList> {
        let model_url = String::from("https://api.openai.com/v1/models");

        let resp = self.http_client.get(model_url.clone()).send().await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<ModelList>().await?),
            _ => Err(OpenAIError::APIError {
                status_code: resp.status().to_string(),
                message: String::from("Model List"),
                url: model_url,
            }),
        }
    }

    /// Retrieves a model instance, providing basic information about the model such as the owner and permissioning
    ///
    /// # Errors
    ///
    /// This function will return an error if -
    /// * _the requested model doesn't exist_
    /// * _endpoint is unavailable_
    /// * _deserialization of JSON Model data fails_
    #[tokio::main]
    pub async fn get_model_info(&self, model: String) -> OpenAIResult<Model> {
        let model_url = format!("https://api.openai.com/v1/models/{}", model);

        // Break out Response into the sending request part and the parsing Json part
        let resp = self.http_client.get(model_url.clone()).send().await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Model>().await?),
            _ => Err(OpenAIError::APIError {
                status_code: resp.status().to_string(),
                message: format!("Model - {}", model),
                url: model_url,
            }),
        }
    }

    #[tokio::main]
    pub async fn create_completion(
        &self,
        completion_params: CompletionParams,
    ) -> OpenAIResult<Completion> {
        let completion_url = String::from("https://api.openai.com/v1/completions");

        let completion_body = serde_json::to_string(&completion_params)?;

        let resp = self
            .http_client
            .post(completion_url.clone())
            .body(completion_body)
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<Completion>().await?),
            _ => Err(OpenAIError::APIError {
                status_code: resp.status().to_string(),
                message: format!("Completion"),
                url: completion_url,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let key = String::from("keystring");
        let client = Client::new(key);

        let test_config = Config {
            openai_org: None,
            openai_secret_key: String::from("keystring"),
        };

        assert_eq!(
            test_config.openai_secret_key,
            client.config.openai_secret_key
        );
    }
}
