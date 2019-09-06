extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod optional_string {
  use serde::de;
  use serde::de::{Deserialize, Deserializer};
  use serde::ser::{Serialize, Serializer};
  use std::fmt::Display;
  use std::str::FromStr;

  pub fn serialize<T, S>(value: Option<&T>, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: Display,
    S: Serializer,
  {
    serializer.collect_str(value.unwrap())
  }

  pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
  where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
  {
    Ok(Some(
      String::deserialize(deserializer)?
        .parse()
        .map_err(de::Error::custom)?,
    ))
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedAirconModel {
  pub model: Model,
  pub params: AirconSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorValue {
  pub temperature: i32,
  pub humidity: i32,
  pub illumination: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
  pub id: String,
  pub name: String,
  pub temperature_offset: i32,
  pub humidity_offset: i32,
  pub created_at: String,
  pub updated_at: String,
  pub firmware_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewestEvents {
  pub te: EventValue,
  pub hu: EventValue,
  pub il: EventValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceWithEvents {
  pub id: String,
  pub name: String,
  pub temperature_offset: i32,
  pub humidity_offset: i32,
  pub created_at: String,
  pub updated_at: String,
  pub firmware_version: String,
  pub newest_events: NewestEvents,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventValue {
  pub val: i32,
  pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
  pub id: String,
  pub manufacturer: String,
  pub remote_name: String,
  pub name: String,
  pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconSettings {
  pub mode: String,
  pub temp: String,
  pub vol: String,
  pub dir: String,
  pub button: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAirconSettingsResponse {
  pub mode: String,
  pub temp: String,
  pub vol: String,
  pub dir: String,
  pub button: String,
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconModeValue {
  pub temp: Vec<String>,
  pub dir: Vec<String>,
  pub vol: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconModes {
  pub cool: AirconModeValue,
  pub warm: AirconModeValue,
  pub dry: AirconModeValue,
  pub blow: AirconModeValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconRange {
  pub modes: AirconModes,
  #[serde(rename = "fixedButtons")]
  pub fixed_buttons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aircon {
  pub range: AirconRange,
  #[serde(rename = "tempUnit")]
  pub temp_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Signal {
  pub id: String,
  pub name: String,
  pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalMessage {
  pub data: Vec<i32>,
  pub freq: i32,
  pub format: String,
}

pub type RequestBody<'a> = HashMap<&'a str, &'a str>;
