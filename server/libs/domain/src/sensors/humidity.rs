//! Humidity Sensor Module
//!
//! Provides structures and validation for humidity sensor data.

use chrono::{DateTime, Utc};

use crate::sensors::error::SensorValidationError;

/// Minimum allowed value for humidity sensor (%)
const MIN_VALUE: f64 = 0.0;
/// Maximum allowed value for humidity sensor (%)
const MAX_VALUE: f64 = 100.0;

/// Enumeration representing the unit of humidity measurement.
///
/// # Variants
///
/// * `Percent` - Percentage (%)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumidityUnit {
    Percent,
}

impl HumidityUnit {
    /// Returns the unit as a string slice.
    ///
    /// # Returns
    ///
    /// A static string slice representing the unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use domain::sensors::humidity::HumidityUnit;
    ///
    /// let unit = HumidityUnit::Percent;
    /// assert_eq!(unit.as_str(), "Percent");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            HumidityUnit::Percent => "Percent",
        }
    }
}

impl TryFrom<&str> for HumidityUnit {
    type Error = SensorValidationError;

    /// Attempts to convert a string to HumidityUnit.
    ///
    /// # Arguments
    ///
    /// * `value` - The string to convert (case-insensitive, accepts "percent" or "%")
    ///
    /// # Returns
    ///
    /// `Ok(HumidityUnit)` on success, `Err(SensorValidationError::InvalidUnit)` on failure.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "percent" | "%" => Ok(HumidityUnit::Percent),
            _ => Err(SensorValidationError::InvalidUnit(value.to_string())),
        }
    }
}

/// Structure representing humidity sensor data.
///
/// Holds humidity data collected from IoT devices.
/// Validation is performed during instance creation.
///
/// # Fields
///
/// * `device_id` - Unique identifier for the device
/// * `timestamp` - Measurement time (UTC)
/// * `value` - Humidity percentage (0.0 to 100.0%)
/// * `unit` - Unit of measurement
///
/// # Examples
///
/// ```
/// use chrono::Utc;
/// use domain::sensors::humidity::{HumiditySensor, HumidityUnit};
///
/// let sensor = HumiditySensor::new(
///     "device-001".to_string(),
///     Utc::now(),
///     50.0,
///     HumidityUnit::Percent,
/// ).expect("Valid sensor data");
///
/// assert_eq!(sensor.device_id(), "device-001");
/// assert_eq!(sensor.value(), 50.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HumiditySensor {
    device_id: String,
    timestamp: DateTime<Utc>,
    value: f64,
    unit: HumidityUnit,
}

impl HumiditySensor {
    /// Creates a new HumiditySensor instance.
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique identifier for the device (must not be empty)
    /// * `timestamp` - Measurement time (must not be in the future)
    /// * `value` - Humidity percentage (0.0 to 100.0%)
    /// * `unit` - Unit of measurement
    ///
    /// # Returns
    ///
    /// `Ok(HumiditySensor)` on success, `Err(SensorValidationError)` on validation failure.
    ///
    /// # Errors
    ///
    /// * `SensorValidationError::EmptyDeviceId` - If device_id is empty
    /// * `SensorValidationError::FutureTimestamp` - If timestamp is in the future
    /// * `SensorValidationError::ValueOutOfRange` - If value is out of range
    pub fn new(
        device_id: String,
        timestamp: DateTime<Utc>,
        value: f64,
        unit: HumidityUnit,
    ) -> Result<Self, SensorValidationError> {
        if device_id.is_empty() {
            return Err(SensorValidationError::EmptyDeviceId);
        }

        if timestamp > Utc::now() {
            return Err(SensorValidationError::FutureTimestamp);
        }

        if !(MIN_VALUE..=MAX_VALUE).contains(&value) {
            return Err(SensorValidationError::ValueOutOfRange {
                value,
                min: MIN_VALUE,
                max: MAX_VALUE,
            });
        }

        Ok(Self {
            device_id,
            timestamp,
            value,
            unit,
        })
    }

    /// Returns the device ID.
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Returns the measurement timestamp.
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Returns the humidity value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the unit of measurement.
    pub fn unit(&self) -> HumidityUnit {
        self.unit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod humidity_sensor_new {
        use super::*;

        #[test]
        fn success_with_valid_data() {
            let result = HumiditySensor::new(
                "device-001".to_string(),
                Utc::now(),
                50.0,
                HumidityUnit::Percent,
            );

            assert!(result.is_ok());
            let sensor = result.unwrap();
            assert_eq!(sensor.device_id(), "device-001");
            assert_eq!(sensor.value(), 50.0);
            assert_eq!(sensor.unit(), HumidityUnit::Percent);
        }

        #[test]
        fn fails_with_empty_device_id() {
            let result =
                HumiditySensor::new("".to_string(), Utc::now(), 50.0, HumidityUnit::Percent);

            assert_eq!(result, Err(SensorValidationError::EmptyDeviceId));
        }

        #[test]
        fn fails_with_future_timestamp() {
            let future = Utc::now() + chrono::Duration::hours(1);

            let result = HumiditySensor::new(
                "device-001".to_string(),
                future,
                50.0,
                HumidityUnit::Percent,
            );

            assert_eq!(result, Err(SensorValidationError::FutureTimestamp));
        }

        #[test]
        fn fails_with_value_below_min() {
            let result = HumiditySensor::new(
                "device-001".to_string(),
                Utc::now(),
                -1.0,
                HumidityUnit::Percent,
            );

            assert!(matches!(
                result,
                Err(SensorValidationError::ValueOutOfRange { .. })
            ));
        }

        #[test]
        fn fails_with_value_above_max() {
            let result = HumiditySensor::new(
                "device-001".to_string(),
                Utc::now(),
                101.0,
                HumidityUnit::Percent,
            );

            assert!(matches!(
                result,
                Err(SensorValidationError::ValueOutOfRange { .. })
            ));
        }

        #[test]
        fn success_with_min_boundary_value() {
            let result = HumiditySensor::new(
                "device-001".to_string(),
                Utc::now(),
                0.0,
                HumidityUnit::Percent,
            );

            assert!(result.is_ok());
        }

        #[test]
        fn success_with_max_boundary_value() {
            let result = HumiditySensor::new(
                "device-001".to_string(),
                Utc::now(),
                100.0,
                HumidityUnit::Percent,
            );

            assert!(result.is_ok());
        }
    }

    mod humidity_unit {
        use super::*;

        #[test]
        fn as_str_returns_percent() {
            assert_eq!(HumidityUnit::Percent.as_str(), "Percent");
        }

        #[test]
        fn try_from_lowercase_percent() {
            assert_eq!(HumidityUnit::try_from("percent"), Ok(HumidityUnit::Percent));
        }

        #[test]
        fn try_from_percent_symbol() {
            assert_eq!(HumidityUnit::try_from("%"), Ok(HumidityUnit::Percent));
        }

        #[test]
        fn try_from_invalid_unit() {
            let result = HumidityUnit::try_from("invalid");

            assert!(matches!(result, Err(SensorValidationError::InvalidUnit(_))));
        }
    }
}
