use ry26::{DataPoint, generate_random_data_point};

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
