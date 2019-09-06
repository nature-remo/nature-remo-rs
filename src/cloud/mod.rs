extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

mod interfaces;

use failure::Fail;
pub use interfaces::*;
use reqwest::ClientBuilder;
use serde::de::DeserializeOwned;
use url::Url;

#[derive(Debug, Fail)]
pub enum APIError {
  #[fail(display = "Error: {:?}", error)]
  Error { error: String },

  #[fail(display = "Authorization failed")]
  AuthError,

  #[fail(display = "Client error: {:?}", error)]
  HTTPError { error: reqwest::Error },
}

impl From<reqwest::Error> for APIError {
  fn from(error: reqwest::Error) -> Self {
    APIError::HTTPError { error }
  }
}

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

  pub fn get_user(&self) -> Result<User, APIError> {
    self.get::<User>("/1/users/me")
  }

  pub fn get_devices(&self) -> Result<Vec<Device>, APIError> {
    self.get::<Vec<Device>>("/1/devices")
  }

  pub fn get_appliances(&self) -> Result<Vec<Appliance>, APIError> {
    self.get::<Vec<Appliance>>("/1/appliances")
  }

  pub fn update_aircon_settings(
    &self,
    aircon_id: &str,
    body: &RequestBody,
  ) -> Result<UpdateAirconSettingsResponse, APIError> {
    self.post::<UpdateAirconSettingsResponse>(
      &format!("/1/appliances/{}/aircon_settings", aircon_id),
      body,
    )
  }

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
