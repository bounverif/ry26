use ry26::{DataPoint, DoubleBuffer};

#[test]
fn test_double_buffer_creation() {
    let buffer: DoubleBuffer<i32> = DoubleBuffer::new(10);
    assert_eq!(buffer.front().len(), 0);
}

#[test]
fn test_double_buffer_write_to_back() {
    let mut buffer: DoubleBuffer<i32> = DoubleBuffer::new(5);

    buffer.back_mut().push(1);
    buffer.back_mut().push(2);
    buffer.back_mut().push(3);

    // Front should still be empty
    assert_eq!(buffer.front().len(), 0);
}

#[test]
fn test_double_buffer_swap() {
    let mut buffer: DoubleBuffer<i32> = DoubleBuffer::new(5);

    // Write to back buffer
    buffer.back_mut().push(1);
    buffer.back_mut().push(2);
    buffer.back_mut().push(3);

    // Swap buffers
    buffer.swap();

    // Now front should have the data
    assert_eq!(buffer.front().len(), 3);
    assert_eq!(buffer.front()[0], 1);
    assert_eq!(buffer.front()[1], 2);
    assert_eq!(buffer.front()[2], 3);

    // Back should be empty (fresh from pool)
    assert_eq!(buffer.back_mut().len(), 0);
}

#[test]
fn test_double_buffer_sequential_updates() {
    let mut buffer: DoubleBuffer<String> = DoubleBuffer::new(10);

    // First update
    buffer.back_mut().push("First".to_string());
    buffer.back_mut().push("Update".to_string());
    buffer.swap();

    assert_eq!(buffer.front().len(), 2);
    assert_eq!(buffer.front()[0], "First");

    // Second update
    buffer.back_mut().push("Second".to_string());
    buffer.back_mut().push("Update".to_string());
    buffer.swap();

    assert_eq!(buffer.front().len(), 2);
    assert_eq!(buffer.front()[0], "Second");
}

#[test]
fn test_double_buffer_with_data_points() {
    let mut buffer: DoubleBuffer<DataPoint> = DoubleBuffer::new(5);

    let dp1 = DataPoint {
        id: 1,
        value: 10.0,
        timestamp: "2025-10-27T12:00:00Z".to_string(),
    };

    let dp2 = DataPoint {
        id: 2,
        value: 20.0,
        timestamp: "2025-10-27T12:01:00Z".to_string(),
    };

    buffer.back_mut().push(dp1.clone());
    buffer.back_mut().push(dp2.clone());
    buffer.swap();

    assert_eq!(buffer.front().len(), 2);
    assert_eq!(buffer.front()[0].id, 1);
    assert_eq!(buffer.front()[1].id, 2);
}

#[test]
fn test_double_buffer_clear() {
    let mut buffer: DoubleBuffer<i32> = DoubleBuffer::new(5);

    buffer.back_mut().push(1);
    buffer.back_mut().push(2);
    buffer.swap();

    buffer.back_mut().push(3);
    buffer.back_mut().push(4);

    // Clear both buffers
    buffer.clear();

    assert_eq!(buffer.front().len(), 0);
    assert_eq!(buffer.back_mut().len(), 0);
}

#[test]
fn test_double_buffer_multiple_swaps() {
    let mut buffer: DoubleBuffer<u64> = DoubleBuffer::new(8);

    for i in 0..5 {
        buffer.back_mut().push(i * 10);
        buffer.back_mut().push(i * 10 + 1);
        buffer.swap();

        assert_eq!(buffer.front().len(), 2);
        assert_eq!(buffer.front()[0], i * 10);
    }
}

#[test]
fn test_double_buffer_pool_utilization() {
    let mut buffer: DoubleBuffer<i32> = DoubleBuffer::new(10);

    // Initially, pool should be empty
    let initial_available = buffer.pool_available();

    // Write and swap
    buffer.back_mut().push(1);
    buffer.swap();

    // After swap, one vector should be returned to pool
    let after_swap = buffer.pool_available();
    assert!(after_swap >= initial_available);
}

#[test]
fn test_double_buffer_read_while_write() {
    let mut buffer: DoubleBuffer<i32> = DoubleBuffer::new(5);

    // Setup initial data in front buffer
    buffer.back_mut().push(1);
    buffer.back_mut().push(2);
    buffer.swap();

    // Read from front while writing to back
    let front_data: Vec<i32> = buffer.front().to_vec();
    buffer.back_mut().push(3);
    buffer.back_mut().push(4);

    // Front should still have original data
    assert_eq!(front_data.len(), 2);
    assert_eq!(buffer.front().len(), 2);
    assert_eq!(buffer.front()[0], 1);

    // Swap and verify new data
    buffer.swap();
    assert_eq!(buffer.front().len(), 2);
    assert_eq!(buffer.front()[0], 3);
}
