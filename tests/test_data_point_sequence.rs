use ry26::{DataPoint, DataPointSequence};

#[test]
fn test_data_point_sequence_creation() {
    let sequence = DataPointSequence::new(100, 10);
    assert_eq!(sequence.step(), 0);
    assert_eq!(sequence.len(), 0);
    assert!(sequence.is_empty());
}

#[test]
fn test_data_point_sequence_add_point() {
    let mut sequence = DataPointSequence::new(100, 10);
    
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    
    // Point is pending, not yet in current
    assert_eq!(sequence.len(), 0);
    assert_eq!(sequence.pending_count(), 1);
    assert_eq!(sequence.step(), 0);
}

#[test]
fn test_data_point_sequence_update() {
    let mut sequence = DataPointSequence::new(100, 10);
    
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
fn test_data_point_sequence_accumulates() {
    let mut sequence = DataPointSequence::new(100, 10);
    
    // First step - add one point
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    sequence.update();
    assert_eq!(sequence.step(), 1);
    assert_eq!(sequence.len(), 1);
    
    // Second step - add another point (accumulates, not replaces)
    sequence.add_point(DataPoint {
        id: 2,
        value: 20.0,
        timestamp: "2025-10-27T12:01:00Z".to_string(),
    });
    sequence.update();
    assert_eq!(sequence.step(), 2);
    assert_eq!(sequence.len(), 2); // Now has 2 points total
    assert_eq!(sequence.current()[0].id, 1); // First point still there
    assert_eq!(sequence.current()[1].id, 2); // Second point added
    
    // Third step - add yet another point
    sequence.add_point(DataPoint {
        id: 3,
        value: 30.0,
        timestamp: "2025-10-27T12:02:00Z".to_string(),
    });
    sequence.update();
    assert_eq!(sequence.step(), 3);
    assert_eq!(sequence.len(), 3); // Now has 3 points total
    assert_eq!(sequence.current()[0].id, 1);
    assert_eq!(sequence.current()[1].id, 2);
    assert_eq!(sequence.current()[2].id, 3);
}

#[test]
fn test_data_point_sequence_add_multiple_points() {
    let mut sequence = DataPointSequence::new(100, 10);
    
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
fn test_data_point_sequence_sequential_updates_accumulate() {
    let mut sequence = DataPointSequence::new(100, 10);
    
    // Simulate a time series with sequential updates that accumulate
    for i in 1..=5 {
        sequence.add_point(DataPoint {
            id: i,
            value: i as f64 * 10.0,
            timestamp: format!("2025-10-27T12:0{}:00Z", i),
        });
        sequence.update();
        
        assert_eq!(sequence.step(), i as usize);
        assert_eq!(sequence.len(), i as usize); // Length grows with each step
        assert_eq!(sequence.current()[i as usize - 1].id, i); // Latest point
    }
}

#[test]
fn test_data_point_sequence_clear() {
    let mut sequence = DataPointSequence::new(100, 10);
    
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
    let mut sequence = DataPointSequence::new(100, 10);
    
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
    
    // Add new data while reading from current
    sequence.add_point(DataPoint {
        id: 2,
        value: 20.0,
        timestamp: "2025-10-27T12:01:00Z".to_string(),
    });
    
    // Current should still show only first point
    assert_eq!(sequence.current().len(), 1);
    assert_eq!(sequence.pending_count(), 1);
    
    // Update to see accumulated data
    sequence.update();
    assert_eq!(sequence.current().len(), 2); // Now has both points
    assert_eq!(sequence.current()[0].id, 1);
    assert_eq!(sequence.current()[1].id, 2);
}

#[test]
fn test_data_point_sequence_buffer_growth() {
    let mut sequence = DataPointSequence::new(100, 10);
    
    // Perform multiple updates to verify buffer grows
    for i in 1..=10 {
        sequence.add_point(DataPoint {
            id: i,
            value: i as f64 * 10.0,
            timestamp: format!("2025-10-27T12:0{}:00Z", i % 10),
        });
        sequence.update();
    }
    
    // Verify all points accumulated
    assert_eq!(sequence.len(), 10);
    assert_eq!(sequence.step(), 10);
    
    // Verify buffer has sufficient size
    assert!(sequence.buffer_size() >= 10);
}

#[test]
fn test_data_point_sequence_empty_update() {
    let mut sequence = DataPointSequence::new(100, 10);
    
    // Update without adding any points
    sequence.update();
    
    assert_eq!(sequence.step(), 1);
    assert_eq!(sequence.len(), 0);
    assert!(sequence.is_empty());
}

#[test]
fn test_data_point_sequence_varying_sizes() {
    let mut sequence = DataPointSequence::new(100, 10);
    
    // First update with 1 point
    sequence.add_point(DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    });
    sequence.update();
    assert_eq!(sequence.len(), 1);
    
    // Second update with 3 more points (accumulates)
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
    assert_eq!(sequence.len(), 4); // Now has 4 points total (1 + 3)
    
    // Third update with no new points
    sequence.update();
    assert_eq!(sequence.len(), 4); // Still has 4 points (nothing added)
}
