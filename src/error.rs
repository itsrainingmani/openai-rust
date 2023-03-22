use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type OpenAIResult<T> = Result<T, OpenAIError>;

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("Error -> Status: [{status_code}] | Message: [{message}] \n URL: [{url}]")]
    APIError {
        status_code: String,
        message: String,
        url: String,
    },
    #[error("Unable to process request")]
    RequestError(#[from] reqwest::Error),
    #[error("Unable to parse response into valid JSON")]
    ParseError(#[from] serde_json::Error),
    #[error("Other Error happened")]
    OtherError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIErrorData {
    pub error: APIErrorSubfields,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIErrorSubfields {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub param: Option<String>,
    pub code: Option<String>,
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

        let de_error: APIErrorData = serde_json::from_str(error_data).expect("Deserialize failed");

        assert_eq!(
            de_error.error.message,
            String::from("The model 'text-davinci-007' does not exist")
        );
        assert_eq!(de_error.error.kind, String::from("invalid_request_error"));
        assert_eq!(de_error.error.code, Some(String::from("model")));
        assert_eq!(de_error.error.param, Some(String::from("model_not_found")));
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

        let de_error: APIErrorData = serde_json::from_str(error_data).expect("Deserialize failed");

        assert_eq!(
            de_error.error.message,
            String::from("The model 'text-davinci-007' does not exist")
        );
        assert_eq!(de_error.error.kind, String::from("invalid_request_error"));
        assert_eq!(de_error.error.code, None);
        assert_eq!(de_error.error.param, None);
    }
}
