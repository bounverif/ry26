use ry26::ObjectPool;

#[test]
fn test_object_pool_creation() {
    let pool: ObjectPool<i32> = ObjectPool::new(10);
    assert_eq!(pool.available_count(), 0);
}

#[test]
fn test_object_pool_acquire() {
    let mut pool: ObjectPool<i32> = ObjectPool::new(5);
    let vec = pool.acquire();
    assert_eq!(vec.len(), 0);
    // Vector is valid and empty
    assert!(vec.is_empty());
}

#[test]
fn test_object_pool_release() {
    let mut pool: ObjectPool<i32> = ObjectPool::new(5);

    let mut vec = pool.acquire();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    pool.release(vec);
    assert_eq!(pool.available_count(), 1);
}

#[test]
fn test_object_pool_reuse() {
    let mut pool: ObjectPool<String> = ObjectPool::new(5);

    // Acquire and release a vector
    let mut vec1 = pool.acquire();
    vec1.push("test".to_string());
    pool.release(vec1);

    // Acquire again - should get the same vector (cleared)
    let vec2 = pool.acquire();
    assert_eq!(vec2.len(), 0);
    assert_eq!(pool.available_count(), 0);
}

#[test]
fn test_object_pool_capacity_limit() {
    let mut pool: ObjectPool<i32> = ObjectPool::new(3);

    // Create and release more vectors than capacity
    let mut vecs = Vec::new();
    for _ in 0..5 {
        vecs.push(Vec::new());
    }

    for vec in vecs {
        pool.release(vec);
    }

    // Should not exceed capacity
    assert_eq!(pool.available_count(), 3);
}

#[test]
fn test_object_pool_multiple_acquire_release() {
    let mut pool: ObjectPool<u64> = ObjectPool::new(10);

    let vec1 = pool.acquire();
    let vec2 = pool.acquire();
    let vec3 = pool.acquire();

    assert_eq!(pool.available_count(), 0);

    pool.release(vec1);
    pool.release(vec2);
    pool.release(vec3);

    assert_eq!(pool.available_count(), 3);
}

#[test]
fn test_object_pool_cleared_on_release() {
    let mut pool: ObjectPool<i32> = ObjectPool::new(5);

    let mut vec = pool.acquire();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    assert_eq!(vec.len(), 5);

    pool.release(vec);

    let reused_vec = pool.acquire();
    assert_eq!(reused_vec.len(), 0);
}

#[test]
fn test_object_pool_with_different_types() {
    let mut int_pool: ObjectPool<i32> = ObjectPool::new(5);
    let mut string_pool: ObjectPool<String> = ObjectPool::new(5);

    let int_vec = int_pool.acquire();
    let string_vec = string_pool.acquire();

    int_pool.release(int_vec);
    string_pool.release(string_vec);

    assert_eq!(int_pool.available_count(), 1);
    assert_eq!(string_pool.available_count(), 1);
}

#[test]
fn test_object_pool_large_capacity() {
    let mut pool: ObjectPool<i32> = ObjectPool::new(1000);

    // Create and release many vectors
    let mut vecs = Vec::new();
    for _ in 0..100 {
        vecs.push(Vec::new());
    }

    for vec in vecs {
        pool.release(vec);
    }

    assert_eq!(pool.available_count(), 100);
}

#[test]
fn test_object_pool_zero_capacity() {
    let mut pool: ObjectPool<i32> = ObjectPool::new(0);

    let vec = pool.acquire();
    pool.release(vec);

    // With zero capacity, nothing should be stored
    assert_eq!(pool.available_count(), 0);
}

#[test]
fn test_object_pool_stress_test() {
    let mut pool: ObjectPool<Vec<i32>> = ObjectPool::new(10);

    // Simulate heavy usage
    for round in 0..100 {
        let mut vec = pool.acquire();
        for i in 0..round % 10 {
            vec.push(vec![i as i32]);
        }
        pool.release(vec);
    }

    // Pool should maintain its limit
    assert!(pool.available_count() <= 10);
}
