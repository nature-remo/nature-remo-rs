use super::device::Device;
use super::error::APIError;
use super::Client;
use super::RequestBody;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Appliance {
  pub id: String,
  pub device: Device,
  pub model: Option<Model>,
  pub nickname: String,
  pub image: String,
  pub r#type: String,
  pub settings: Option<AirconSettings>,
  pub aircon: Option<Aircon>,
  pub signals: Vec<Signal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DetectedAirconModel {
  pub model: Model,
  pub params: AirconSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Model {
  pub id: String,
  pub manufacturer: String,
  pub remote_name: String,
  pub name: String,
  pub image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AirconSettings {
  pub mode: String,
  pub temp: String,
  pub vol: String,
  pub dir: String,
  pub button: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateAirconSettingsResponse {
  pub mode: String,
  pub temp: String,
  pub vol: String,
  pub dir: String,
  pub button: String,
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AirconModeValue {
  pub temp: Vec<String>,
  pub dir: Vec<String>,
  pub vol: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AirconModes {
  pub cool: AirconModeValue,
  pub warm: AirconModeValue,
  pub dry: AirconModeValue,
  pub blow: AirconModeValue,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AirconRange {
  pub modes: AirconModes,
  #[serde(rename = "fixedButtons")]
  pub fixed_buttons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Aircon {
  pub range: AirconRange,
  #[serde(rename = "tempUnit")]
  pub temp_unit: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Signal {
  pub id: String,
  pub name: String,
  pub image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SignalMessage {
  pub data: Vec<i32>,
  pub freq: i32,
  pub format: String,
}

impl Client {
  pub fn get_appliances(&self) -> Result<Vec<Appliance>, APIError> {
    self.get::<Vec<Appliance>>("/1/appliances")
  }

  pub fn find_aircon(&self) -> Result<Appliance, APIError> {
    let appliances = self.get_appliances().unwrap();
    let aircon = appliances.iter().find(|&app| app.r#type == "AC").unwrap();
    Ok(aircon.clone())
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
}

#[cfg(test)]
mod tests {
  use super::Client;
  use super::RequestBody;
  use crate::tests::get_test_token;

  #[test]
  fn it_get_appliances() {
    let token = get_test_token();
    let client = Client::new(Some(token));
    let res = client.get_appliances().unwrap();
    println!("{:?}", res);
    assert_eq!(res[0].device.name, "Living Room")
  }

  #[test]
  fn it_update_aircon_settings() {
    let token = get_test_token();
    let client = Client::new(Some(token));

    let appliances = client.get_appliances().unwrap();
    let aircon = appliances.iter().find(|&app| app.r#type == "AC").unwrap();

    {
      let mut body = RequestBody::new();
      body.insert("operation_mode", "warm");
      let resp = client.update_aircon_settings(&aircon.id, &body).unwrap();
      println!("{:?}", resp);
      assert_eq!(resp.mode, "warm");
    }

    {
      let mut body = RequestBody::new();
      body.insert("operation_mode", "cool");
      let resp = client.update_aircon_settings(&aircon.id, &body).unwrap();
      println!("{:?}", resp);
      assert_eq!(resp.mode, "cool");
    }
  }
}
