use ry26::FlatObjectPool;

#[test]
fn test_flat_pool_creation() {
    let pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    assert_eq!(pool.buffer_size(), 100);
    assert_eq!(pool.available_count(), 0);
}

#[test]
fn test_flat_pool_acquire() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    let (begin, end) = pool.acquire(10);
    assert_eq!(end - begin, 10);
    assert!(begin < pool.buffer_size());
    assert!(end <= pool.buffer_size());
}

#[test]
fn test_flat_pool_acquire_and_release() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    let (begin, end) = pool.acquire(10);
    assert_eq!(pool.available_count(), 0);
    
    pool.release(begin, end);
    assert_eq!(pool.available_count(), 1);
}

#[test]
fn test_flat_pool_reuse() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    // Acquire and release a range
    let (begin1, end1) = pool.acquire(10);
    pool.release(begin1, end1);
    
    // Acquire again - should reuse the same range
    let (begin2, end2) = pool.acquire(10);
    assert_eq!(begin1, begin2);
    assert_eq!(end1, end2);
}

#[test]
fn test_flat_pool_set_and_get() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    let (begin, end) = pool.acquire(10);
    
    // Set values
    for i in begin..end {
        pool.set(i, (i * 2) as i32);
    }
    
    // Get values
    for i in begin..end {
        assert_eq!(pool.get(i), Some(&((i * 2) as i32)));
    }
}

#[test]
fn test_flat_pool_get_slice() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    let (begin, end) = pool.acquire(10);
    
    // Set values
    for i in begin..end {
        pool.set(i, i as i32);
    }
    
    // Get slice
    let slice = pool.get_slice(begin, end);
    assert_eq!(slice.len(), 10);
    for (idx, &val) in slice.iter().enumerate() {
        assert_eq!(val, (begin + idx) as i32);
    }
}

#[test]
fn test_flat_pool_get_slice_mut() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    let (begin, end) = pool.acquire(10);
    
    // Modify via mutable slice
    {
        let slice = pool.get_slice_mut(begin, end);
        for (idx, val) in slice.iter_mut().enumerate() {
            *val = idx as i32 * 3;
        }
    }
    
    // Verify changes
    let slice = pool.get_slice(begin, end);
    for (idx, &val) in slice.iter().enumerate() {
        assert_eq!(val, idx as i32 * 3);
    }
}

#[test]
fn test_flat_pool_multiple_ranges() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    let (b1, e1) = pool.acquire(10);
    let (b2, e2) = pool.acquire(15);
    let (b3, e3) = pool.acquire(20);
    
    // Ranges should not overlap
    assert!(e1 <= b2);
    assert!(e2 <= b3);
    
    // Set different values in each range
    for i in b1..e1 {
        pool.set(i, 100);
    }
    for i in b2..e2 {
        pool.set(i, 200);
    }
    for i in b3..e3 {
        pool.set(i, 300);
    }
    
    // Verify isolation
    assert_eq!(pool.get(b1), Some(&100));
    assert_eq!(pool.get(b2), Some(&200));
    assert_eq!(pool.get(b3), Some(&300));
}

#[test]
fn test_flat_pool_release_clears_data() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    let (begin, end) = pool.acquire(10);
    
    // Set values
    for i in begin..end {
        pool.set(i, 42);
    }
    
    // Release
    pool.release(begin, end);
    
    // Values should be cleared to default
    for i in begin..end {
        assert_eq!(pool.get(i), Some(&0));
    }
}

#[test]
fn test_flat_pool_capacity_limit() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 3);
    
    // Acquire and release more ranges than capacity
    for _ in 0..5 {
        let (b, e) = pool.acquire(5);
        pool.release(b, e);
    }
    
    // Should not exceed capacity
    assert!(pool.available_count() <= 3);
}

#[test]
fn test_flat_pool_buffer_extension() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(10, 5);
    
    let initial_size = pool.buffer_size();
    
    // Acquire more than initial buffer size
    let (_, end) = pool.acquire(20);
    
    // Buffer should have grown
    assert!(pool.buffer_size() > initial_size);
    assert!(end <= pool.buffer_size());
}

#[test]
fn test_flat_pool_invalid_range() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    // Release invalid range (begin >= end)
    pool.release(50, 50);
    pool.release(60, 50);
    
    // Should not crash, just ignore
    assert_eq!(pool.available_count(), 0);
}

#[test]
fn test_flat_pool_partial_range_reuse() {
    let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 10);
    
    // Acquire and release a large range
    let (begin, end) = pool.acquire(20);
    pool.release(begin, end);
    
    // Acquire a smaller range - should reuse part of it
    let (begin2, end2) = pool.acquire(10);
    assert_eq!(begin, begin2);
    assert_eq!(end2 - begin2, 10);
    
    // Should have leftover in free list
    assert!(pool.available_count() > 0);
}

#[test]
fn test_flat_pool_with_strings() {
    let mut pool: FlatObjectPool<String> = FlatObjectPool::new(50, 5);
    
    let (begin, end) = pool.acquire(5);
    
    // Set string values
    for i in begin..end {
        pool.set(i, format!("String {}", i));
    }
    
    // Verify
    for i in begin..end {
        assert_eq!(pool.get(i), Some(&format!("String {}", i)));
    }
    
    // Release should clear
    pool.release(begin, end);
    
    for i in begin..end {
        assert_eq!(pool.get(i), Some(&String::new()));
    }
}
