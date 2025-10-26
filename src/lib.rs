use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A simple data structure that demonstrates serialization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataPoint {
    pub id: u64,
    pub value: f64,
    pub timestamp: String,
}

/// Custom error type using thiserror
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Add two numbers together
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Generate a random data point with current timestamp
pub fn generate_random_data_point() -> DataPoint {
    let mut rng = rand::thread_rng();
    let now: DateTime<Utc> = Utc::now();

    DataPoint {
        id: rng.gen_range(1..1000),
        value: rng.gen_range(0.0..100.0),
        timestamp: now.to_rfc3339(),
    }
}

/// Serialize a data point to JSON string
pub fn to_json(data: &DataPoint) -> Result<String, LibraryError> {
    Ok(serde_json::to_string(data)?)
}

/// Deserialize a data point from JSON string
pub fn from_json(json: &str) -> Result<DataPoint, LibraryError> {
    Ok(serde_json::from_str(json)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_generate_random_data_point() {
        let data = generate_random_data_point();
        assert!(data.id > 0 && data.id < 1000);
        assert!(data.value >= 0.0 && data.value < 100.0);
        assert!(!data.timestamp.is_empty());
    }

    #[test]
    fn test_json_serialization() {
        let data = DataPoint {
            id: 42,
            value: 3.14,
            timestamp: "2025-10-26T13:00:00Z".to_string(),
        };

        let json = to_json(&data).unwrap();
        assert!(json.contains("42"));
        assert!(json.contains("3.14"));

        let deserialized = from_json(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_error_handling() {
        let invalid_json = "{invalid json}";
        let result = from_json(invalid_json);
        assert!(result.is_err());
    }
}
