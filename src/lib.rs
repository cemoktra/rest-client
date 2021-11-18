use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

pub trait IntoBody {
    fn content_type() -> Option<(reqwest::header::HeaderName, reqwest::header::HeaderValue)> {
        None
    }

    fn content(&self) -> reqwest::Body {
        reqwest::Body::from(String::new())
    }
}

impl<T: Serialize> IntoBody for T {
    fn content(&self) -> reqwest::Body {
        reqwest::Body::from(serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug)]
pub enum Error<E> {
    RequestFailed(reqwest::Error),
    ErrorStatus(E),
    ParseError
}

impl<E> std::convert::From<reqwest::Error> for Error<E> {
    fn from(e: reqwest::Error) -> Self {
        Error::RequestFailed(e)
    }
}

#[async_trait]
pub trait FromBody {
    async fn from_body(response: reqwest::Response) -> Result<Self, Error<()>> where Self: Sized;
}

#[async_trait]
impl<T: DeserializeOwned> FromBody for T {
    async fn from_body(response: reqwest::Response) -> Result<Self, Error<()>> {
        Ok(response.json::<T>().await?)
    }
}

#[async_trait]
pub trait Endpoint where Error<<Self as Endpoint>::ErrorBody>: From<Error<()>> {
    type RequestBody: std::default::Default + IntoBody;
    type ResponseBody: FromBody;
    type ErrorBody: FromBody;

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn endpoint(&self) -> &str {
        "/"
    }

    fn body(&self) -> Self::RequestBody {
        Self::RequestBody::default()
    }

    async fn call(&self, base_url: &reqwest::Url) -> Result<Self::ResponseBody, Error<Self::ErrorBody>> {
        let client = reqwest::Client::new();
        let url = format!("{}{}", base_url, self.endpoint());
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(Self::ResponseBody::from_body(response).await?)
                } else {
                    Err(Error::ErrorStatus(Self::ErrorBody::from_body(response).await?))
                }
            },
            Err(error) => {
                Err(error.into())
            },
        }
    }
}