extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceEventType {
  #[serde(rename = "te")]
  Temperature,
  #[serde(rename = "hu")]
  Humidity,
  #[serde(rename = "il")]
  Illumination,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AirconModeType {
  Cool,
  Warm,
  Dry,
  Blow,
  Auto,
}

impl AirconModeType {
  pub fn as_str(&self) -> &str {
    match self {
      &AirconModeType::Cool => "cool",
      &AirconModeType::Warm => "warm",
      &AirconModeType::Dry => "dry",
      &AirconModeType::Blow => "blow",
      &AirconModeType::Auto => "auto",
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TemperatureUnit {
  Celcius,
  Fahrenheit,
}

impl TemperatureUnit {
  pub fn as_str(&self) -> &str {
    match self {
      &TemperatureUnit::Celcius => "c",
      &TemperatureUnit::Fahrenheit => "f",
    }
  }
}