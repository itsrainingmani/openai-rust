use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type OpenAIResult<T> = Result<T, OpenAIError>;

pub fn construct_error_msg(status_code: String, err_data: APIErrorData) -> String {
    format!(
        "[{}] | [{}] \n [{}]",
        status_code, err_data.message, err_data.kind
    )
}

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("Internal API Error: {0}")]
    InternalAPIError(String),
    #[error("Error: {0}")]
    AuthenticationError(String),
    #[error("Rate Limit Error: {0}")]
    RateLimitError(String),
    #[error("Server Error: {0}")]
    ServerError(String),
    #[error("Other Error: {0}")]
    OtherError(String),
    #[error("Unable to process request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Unable to parse response into valid JSON: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Unknown Error happened")]
    UnknownError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIError {
    pub error: APIErrorData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "APIError")]
pub struct APIErrorData {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

impl From<APIError> for APIErrorData {
    fn from(value: APIError) -> Self {
        APIErrorData {
            message: value.error.message.clone(),
            kind: value.error.kind.clone(),
            param: value.error.param.clone(),
            code: value.error.param.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_full() {
        let error_data = r#"
        {
            "error": {
                "message": "The model 'text-davinci-007' does not exist",
                "type": "invalid_request_error",
                "param": "model",
                "code": "model_not_found"
            }
        }
        "#;

        let de_error: APIErrorData = serde_json::from_str::<APIError>(error_data)
            .expect("Deserialize failed")
            .into();

        assert_eq!(
            de_error.message,
            String::from("The model 'text-davinci-007' does not exist")
        );
        assert_eq!(de_error.kind, String::from("invalid_request_error"));
        assert_eq!(de_error.code, Some(String::from("model")));
        assert_eq!(de_error.param, Some(String::from("model_not_found")));
    }

    #[test]
    fn test_api_error_partial() {
        let error_data = r#"
        {
            "error": {
                "message": "The model 'text-davinci-007' does not exist",
                "type": "invalid_request_error",
                "param": null,
                "code": null
            }
        }
        "#;

        let de_error: APIErrorData = serde_json::from_str::<APIError>(error_data)
            .expect("Deserialize failed")
            .into();

        assert_eq!(
            de_error.message,
            String::from("The model 'text-davinci-007' does not exist")
        );
        assert_eq!(de_error.kind, String::from("invalid_request_error"));
        assert_eq!(de_error.code, None);
        assert_eq!(de_error.param, None);
    }
}
