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
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    pub fn as_str(&self) -> &str {
        match self {
            &TemperatureUnit::Celsius => "c",
            &TemperatureUnit::Fahrenheit => "f",
        }
    }
}
