use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SensorData {
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub temperature: Option<SensorMeasurement>,
    pub humidity: Option<SensorMeasurement>,
    pub co2: Option<SensorMeasurement>,
    pub additional_sensors: HashMap<String, SensorMeasurement>,
}

#[derive(Debug, Clone)]
pub struct SensorMeasurement {
    pub value: f64,
    pub unit: String,
}

impl SensorData {
    pub fn new(device_id: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            device_id,
            timestamp,
            temperature: None,
            humidity: None,
            co2: None,
            additional_sensors: HashMap::new(),
        }
    }

    pub fn with_temperature(mut self, value: f64, unit: impl Into<String>) -> Self {
        self.temperature = Some(SensorMeasurement {
            value,
            unit: unit.into(),
        });
        self
    }

    pub fn with_humidity(mut self, value: f64, unit: impl Into<String>) -> Self {
        self.humidity = Some(SensorMeasurement {
            value,
            unit: unit.into(),
        });
        self
    }

    pub fn with_co2(mut self, value: f64, unit: impl Into<String>) -> Self {
        self.co2 = Some(SensorMeasurement {
            value,
            unit: unit.into(),
        });
        self
    }

    pub fn with_additional_sensor(
        mut self,
        name: impl Into<String>,
        value: f64,
        unit: impl Into<String>,
    ) -> Self {
        self.additional_sensors.insert(
            name.into(),
            SensorMeasurement {
                value,
                unit: unit.into(),
            },
        );
        self
    }
}
