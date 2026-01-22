//! Sensor Trait Module
//!
//! Provides a common trait for all sensor types.

use chrono::{DateTime, Utc};

use crate::sensors::{co2::CO2Sensor, humidity::HumiditySensor, temperature::TemperatureSensor};

/// A trait representing common behavior for all sensor types.
///
/// This trait provides a unified interface for accessing sensor data
/// regardless of the specific sensor type (temperature, humidity, CO2, etc.).
///
/// # Required Methods
///
/// * `device_id` - Returns the unique identifier of the device
/// * `timestamp` - Returns the measurement timestamp
/// * `value` - Returns the measured value
/// * `unit` - Returns the unit of measurement as a string
pub trait Sensor {
    /// Returns the device ID.
    fn device_id(&self) -> &str;

    /// Returns the measurement timestamp.
    fn timestamp(&self) -> DateTime<Utc>;

    /// Returns the measured value.
    fn value(&self) -> f64;

    /// Returns the unit of measurement as a string.
    fn unit(&self) -> &str;
}

/// Implementation of the Sensor trait for TemperatureSensor.
impl Sensor for TemperatureSensor {
    fn device_id(&self) -> &str {
        self.device_id()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp()
    }

    fn value(&self) -> f64 {
        self.value()
    }

    fn unit(&self) -> &str {
        self.unit().as_str()
    }
}

/// Implementation of the Sensor trait for HumiditySensor.
impl Sensor for HumiditySensor {
    fn device_id(&self) -> &str {
        self.device_id()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp()
    }

    fn value(&self) -> f64 {
        self.value()
    }

    fn unit(&self) -> &str {
        self.unit().as_str()
    }
}

/// Implementation of the Sensor trait for CO2Sensor.
impl Sensor for CO2Sensor {
    fn device_id(&self) -> &str {
        self.device_id()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp()
    }

    fn value(&self) -> f64 {
        self.value()
    }

    fn unit(&self) -> &str {
        self.unit().as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensors::{co2::CO2Unit, humidity::HumidityUnit, temperature::TemperatureUnit};

    mod sensor_trait_for_temperature {
        use super::*;

        #[test]
        fn returns_device_id() {
            let sensor = TemperatureSensor::new(
                "temp-001".to_string(),
                Utc::now(),
                25.0,
                TemperatureUnit::Celsius,
            )
            .unwrap();

            assert_eq!(Sensor::device_id(&sensor), "temp-001");
        }

        #[test]
        fn returns_value() {
            let sensor = TemperatureSensor::new(
                "temp-001".to_string(),
                Utc::now(),
                25.0,
                TemperatureUnit::Celsius,
            )
            .unwrap();

            assert_eq!(Sensor::value(&sensor), 25.0);
        }

        #[test]
        fn returns_unit_as_string() {
            let sensor = TemperatureSensor::new(
                "temp-001".to_string(),
                Utc::now(),
                25.0,
                TemperatureUnit::Celsius,
            )
            .unwrap();

            assert_eq!(Sensor::unit(&sensor), "Celsius");
        }
    }

    mod sensor_trait_for_humidity {
        use super::*;

        #[test]
        fn returns_device_id() {
            let sensor = HumiditySensor::new(
                "hum-001".to_string(),
                Utc::now(),
                50.0,
                HumidityUnit::Percent,
            )
            .unwrap();

            assert_eq!(Sensor::device_id(&sensor), "hum-001");
        }

        #[test]
        fn returns_value() {
            let sensor = HumiditySensor::new(
                "hum-001".to_string(),
                Utc::now(),
                50.0,
                HumidityUnit::Percent,
            )
            .unwrap();

            assert_eq!(Sensor::value(&sensor), 50.0);
        }

        #[test]
        fn returns_unit_as_string() {
            let sensor = HumiditySensor::new(
                "hum-001".to_string(),
                Utc::now(),
                50.0,
                HumidityUnit::Percent,
            )
            .unwrap();

            assert_eq!(Sensor::unit(&sensor), "Percent");
        }
    }

    mod sensor_trait_for_co2 {
        use super::*;

        #[test]
        fn returns_device_id() {
            let sensor =
                CO2Sensor::new("co2-001".to_string(), Utc::now(), 400.0, CO2Unit::Ppm).unwrap();

            assert_eq!(Sensor::device_id(&sensor), "co2-001");
        }

        #[test]
        fn returns_value() {
            let sensor =
                CO2Sensor::new("co2-001".to_string(), Utc::now(), 400.0, CO2Unit::Ppm).unwrap();

            assert_eq!(Sensor::value(&sensor), 400.0);
        }

        #[test]
        fn returns_unit_as_string() {
            let sensor =
                CO2Sensor::new("co2-001".to_string(), Utc::now(), 400.0, CO2Unit::Ppm).unwrap();

            assert_eq!(Sensor::unit(&sensor), "ppm");
        }
    }
}
