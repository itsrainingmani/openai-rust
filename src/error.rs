use reqwest::Error as ReqError;
use serde_json::Error as SerdeError;

pub type AIResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Network(ReqError),
    Parse(SerdeError),
    RateLimit(f32), // TODO: Figure out how to return this error
}

impl From<ReqError> for Error {
    fn from(e: ReqError) -> Self {
        // Strip the URL
        Error::Network(e.without_url())
    }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Self {
        Error::Parse(e)
    }
}
