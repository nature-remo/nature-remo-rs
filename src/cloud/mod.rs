extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

mod appliance;
mod device;
mod error;
mod user;

pub use appliance::*;
pub use device::*;
pub use error::APIError;
use reqwest::ClientBuilder;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use url::Url;
pub use user::*;

pub type RequestBody<'a> = HashMap<&'a str, &'a str>;

pub struct Client {
  token: Option<String>,
  base_url: Url,
  client: reqwest::Client,
}

impl Client {
  pub fn new(token: Option<String>) -> Self {
    Client {
      token: token,
      base_url: Url::parse("https://api.nature.global").unwrap(),
      client: ClientBuilder::new().build().unwrap(),
    }
  }
}

impl Client {
  fn get<T>(&self, path: &str) -> Result<T, APIError>
  where
    T: DeserializeOwned,
  {
    self.make_request::<T>(path, None)
  }

  fn post<T>(&self, path: &str, body: &RequestBody) -> Result<T, APIError>
  where
    T: DeserializeOwned,
  {
    self.make_request::<T>(path, Some(body))
  }

  fn make_request<T>(&self, path: &str, body: Option<&RequestBody>) -> Result<T, APIError>
  where
    T: DeserializeOwned,
  {
    let request_url = self.base_url.join(path).unwrap();
    let client = match body {
      Some(body) => self.client.post(request_url.as_str()).form(body),
      None => self.client.get(request_url.as_str()),
    };
    let mut resp = match &self.token {
      None => client.send()?,
      Some(token) => client
        .header("Authorization", format!("Bearer {}", token))
        .send()?,
    };
    match resp.status() {
      reqwest::StatusCode::OK => Ok(resp.json()?),
      reqwest::StatusCode::UNAUTHORIZED => Err(APIError::AuthError),
      code => Err(APIError::Error {
        error: format!("Returned with non-200 code: {}", code),
      }),
    }
  }
}
