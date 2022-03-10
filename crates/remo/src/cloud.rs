mod appliance;
mod device;
pub mod enums;
mod error;
mod serialize;
mod user;

pub use appliance::*;
pub use device::*;
pub use error::APIError;
pub use user::*;

use reqwest::ClientBuilder;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use url::Url;

pub type RequestBody<'a> = HashMap<&'a str, &'a str>;

pub struct Client {
    token: Option<String>,
    base_url: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new(token: Option<String>) -> Self {
        Client {
            token,
            base_url: Url::parse("https://api.nature.global").unwrap(),
            client: ClientBuilder::new().build().unwrap(),
        }
    }
}

impl Client {
    async fn get<T>(&self, path: &str) -> Result<T, APIError>
    where
        T: DeserializeOwned,
    {
        self.make_request::<T>(path, None).await
    }

    async fn post<T>(&self, path: &str, body: &RequestBody<'_>) -> Result<T, APIError>
    where
        T: DeserializeOwned,
    {
        self.make_request::<T>(path, Some(body)).await
    }

    async fn make_request<T>(
        &self,
        path: &str,
        body: Option<&RequestBody<'_>>,
    ) -> Result<T, APIError>
    where
        T: DeserializeOwned,
    {
        let request_url = self.base_url.join(path).unwrap();
        let client = match body {
            Some(body) => self.client.post(request_url.as_str()).form(body),
            None => self.client.get(request_url.as_str()),
        };
        let resp = match &self.token {
            None => client.send().await?,
            Some(token) => {
                client
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?
            }
        };
        match resp.status() {
            reqwest::StatusCode::OK => Ok(resp.json().await?),
            reqwest::StatusCode::UNAUTHORIZED => Err(APIError::AuthError),
            code => Err(APIError::Error {
                error: format!("Returned with non-200 code: {}", code),
            }),
        }
    }
}
