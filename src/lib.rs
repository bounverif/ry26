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