use super::error::APIError;
use super::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorValue {
  pub temperature: f32,
  pub humidity: f32,
  pub illumination: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
  pub id: String,
  pub name: String,
  pub temperature_offset: i32,
  pub humidity_offset: i32,
  pub created_at: String,
  pub updated_at: String,
  pub firmware_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewestEvents {
  pub te: EventValue,
  pub hu: EventValue,
  pub il: EventValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventValue {
  pub val: f32,
  pub created_at: String,
}

impl Client {
  pub fn get_devices(&self) -> Result<Vec<DeviceWithEvents>, APIError> {
    self.get::<Vec<DeviceWithEvents>>("/1/devices")
  }

  pub fn get_sensor_value(&self) -> Result<SensorValue, APIError> {
    let sensor_value = match self.get_devices() {
      Ok(devices) => {
        let device = &devices[0];
        SensorValue {
          temperature: device.newest_events.te.val,
          humidity: device.newest_events.hu.val,
          illumination: device.newest_events.il.val,
        }
      }
      Err(err) => return Err(err),
    };
    Ok(sensor_value)
  }
}

#[cfg(test)]
mod tests {
  use super::Client;
  use crate::tests::get_test_token;

  #[test]
  fn it_get_devices() {
    let token = get_test_token();
    let client = Client::new(Some(token));
    let res = client.get_devices().unwrap();
    println!("{:?}", res);
    assert_eq!(res[0].firmware_version, "Remo/1.0.62-gabbf5bd")
  }

  #[test]
  fn it_get_sensor_value() {
    let token = get_test_token();
    let client = Client::new(Some(token));

    let senval = client.get_sensor_value().unwrap();
    println!("{:?}", senval);
  }

}
