//! CO2 Sensor Module
//!
//! Provides structures and validation for CO2 concentration sensor data.

use chrono::{DateTime, Utc};

use crate::sensors::error::SensorValidationError;

/// Minimum allowed value for CO2 sensor (ppm)
const MIN_VALUE: f64 = 0.0;
/// Maximum allowed value for CO2 sensor (ppm)
const MAX_VALUE: f64 = 50_000.0;

/// Enumeration representing the unit of CO2 measurement.
///
/// # Variants
///
/// * `Ppm` - Parts per million
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CO2Unit {
    Ppm,
}

impl CO2Unit {
    /// Returns the unit as a string slice.
    ///
    /// # Returns
    ///
    /// A static string slice representing the unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use domain::sensors::co2::CO2Unit;
    ///
    /// let unit = CO2Unit::Ppm;
    /// assert_eq!(unit.as_str(), "ppm");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            CO2Unit::Ppm => "ppm",
        }
    }
}

impl TryFrom<&str> for CO2Unit {
    type Error = SensorValidationError;

    /// Attempts to convert a string to CO2Unit.
    ///
    /// # Arguments
    ///
    /// * `value` - The string to convert (case-insensitive, accepts "ppm")
    ///
    /// # Returns
    ///
    /// `Ok(CO2Unit)` on success, `Err(SensorValidationError::InvalidUnit)` on failure.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "ppm" => Ok(CO2Unit::Ppm),
            _ => Err(SensorValidationError::InvalidUnit(value.to_string())),
        }
    }
}

/// Structure representing CO2 sensor data.
///
/// Holds CO2 concentration data collected from IoT devices.
/// Validation is performed during instance creation.
///
/// # Fields
///
/// * `device_id` - Unique identifier for the device
/// * `timestamp` - Measurement time (UTC)
/// * `value` - CO2 concentration (0.0 to 50,000.0 ppm)
/// * `unit` - Unit of measurement
///
/// # Examples
///
/// ```
/// use chrono::Utc;
/// use domain::sensors::co2::{CO2Sensor, CO2Unit};
///
/// let sensor = CO2Sensor::new(
///     "device-001".to_string(),
///     Utc::now(),
///     400.0,
///     CO2Unit::Ppm,
/// ).expect("Valid sensor data");
///
/// assert_eq!(sensor.device_id(), "device-001");
/// assert_eq!(sensor.value(), 400.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CO2Sensor {
    device_id: String,
    timestamp: DateTime<Utc>,
    value: f64,
    unit: CO2Unit,
}

impl CO2Sensor {
    /// Creates a new CO2Sensor instance.
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique identifier for the device (must not be empty)
    /// * `timestamp` - Measurement time (must not be in the future)
    /// * `value` - CO2 concentration (0.0 to 50,000.0 ppm)
    /// * `unit` - Unit of measurement
    ///
    /// # Returns
    ///
    /// `Ok(CO2Sensor)` on success, `Err(SensorValidationError)` on validation failure.
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
        unit: CO2Unit,
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

    /// Returns the CO2 concentration value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the unit of measurement.
    pub fn unit(&self) -> CO2Unit {
        self.unit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod co2_sensor_new {
        use super::*;

        #[test]
        fn success_with_valid_data() {
            let result = CO2Sensor::new("device-001".to_string(), Utc::now(), 400.0, CO2Unit::Ppm);

            assert!(result.is_ok());
            let sensor = result.unwrap();
            assert_eq!(sensor.device_id(), "device-001");
            assert_eq!(sensor.value(), 400.0);
            assert_eq!(sensor.unit(), CO2Unit::Ppm);
        }

        #[test]
        fn fails_with_empty_device_id() {
            let result = CO2Sensor::new("".to_string(), Utc::now(), 400.0, CO2Unit::Ppm);

            assert_eq!(result, Err(SensorValidationError::EmptyDeviceId));
        }

        #[test]
        fn fails_with_future_timestamp() {
            let future = Utc::now() + chrono::Duration::hours(1);

            let result = CO2Sensor::new("device-001".to_string(), future, 400.0, CO2Unit::Ppm);

            assert_eq!(result, Err(SensorValidationError::FutureTimestamp));
        }

        #[test]
        fn fails_with_value_below_min() {
            let result = CO2Sensor::new("device-001".to_string(), Utc::now(), -1.0, CO2Unit::Ppm);

            assert!(matches!(
                result,
                Err(SensorValidationError::ValueOutOfRange { .. })
            ));
        }

        #[test]
        fn fails_with_value_above_max() {
            let result =
                CO2Sensor::new("device-001".to_string(), Utc::now(), 60_000.0, CO2Unit::Ppm);

            assert!(matches!(
                result,
                Err(SensorValidationError::ValueOutOfRange { .. })
            ));
        }

        #[test]
        fn success_with_min_boundary_value() {
            let result = CO2Sensor::new("device-001".to_string(), Utc::now(), 0.0, CO2Unit::Ppm);

            assert!(result.is_ok());
        }

        #[test]
        fn success_with_max_boundary_value() {
            let result =
                CO2Sensor::new("device-001".to_string(), Utc::now(), 50_000.0, CO2Unit::Ppm);

            assert!(result.is_ok());
        }
    }

    mod co2_unit {
        use super::*;

        #[test]
        fn as_str_returns_ppm() {
            assert_eq!(CO2Unit::Ppm.as_str(), "ppm");
        }

        #[test]
        fn try_from_lowercase_ppm() {
            assert_eq!(CO2Unit::try_from("ppm"), Ok(CO2Unit::Ppm));
        }

        #[test]
        fn try_from_uppercase_ppm() {
            assert_eq!(CO2Unit::try_from("PPM"), Ok(CO2Unit::Ppm));
        }

        #[test]
        fn try_from_invalid_unit() {
            let result = CO2Unit::try_from("invalid");

            assert!(matches!(result, Err(SensorValidationError::InvalidUnit(_))));
        }
    }
}
