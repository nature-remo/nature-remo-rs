extern crate reqwest;
extern crate serde;
extern crate url;

pub mod interfaces;

use interfaces::*;
use reqwest::Error;
use serde::de::DeserializeOwned;
use url::Url;

pub struct Cloud {
  token: String,
  base_url: Url,
  client: reqwest::Client,
}

impl Cloud {
  pub fn new(token: Option<&str>) -> Self {
    Cloud {
      token: match token {
        Some(token) => token.to_string(),
        None => "".to_string(),
      },
      base_url: Url::parse("https://api.nature.global").unwrap(),
      client: reqwest::Client::new(),
    }
  }

  pub fn get_user(&self) -> Result<User, Error> {
    let user = self.get::<User>("/1/users/me");
    user
  }

  fn get<T>(&self, path: &str) -> Result<T, Error>
  where
    T: DeserializeOwned,
  {
    let request_url = self.base_url.join(path).unwrap();
    println!("{}", request_url);
    let mut response = self
      .client
      .get(request_url.as_str())
      .header("Authorization", format!("Bearer {}", self.token))
      .send()?;
    let result: T = response.json()?;
    Ok(result)
  }

  #[allow(dead_code)]
  fn post<T>(&self, path: &str, body: RequestBody) -> Result<T, Error>
  where
    T: DeserializeOwned,
  {
    let request_url = self.base_url.join(path).unwrap();
    let mut response = self
      .client
      .post(request_url.as_str())
      .header("Authorization", format!("Bearer {}", self.token))
      .json(&body)
      .send()?;
    let result: T = response.json()?;
    Ok(result)
  }
}
