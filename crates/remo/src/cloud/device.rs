use super::error::APIError;
use super::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SensorValue {
    pub temperature: f32,
    pub humidity: f32,
    pub illumination: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub temperature_offset: i32,
    pub humidity_offset: i32,
    pub created_at: String,
    pub updated_at: String,
    pub firmware_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NewestEvents {
    pub te: EventValue,
    pub hu: EventValue,
    pub il: EventValue,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EventValue {
    pub val: f32,
    pub created_at: String,
}

impl Client {
    pub async fn get_devices(&self) -> Result<Vec<DeviceWithEvents>, APIError> {
        self.get::<Vec<DeviceWithEvents>>("/1/devices").await
    }

    pub async fn get_sensor_value(&self) -> Result<SensorValue, APIError> {
        let devices = self.get_devices().await?;
        let device = &devices[0];
        Ok(SensorValue {
            temperature: device.newest_events.te.val,
            humidity: device.newest_events.hu.val,
            illumination: device.newest_events.il.val,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::tests::get_test_token;

    #[tokio::test]
    async fn it_get_devices() {
        let token = get_test_token();
        let client = Client::new(Some(token));
        let res = client.get_devices().await.unwrap();
        assert_eq!(res[0].firmware_version, "Remo/1.0.69-gbbcc0de")
    }

    #[tokio::test]
    async fn it_get_sensor_value() {
        let token = get_test_token();
        let client = Client::new(Some(token));

        let senval = client.get_sensor_value().await.unwrap();
        assert!(senval.temperature > 0.0);
        assert!(senval.humidity > 0.0);
        assert!(senval.illumination > 0.0);
    }
}
