use ry26::{DataPoint, DataPointSequence};

#[test]
fn test_data_point_sequence_creation() {
    let sequence = DataPointSequence::new(10);
    assert_eq!(sequence.step(), 0);
    assert_eq!(sequence.len(), 0);
    assert!(sequence.is_empty());
}

#[test]
fn test_data_point_sequence_add_point() {
    let mut sequence = DataPointSequence::new(10);
    
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    
    // Point is in back buffer, not yet in current
    assert_eq!(sequence.len(), 0);
    assert_eq!(sequence.step(), 0);
}

#[test]
fn test_data_point_sequence_update() {
    let mut sequence = DataPointSequence::new(10);
    
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    
    // Update to make the point current
    sequence.update();
    
    assert_eq!(sequence.len(), 1);
    assert_eq!(sequence.step(), 1);
    assert_eq!(sequence.current()[0].id, 1);
}

#[test]
fn test_data_point_sequence_multiple_updates() {
    let mut sequence = DataPointSequence::new(10);
    
    // First step
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    sequence.update();
    assert_eq!(sequence.step(), 1);
    assert_eq!(sequence.len(), 1);
    
    // Second step
    sequence.add_point(DataPoint {
        id: 2,
        value: 20.0,
        timestamp: "2025-10-27T12:01:00Z".to_string(),
    });
    sequence.update();
    assert_eq!(sequence.step(), 2);
    assert_eq!(sequence.len(), 1);
    assert_eq!(sequence.current()[0].id, 2);
    
    // Third step
    sequence.add_point(DataPoint {
        id: 3,
        value: 30.0,
        timestamp: "2025-10-27T12:02:00Z".to_string(),
    });
    sequence.update();
    assert_eq!(sequence.step(), 3);
    assert_eq!(sequence.len(), 1);
    assert_eq!(sequence.current()[0].id, 3);
}

#[test]
fn test_data_point_sequence_add_multiple_points() {
    let mut sequence = DataPointSequence::new(10);
    
    let points = vec![
        DataPoint {
            id: 1,
            value: 10.0,
            timestamp: "2025-10-27T12:00:00Z".to_string(),
        },
        DataPoint {
            id: 2,
            value: 20.0,
            timestamp: "2025-10-27T12:01:00Z".to_string(),
        },
        DataPoint {
            id: 3,
            value: 30.0,
            timestamp: "2025-10-27T12:02:00Z".to_string(),
        },
    ];
    
    sequence.add_points(points);
    sequence.update();
    
    assert_eq!(sequence.len(), 3);
    assert_eq!(sequence.step(), 1);
    assert_eq!(sequence.current()[0].id, 1);
    assert_eq!(sequence.current()[1].id, 2);
    assert_eq!(sequence.current()[2].id, 3);
}

#[test]
fn test_data_point_sequence_sequential_updates() {
    let mut sequence = DataPointSequence::new(10);
    
    // Simulate a time series with sequential updates
    for i in 1..=5 {
        sequence.add_point(DataPoint {
            id: i,
            value: i as f64 * 10.0,
            timestamp: format!("2025-10-27T12:0{}:00Z", i),
        });
        sequence.update();
        
        assert_eq!(sequence.step(), i as usize);
        assert_eq!(sequence.len(), 1);
        assert_eq!(sequence.current()[0].id, i);
    }
}

#[test]
fn test_data_point_sequence_clear() {
    let mut sequence = DataPointSequence::new(10);
    
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    sequence.update();
    
    assert_eq!(sequence.len(), 1);
    assert_eq!(sequence.step(), 1);
    
    sequence.clear();
    
    assert_eq!(sequence.len(), 0);
    assert_eq!(sequence.step(), 0);
    assert!(sequence.is_empty());
}

#[test]
fn test_data_point_sequence_read_while_write() {
    let mut sequence = DataPointSequence::new(10);
    
    // Initial data
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    sequence.update();
    
    // Read current data
    let current = sequence.current();
    assert_eq!(current.len(), 1);
    assert_eq!(current[0].id, 1);
    
    // Add new data to back buffer while reading from front
    sequence.add_point(DataPoint {
        id: 2,
        value: 20.0,
        timestamp: "2025-10-27T12:01:00Z".to_string(),
    });
    
    // Current should still be unchanged
    assert_eq!(sequence.current()[0].id, 1);
    
    // Update to see new data
    sequence.update();
    assert_eq!(sequence.current()[0].id, 2);
}

#[test]
fn test_data_point_sequence_pool_efficiency() {
    let mut sequence = DataPointSequence::new(5);
    
    // Perform multiple updates to exercise the pool
    for i in 1..=10 {
        sequence.add_point(DataPoint {
            id: i,
            value: i as f64 * 10.0,
            timestamp: format!("2025-10-27T12:0{}:00Z", i % 10),
        });
        sequence.update();
    }
    
    // Pool should have vectors available after updates
    // Just verify we can call the method without panicking
    let _ = sequence.pool_available();
}

#[test]
fn test_data_point_sequence_empty_update() {
    let mut sequence = DataPointSequence::new(10);
    
    // Update without adding any points
    sequence.update();
    
    assert_eq!(sequence.step(), 1);
    assert_eq!(sequence.len(), 0);
    assert!(sequence.is_empty());
}

#[test]
fn test_data_point_sequence_varying_sizes() {
    let mut sequence = DataPointSequence::new(10);
    
    // First update with 1 point
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    sequence.update();
    assert_eq!(sequence.len(), 1);
    
    // Second update with 3 points
    sequence.add_points(vec![
        DataPoint {
            id: 2,
            value: 20.0,
            timestamp: "2025-10-27T12:01:00Z".to_string(),
        },
        DataPoint {
            id: 3,
            value: 30.0,
            timestamp: "2025-10-27T12:02:00Z".to_string(),
        },
        DataPoint {
            id: 4,
            value: 40.0,
            timestamp: "2025-10-27T12:03:00Z".to_string(),
        },
    ]);
    sequence.update();
    assert_eq!(sequence.len(), 3);
    
    // Third update with no points
    sequence.update();
    assert_eq!(sequence.len(), 0);
}
