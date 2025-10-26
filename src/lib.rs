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
    fn test_add_with_zero() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(5, 0), 5);
        assert_eq!(add(0, 5), 5);
    }

    #[test]
    fn test_add_large_numbers() {
        assert_eq!(add(1_000_000, 2_000_000), 3_000_000);
        assert_eq!(add(u64::MAX - 1, 1), u64::MAX);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let _ = add(u64::MAX, 1);
    }

    #[test]
    fn test_generate_random_data_point() {
        let data = generate_random_data_point();
        assert!(data.id > 0 && data.id < 1000);
        assert!(data.value >= 0.0 && data.value < 100.0);
        assert!(!data.timestamp.is_empty());
    }

    #[test]
    fn test_generate_multiple_random_data_points() {
        let data1 = generate_random_data_point();
        let data2 = generate_random_data_point();

        // Verify both are valid
        assert!(data1.id > 0 && data1.id < 1000);
        assert!(data2.id > 0 && data2.id < 1000);
        assert!(data1.value >= 0.0 && data1.value < 100.0);
        assert!(data2.value >= 0.0 && data2.value < 100.0);
    }

    #[test]
    fn test_data_point_equality() {
        let data1 = DataPoint {
            id: 100,
            value: 50.0,
            timestamp: "2025-10-26T14:00:00Z".to_string(),
        };
        let data2 = DataPoint {
            id: 100,
            value: 50.0,
            timestamp: "2025-10-26T14:00:00Z".to_string(),
        };
        let data3 = DataPoint {
            id: 101,
            value: 50.0,
            timestamp: "2025-10-26T14:00:00Z".to_string(),
        };

        assert_eq!(data1, data2);
        assert_ne!(data1, data3);
    }

    #[test]
    fn test_data_point_clone() {
        let original = DataPoint {
            id: 123,
            value: 45.67,
            timestamp: "2025-10-26T15:00:00Z".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_data_point_boundary_values() {
        let min_id = DataPoint {
            id: 1,
            value: 50.0,
            timestamp: "2025-10-26T00:00:00Z".to_string(),
        };
        assert_eq!(min_id.id, 1);

        let max_id = DataPoint {
            id: 999,
            value: 50.0,
            timestamp: "2025-10-26T23:59:59Z".to_string(),
        };
        assert_eq!(max_id.id, 999);

        let min_value = DataPoint {
            id: 100,
            value: 0.0,
            timestamp: "2025-10-26T12:00:00Z".to_string(),
        };
        assert_eq!(min_value.value, 0.0);

        let max_value = DataPoint {
            id: 100,
            value: 99.99,
            timestamp: "2025-10-26T12:00:00Z".to_string(),
        };
        assert!(max_value.value < 100.0);
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
    fn test_json_round_trip() {
        let original = DataPoint {
            id: 999,
            value: 0.0,
            timestamp: "2025-10-26T23:59:59.999Z".to_string(),
        };

        let json = to_json(&original).expect("Serialization should succeed");
        let deserialized = from_json(&json).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_json_with_special_characters() {
        let data = DataPoint {
            id: 1,
            value: 1.5,
            timestamp: "2025-10-26T12:00:00+00:00".to_string(),
        };

        let json = to_json(&data).unwrap();
        let deserialized = from_json(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_error_handling() {
        let invalid_json = "{invalid json}";
        let result = from_json(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_handling_empty_string() {
        let result = from_json("");
        assert!(result.is_err());
    }

    #[test]
    fn test_error_handling_incomplete_json() {
        let result = from_json("{\"id\": 1}");
        assert!(result.is_err());
    }

    #[test]
    fn test_error_handling_wrong_types() {
        let result = from_json(
            "{\"id\": \"not_a_number\", \"value\": 1.0, \"timestamp\": \"2025-10-26T12:00:00Z\"}",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_library_error_display() {
        let error = LibraryError::InvalidValue("test error".to_string());
        let error_message = format!("{}", error);
        assert_eq!(error_message, "Invalid value: test error");
    }

    #[test]
    fn test_library_error_debug() {
        let error = LibraryError::InvalidValue("test".to_string());
        let debug_output = format!("{:?}", error);
        assert!(debug_output.contains("InvalidValue"));
    }
}
