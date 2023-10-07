pub mod construct;
pub mod error;
pub mod param;

use construct::{ChatCompletion, Completion, EditedPrompt, Model, ModelList};
use error::{APIError, APIErrorData, OpenAIError, OpenAIResult};
use param::{ChatParams, CompletionParams, EditParams};
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    ClientBuilder, StatusCode,
};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

// trait APIClientBuilder {
//     fn api_url() -> String;
// }

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub openai_secret_key: String,
    pub openai_org: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub config: Config,
    http_client: reqwest::Client,
}

impl Client {
    /// Creates a new Client given a secret API key
    ///
    /// This function will panic if there isn't a valid TLS Backend / Resolver cannot load system config
    pub fn new(config: Config) -> Self {
        let mut headers = HeaderMap::new();
        let auth_token = format!("Bearer {}", config.openai_secret_key);
        headers.insert(AUTHORIZATION, auth_token.parse().unwrap());

        // Only add this if the user provides an Organization String
        if let Some(ref org_string) = config.openai_org {
            headers.insert("OpenAI-Organization", org_string.parse().unwrap());
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Expected a valid TLS Backend / Resolver");

        Client {
            config,
            http_client: client,
        }
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

        if resp.status() == StatusCode::OK {
            Ok(resp.json::<ModelList>().await?)
        } else {
            let err_code = resp.status();
            let err_data: APIErrorData = resp.json::<APIError>().await?.into();
            let err_msg = err_data.construct_error_msg(err_code.to_string().clone());

            match err_code {
                StatusCode::NOT_FOUND => Err(OpenAIError::InternalAPIError(err_msg)),
                StatusCode::UNAUTHORIZED => Err(OpenAIError::AuthenticationError(err_msg)),
                StatusCode::TOO_MANY_REQUESTS => Err(OpenAIError::RateLimitError(err_msg)),
                StatusCode::INTERNAL_SERVER_ERROR => Err(OpenAIError::ServerError(err_msg)),
                _ => Err(OpenAIError::OtherError(err_msg)),
            }
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

        if resp.status() == StatusCode::OK {
            Ok(resp.json::<Model>().await?)
        } else {
            let err_code = resp.status();
            let err_data: APIErrorData = resp.json::<APIError>().await?.into();
            let err_msg = err_data.construct_error_msg(err_code.to_string().clone());

            match err_code {
                StatusCode::NOT_FOUND => Err(OpenAIError::InternalAPIError(err_msg)),
                StatusCode::UNAUTHORIZED => Err(OpenAIError::AuthenticationError(err_msg)),
                StatusCode::TOO_MANY_REQUESTS => Err(OpenAIError::RateLimitError(err_msg)),
                StatusCode::INTERNAL_SERVER_ERROR => Err(OpenAIError::ServerError(err_msg)),
                _ => Err(OpenAIError::OtherError(err_msg)),
            }
        }
    }

    /// Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position.
    ///
    /// # Errors
    ///
    /// This function will return an error if -
    /// * _the requested model doesn't exist_
    /// * _endpoint is unavailable_
    /// * _deserialization of JSON Model data fails_
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

        if resp.status() == StatusCode::OK {
            Ok(resp.json::<Completion>().await?)
        } else {
            let err_code = resp.status();
            let err_data: APIErrorData = resp.json::<APIError>().await?.into();
            let err_msg = err_data.construct_error_msg(err_code.to_string().clone());

            match err_code {
                StatusCode::NOT_FOUND => Err(OpenAIError::InternalAPIError(err_msg)),
                StatusCode::UNAUTHORIZED => Err(OpenAIError::AuthenticationError(err_msg)),
                StatusCode::TOO_MANY_REQUESTS => Err(OpenAIError::RateLimitError(err_msg)),
                StatusCode::INTERNAL_SERVER_ERROR => Err(OpenAIError::ServerError(err_msg)),
                _ => Err(OpenAIError::OtherError(err_msg)),
            }
        }
    }

    #[tokio::main]
    pub async fn create_chat_completion(
        &self,
        _chat_params: ChatParams,
    ) -> OpenAIResult<ChatCompletion> {
        todo!()
    }

    #[tokio::main]
    pub async fn edit_prompt(&self, _edit_params: EditParams) -> OpenAIResult<EditedPrompt> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let key = String::from("keystring");
        let client = Client::new(Config {
            openai_secret_key: key.clone(),
            ..Default::default()
        });

        let test_config = Config {
            openai_secret_key: key,
            openai_org: None,
        };

        assert_eq!(
            test_config.openai_secret_key,
            client.config.openai_secret_key
        );
    }

    #[test]
    fn test_config_org_creation() {
        let key = String::from("keystring");
        let org = String::from("org-testing");
        let client = Client::new(Config {
            openai_secret_key: key.clone(),
            openai_org: Some(org.clone()),
        });

        let test_config = Config {
            openai_secret_key: key,
            openai_org: Some(org),
        };

        assert_eq!(test_config.openai_org, client.config.openai_org);
    }
}
