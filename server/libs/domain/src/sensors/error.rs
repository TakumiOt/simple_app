use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SensorValidationError {
    EmptyDeviceId,
    FutureTimestamp,
    ValueOutOfRange { value: f64, min: f64, max: f64 },
    InvalidUnit(String),
}

impl fmt::Display for SensorValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorValidationError::EmptyDeviceId => {
                write!(f, "device_id must not be empty")
            }
            SensorValidationError::FutureTimestamp => {
                write!(f, "timestamp must not be in the future")
            }
            SensorValidationError::ValueOutOfRange { value, min, max } => {
                write!(f, "value {} is out of range [{}, {}]", value, min, max)
            }
            SensorValidationError::InvalidUnit(unit) => {
                write!(f, "invalid unit: {}", unit)
            }
        }
    }
}

impl std::error::Error for SensorValidationError {}
