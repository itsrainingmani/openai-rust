use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use http::request::Builder as RequestBuilder;
use http::Response;
use std::borrow::Cow;
use url::Url;

use http::Method;

use crate::{
    api::{ApiError, UrlBase},
    error::OpenAIError,
};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// A trait representing a client which can communicate with a GitLab instance via REST.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for a REST v4 endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    /// Get the URL for an instance endpoint for the client.
    fn instance_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        let _ = endpoint;
        Err(ApiError::unsupported_url_base(UrlBase::Instance))
    }
}

/// A trait representing an asynchronous client which can communicate with OpenAI.
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

trait Endpoint {
    fn method(&self) -> Method;
    fn endpoint(&self) -> Cow<'static, str>;
    fn parameters(&self) -> QueryParams {
        // Most of the endpoints should have parameters so this may not be the right choice
        QueryParams::default()
    }
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        // Many endpoints also do not have request bodies
        Ok(None)
    }
}

trait Client: RestClient {
    type Error: Error + Send + Sync + 'static;
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, OpenAIError>;
    fn rest(&self, request: RequestBuilder, body: Vec<u8>) -> Result<Response<Bytes>, OpenAIError>;
}

trait Query<T, C> {
    fn query(&self, client: &C) -> Result<T, ApiError<C: Error>>;
}

pub struct Ignore<E> {
    endpoint: E,
}

impl<E, C> Query<(), C> for Ignore<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        // as in `impl Query for Endpoint` except we throw away the response data
    }
}

pub struct Raw<E> {
    endpoint: E,
}

impl<E, C> Query<Vec<u8>, C> for Raw<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
        // as in `impl Query for Endpoint` except the response data is returned directly (errors are still JSON objects)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub openai_secret_key: String,
    pub openai_org: Option<String>,
}
