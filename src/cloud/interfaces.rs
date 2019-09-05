extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

mod string {
  use std::fmt::Display;
  use std::str::FromStr;

  use serde::{de, Deserialize, Deserializer, Serializer};

  pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: Display,
    S: Serializer,
  {
    serializer.collect_str(value)
  }

  pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
  where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
  {
    String::deserialize(deserializer)?
      .parse()
      .map_err(de::Error::custom)
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DeviceEventType {
  #[serde(rename = "te")]
  Temperature,
  #[serde(rename = "hu")]
  Humidity,
  #[serde(rename = "il")]
  Illumination,
}

// impl DeviceEventType {
//   pub fn as_str(&self) -> &str {
//     match self {
//       &DeviceEventType::Temperature => "te",
//       &DeviceEventType::Humidity => "hu",
//       &DeviceEventType::Illumination => "il",
//     }
//   }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum AirconModeType {
//   Cool,
//   Warm,
//   Dry,
//   Blow,
//   Auto,
// }

// impl AirconModeType {
//   pub fn as_str(&self) -> &str {
//     match self {
//       &AirconModeType::Cool => "cool",
//       &AirconModeType::Warm => "warm",
//       &AirconModeType::Dry => "dry",
//       &AirconModeType::Blow => "blow",
//       &AirconModeType::Auto => "auto",
//     }
//   }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum TemperatureUnit {
//   Celcius,
//   Fahrenheit,
// }

// impl TemperatureUnit {
//   pub fn as_str(&self) -> &str {
//     match self {
//       &TemperatureUnit::Celcius => "c",
//       &TemperatureUnit::Fahrenheit => "f",
//     }
//   }
// }

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

pub type RequestBody = HashMap<String, String>;
