use ry26::{DataPoint, from_json, to_json};

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
