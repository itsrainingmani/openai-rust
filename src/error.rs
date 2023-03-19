use thiserror::Error;

pub type OpenAIResult<T> = Result<T, OpenAIError>;

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("Request returned with Status: {status_code} for {message} \n URL: {url}")]
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
