//! Persistance Module
//!

use chrono::{DateTime, Utc};
use domain::entities::{SensorData, SensorMeasurement as DomainMeasurement};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorDataDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub device_id: String,

    pub timestamp: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<SensorMeasurement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity: Option<SensorMeasurement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2: Option<SensorMeasurement>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub additional_sensors: HashMap<String, SensorMeasurement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorMeasurement {
    pub value: f64,
    pub unit: String,
}

impl From<&SensorData> for SensorDataDocument {
    fn from(data: &SensorData) -> Self {
        Self {
            id: None,
            device_id: data.device_id.clone(),
            timestamp: data.timestamp,
            temperature: data.temperature.as_ref().map(SensorMeasurement::from),
            humidity: data.humidity.as_ref().map(SensorMeasurement::from),
            co2: data.co2.as_ref().map(SensorMeasurement::from),
            additional_sensors: data
                .additional_sensors
                .iter()
                .map(|(k, v)| (k.clone(), SensorMeasurement::from(v)))
                .collect(),
        }
    }
}

impl From<SensorDataDocument> for SensorData {
    fn from(doc: SensorDataDocument) -> Self {
        Self {
            device_id: doc.device_id,
            timestamp: doc.timestamp,
            temperature: doc.temperature.map(DomainMeasurement::from),
            humidity: doc.humidity.map(DomainMeasurement::from),
            co2: doc.co2.map(DomainMeasurement::from),
            additional_sensors: doc
                .additional_sensors
                .into_iter()
                .map(|(k, v)| (k, DomainMeasurement::from(v)))
                .collect(),
        }
    }
}

impl From<&DomainMeasurement> for SensorMeasurement {
    fn from(m: &DomainMeasurement) -> Self {
        Self {
            value: m.value,
            unit: m.unit.clone(),
        }
    }
}

impl From<SensorMeasurement> for DomainMeasurement {
    fn from(m: SensorMeasurement) -> Self {
        Self {
            value: m.value,
            unit: m.unit,
        }
    }
}
