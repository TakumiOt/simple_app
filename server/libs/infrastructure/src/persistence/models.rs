//! Persistance Module
//!

use chrono::{DateTime, Utc};
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
