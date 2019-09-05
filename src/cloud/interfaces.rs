use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceEventType {
  Temperature,
  Humidity,
  Illumination,
}

impl DeviceEventType {
  pub fn as_str(&self) -> &str {
    match self {
      &DeviceEventType::Temperature => "te",
      &DeviceEventType::Humidity => "hu",
      &DeviceEventType::Illumination => "il",
    }
  }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedAirconModel {
  model: Model,
  params: AirconSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorValue {
  temperature: i32,
  humidity: i32,
  illumination: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
  id: String,
  name: String,
  temperature_offset: i32,
  humidity_offset: i32,
  created_at: String,
  updated_at: String,
  firmware_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewestEvents {
  te: EventValue,
  hu: EventValue,
  il: EventValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceWithEvents {
  id: String,
  name: String,
  temperature_offset: i32,
  humidity_offset: i32,
  created_at: String,
  updated_at: String,
  firmware_version: String,
  newest_events: NewestEvents,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventValue {
  val: i32,
  created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
  id: String,
  manufacturer: String,
  remote_name: String,
  name: String,
  image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconSettings {
  temp: String,
  mode: AirconModeType,
  vol: String,
  dir: String,
  button: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAirconSettingsResponse {
  temp: String,
  mode: AirconModeType,
  vol: String,
  dir: String,
  button: String,
  updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconModeValue {
  temp: Vec<String>,
  dir: Vec<String>,
  vol: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconModes {
  cool: AirconModeValue,
  warm: AirconModeValue,
  dry: AirconModeValue,
  blow: AirconModeValue,
  auto: AirconModeValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirconRange {
  modes: AirconModes,
  #[serde(rename = "fixedButtons")]
  fixed_buttons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aircon {
  range: AirconRange,
  #[serde(rename = "tempUnit")]
  temp_unit: TemperatureUnit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signal {
  id: String,
  name: String,
  image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appliance {
  id: String,
  device: Device,
  model: Model,
  nickname: String,
  image: String,
  r#type: String,
  settings: AirconSettings,
  aircon: Option<Aircon>,
  signals: Vec<Signal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalMessage {
  data: Vec<i32>,
  freq: i32,
  format: String,
}

pub type RequestBody = HashMap<String, String>;
