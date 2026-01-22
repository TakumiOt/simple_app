//! Temperature Sensor Module
//!
//! Provides structures and validation for temperature sensor data.

use chrono::{DateTime, Utc};

use crate::sensors::error::SensorValidationError;

/// Minimum allowed value for temperature sensor
const MIN_VALUE: f64 = -50.0;
/// Maximum allowed value for temperature sensor
const MAX_VALUE: f64 = 150.0;

/// Enumeration representing the unit of temperature measurement.
///
/// # Variants
///
/// * `Celsius` - Degrees Celsius
/// * `Fahrenheit` - Degrees Fahrenheit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    /// Returns the unit as a string slice.
    ///
    /// # Returns
    ///
    /// A static string slice representing the unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use domain::sensors::temperature::TemperatureUnit;
    ///
    /// let unit = TemperatureUnit::Celsius;
    /// assert_eq!(unit.as_str(), "Celsius");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "Celsius",
            TemperatureUnit::Fahrenheit => "Fahrenheit",
        }
    }
}

impl TryFrom<&str> for TemperatureUnit {
    type Error = SensorValidationError;

    /// Attempts to convert a string to TemperatureUnit.
    ///
    /// # Arguments
    ///
    /// * `value` - The string to convert (case-insensitive, accepts "celsius"/"c" or "fahrenheit"/"f")
    ///
    /// # Returns
    ///
    /// `Ok(TemperatureUnit)` on success, `Err(SensorValidationError::InvalidUnit)` on failure.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "celsius" | "c" => Ok(TemperatureUnit::Celsius),
            "fahrenheit" | "f" => Ok(TemperatureUnit::Fahrenheit),
            _ => Err(SensorValidationError::InvalidUnit(value.to_string())),
        }
    }
}

/// Structure representing temperature sensor data.
///
/// Holds temperature data collected from IoT devices.
/// Validation is performed during instance creation.
///
/// # Fields
///
/// * `device_id` - Unique identifier for the device
/// * `timestamp` - Measurement time (UTC)
/// * `value` - Temperature value (-50.0 to 150.0)
/// * `unit` - Unit of measurement (Celsius or Fahrenheit)
///
/// # Examples
///
/// ```
/// use chrono::Utc;
/// use domain::sensors::temperature::{TemperatureSensor, TemperatureUnit};
///
/// let sensor = TemperatureSensor::new(
///     "device-001".to_string(),
///     Utc::now(),
///     25.0,
///     TemperatureUnit::Celsius,
/// ).expect("Valid sensor data");
///
/// assert_eq!(sensor.device_id(), "device-001");
/// assert_eq!(sensor.value(), 25.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TemperatureSensor {
    device_id: String,
    timestamp: DateTime<Utc>,
    value: f64,
    unit: TemperatureUnit,
}

impl TemperatureSensor {
    /// Creates a new TemperatureSensor instance.
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique identifier for the device (must not be empty)
    /// * `timestamp` - Measurement time (must not be in the future)
    /// * `value` - Temperature value (-50.0 to 150.0)
    /// * `unit` - Unit of measurement
    ///
    /// # Returns
    ///
    /// `Ok(TemperatureSensor)` on success, `Err(SensorValidationError)` on validation failure.
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
        unit: TemperatureUnit,
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

    /// Returns the temperature value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the unit of measurement.
    pub fn unit(&self) -> TemperatureUnit {
        self.unit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod temperature_sensor_new {
        use super::*;

        #[test]
        fn success_with_valid_data() {
            let result = TemperatureSensor::new(
                "device-001".to_string(),
                Utc::now(),
                25.0,
                TemperatureUnit::Celsius,
            );

            assert!(result.is_ok());
            let sensor = result.unwrap();
            assert_eq!(sensor.device_id(), "device-001");
            assert_eq!(sensor.value(), 25.0);
            assert_eq!(sensor.unit(), TemperatureUnit::Celsius);
        }

        #[test]
        fn fails_with_empty_device_id() {
            let result =
                TemperatureSensor::new("".to_string(), Utc::now(), 25.0, TemperatureUnit::Celsius);

            assert_eq!(result, Err(SensorValidationError::EmptyDeviceId));
        }

        #[test]
        fn fails_with_future_timestamp() {
            let future = Utc::now() + chrono::Duration::hours(1);

            let result = TemperatureSensor::new(
                "device-001".to_string(),
                future,
                25.0,
                TemperatureUnit::Celsius,
            );

            assert_eq!(result, Err(SensorValidationError::FutureTimestamp));
        }

        #[test]
        fn fails_with_value_below_min() {
            let result = TemperatureSensor::new(
                "device-001".to_string(),
                Utc::now(),
                -60.0,
                TemperatureUnit::Celsius,
            );

            assert!(matches!(
                result,
                Err(SensorValidationError::ValueOutOfRange { .. })
            ));
        }

        #[test]
        fn fails_with_value_above_max() {
            let result = TemperatureSensor::new(
                "device-001".to_string(),
                Utc::now(),
                160.0,
                TemperatureUnit::Celsius,
            );

            assert!(matches!(
                result,
                Err(SensorValidationError::ValueOutOfRange { .. })
            ));
        }

        #[test]
        fn success_with_min_boundary_value() {
            let result = TemperatureSensor::new(
                "device-001".to_string(),
                Utc::now(),
                -50.0,
                TemperatureUnit::Celsius,
            );

            assert!(result.is_ok());
        }

        #[test]
        fn success_with_max_boundary_value() {
            let result = TemperatureSensor::new(
                "device-001".to_string(),
                Utc::now(),
                150.0,
                TemperatureUnit::Celsius,
            );

            assert!(result.is_ok());
        }

        #[test]
        fn success_with_fahrenheit_unit() {
            let result = TemperatureSensor::new(
                "device-001".to_string(),
                Utc::now(),
                77.0,
                TemperatureUnit::Fahrenheit,
            );

            assert!(result.is_ok());
            let sensor = result.unwrap();
            assert_eq!(sensor.unit(), TemperatureUnit::Fahrenheit);
        }
    }

    mod temperature_unit {
        use super::*;

        #[test]
        fn as_str_returns_celsius() {
            assert_eq!(TemperatureUnit::Celsius.as_str(), "Celsius");
        }

        #[test]
        fn as_str_returns_fahrenheit() {
            assert_eq!(TemperatureUnit::Fahrenheit.as_str(), "Fahrenheit");
        }

        #[test]
        fn try_from_lowercase_celsius() {
            assert_eq!(
                TemperatureUnit::try_from("celsius"),
                Ok(TemperatureUnit::Celsius)
            );
        }

        #[test]
        fn try_from_short_celsius() {
            assert_eq!(TemperatureUnit::try_from("C"), Ok(TemperatureUnit::Celsius));
        }

        #[test]
        fn try_from_lowercase_fahrenheit() {
            assert_eq!(
                TemperatureUnit::try_from("fahrenheit"),
                Ok(TemperatureUnit::Fahrenheit)
            );
        }

        #[test]
        fn try_from_short_fahrenheit() {
            assert_eq!(
                TemperatureUnit::try_from("F"),
                Ok(TemperatureUnit::Fahrenheit)
            );
        }

        #[test]
        fn try_from_invalid_unit() {
            let result = TemperatureUnit::try_from("invalid");

            assert!(matches!(result, Err(SensorValidationError::InvalidUnit(_))));
        }
    }
}
