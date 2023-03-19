use thiserror::Error;

pub type OpenAIResult<T> = Result<T, OpenAIError>;

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("Request returned with Status: {status_code} for {query_type} - {item_type}")]
    URLNotFound {
        status_code: String,
        query_type: String,
        item_type: String,
    },
    #[error("Unable to process request")]
    RequestError(#[from] reqwest::Error),
    #[error("Unable to parse response into valid JSON")]
    ParseError(#[from] serde_json::Error),
    #[error("Other Error happened")]
    OtherError,
}
