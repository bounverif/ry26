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

/// Flat object pool using a single contiguous buffer with begin/end pointers.
///
/// This structure uses a single flat `Vec<T>` as backing storage and tracks
/// vector slices using (begin, end) index pairs. This approach provides better
/// cache locality and reduces memory fragmentation compared to storing multiple
/// separate vectors.
///
/// # Examples
///
/// ```
/// use ry26::FlatObjectPool;
///
/// let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(100, 5);
///
/// // Acquire a slice from the pool
/// let (begin, end) = pool.acquire(10);
///
/// // Access the data via the pool
/// for i in begin..end {
///     pool.set(i, i as i32 * 2);
/// }
///
/// // Release the slice back to the pool
/// pool.release(begin, end);
/// ```
#[derive(Debug)]
pub struct FlatObjectPool<T> {
    buffer: Vec<T>,
    free_ranges: Vec<(usize, usize)>, // (begin, end) pairs
    capacity: usize,
}

impl<T: Default + Clone> FlatObjectPool<T> {
    /// Create a new flat object pool with the specified buffer size and capacity
    ///
    /// # Arguments
    /// * `buffer_size` - Total size of the backing buffer
    /// * `capacity` - Maximum number of free ranges to track
    pub fn new(buffer_size: usize, capacity: usize) -> Self {
        Self {
            buffer: vec![T::default(); buffer_size],
            free_ranges: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Acquire a slice of the specified size from the pool.
    ///
    /// Returns (begin, end) indices for the acquired slice.
    /// If no suitable range is available, extends the buffer.
    pub fn acquire(&mut self, size: usize) -> (usize, usize) {
        // Try to find a free range that fits
        for i in 0..self.free_ranges.len() {
            let (begin, end) = self.free_ranges[i];
            let range_size = end - begin;
            
            if range_size >= size {
                // Use this range
                self.free_ranges.remove(i);
                
                // If range is larger than needed, return the excess
                if range_size > size {
                    let new_begin = begin + size;
                    if self.free_ranges.len() < self.capacity {
                        self.free_ranges.push((new_begin, end));
                    }
                }
                
                return (begin, begin + size);
            }
        }
        
        // No suitable range found, extend buffer
        let begin = self.buffer.len();
        let end = begin + size;
        self.buffer.resize(end, T::default());
        (begin, end)
    }

    /// Release a slice back to the pool for reuse.
    ///
    /// The slice data is cleared and the range is added to the free list.
    pub fn release(&mut self, begin: usize, end: usize) {
        if begin >= end || end > self.buffer.len() {
            return; // Invalid range
        }
        
        // Clear the range
        for i in begin..end {
            self.buffer[i] = T::default();
        }
        
        // Add to free ranges if capacity allows
        if self.free_ranges.len() < self.capacity {
            self.free_ranges.push((begin, end));
        }
    }

    /// Get a reference to an element in the buffer
    pub fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }

    /// Get a mutable reference to an element in the buffer
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.buffer.get_mut(index)
    }

    /// Set the value at the specified index
    pub fn set(&mut self, index: usize, value: T) {
        if index < self.buffer.len() {
            self.buffer[index] = value;
        }
    }

    /// Get a slice of the buffer
    pub fn get_slice(&self, begin: usize, end: usize) -> &[T] {
        &self.buffer[begin..end]
    }

    /// Get a mutable slice of the buffer
    pub fn get_slice_mut(&mut self, begin: usize, end: usize) -> &mut [T] {
        &mut self.buffer[begin..end]
    }

    /// Get the number of available free ranges in the pool
    pub fn available_count(&self) -> usize {
        self.free_ranges.len()
    }

    /// Get the total buffer size
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
}

/// Object pool for managing reusable vector objects.
///
/// This structure maintains a pool of pre-allocated vectors that can be reused,
/// reducing the overhead of frequent allocations and deallocations.
///
/// # Examples
///
/// ```
/// use ry26::ObjectPool;
///
/// let mut pool: ObjectPool<i32> = ObjectPool::new(5);
///
/// // Acquire a vector from the pool
/// let mut vec = pool.acquire();
/// vec.push(1);
/// vec.push(2);
///
/// // Return it to the pool for reuse
/// pool.release(vec);
///
/// // The next acquire will reuse the vector (cleared)
/// let vec2 = pool.acquire();
/// assert_eq!(vec2.len(), 0);
/// ```
#[derive(Debug)]
pub struct ObjectPool<T> {
    available: Vec<Vec<T>>,
    capacity: usize,
}

impl<T> ObjectPool<T> {
    /// Create a new object pool with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            available: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Acquire a vector from the pool, or create a new one if none available.
    ///
    /// If the pool has an available vector, it will be returned (cleared).
    /// Otherwise, a new empty vector will be created.
    pub fn acquire(&mut self) -> Vec<T> {
        self.available.pop().unwrap_or_default()
    }

    /// Return a vector to the pool for reuse.
    ///
    /// The vector will be cleared before being added to the pool.
    /// If the pool is at capacity, the vector will be dropped instead.
    pub fn release(&mut self, mut vec: Vec<T>) {
        if self.available.len() < self.capacity {
            vec.clear();
            self.available.push(vec);
        }
    }

    /// Get the number of available vectors in the pool
    pub fn available_count(&self) -> usize {
        self.available.len()
    }
}

/// Double buffer for sequential updates of object vectors.
///
/// This structure implements a double buffering pattern where one buffer (front)
/// is available for reading while another buffer (back) can be written to.
/// When `swap()` is called, the buffers are exchanged atomically.
///
/// The double buffer integrates with an object pool to efficiently manage memory
/// by reusing vector allocations.
///
/// # Examples
///
/// ```
/// use ry26::{DoubleBuffer, DataPoint};
///
/// let mut buffer: DoubleBuffer<DataPoint> = DoubleBuffer::new(10);
///
/// // Write to back buffer
/// buffer.back_mut().push(DataPoint {
///     id: 1,
///     value: 42.0,
///     timestamp: "2025-10-27T12:00:00Z".to_string(),
/// });
///
/// // Swap buffers
/// buffer.swap();
///
/// // Now read from front while writing new data to back
/// assert_eq!(buffer.front().len(), 1);
/// assert_eq!(buffer.front()[0].id, 1);
///
/// buffer.back_mut().push(DataPoint {
///     id: 2,
///     value: 84.0,
///     timestamp: "2025-10-27T12:01:00Z".to_string(),
/// });
/// ```
#[derive(Debug)]
pub struct DoubleBuffer<T> {
    front: Vec<T>,
    back: Vec<T>,
    pool: ObjectPool<T>,
}

impl<T: Clone> DoubleBuffer<T> {
    /// Create a new double buffer with an object pool of the specified capacity
    pub fn new(pool_capacity: usize) -> Self {
        Self {
            front: Vec::new(),
            back: Vec::new(),
            pool: ObjectPool::new(pool_capacity),
        }
    }

    /// Get a reference to the front buffer (read buffer)
    pub fn front(&self) -> &[T] {
        &self.front
    }

    /// Get a mutable reference to the back buffer (write buffer)
    pub fn back_mut(&mut self) -> &mut Vec<T> {
        &mut self.back
    }

    /// Swap the front and back buffers.
    ///
    /// After swapping, the back buffer becomes the front buffer (for reading),
    /// and the old front buffer is returned to the pool. A fresh vector is
    /// acquired from the pool to become the new back buffer.
    ///
    /// This operation is efficient and allows for lock-free reading of the
    /// front buffer while the back buffer is being prepared.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
        // Return old front (now back) to pool and get a fresh vector
        let old_back = std::mem::take(&mut self.back);
        self.pool.release(old_back);
        self.back = self.pool.acquire();
    }

    /// Clear both buffers and return them to the pool
    pub fn clear(&mut self) {
        let front = std::mem::take(&mut self.front);
        let back = std::mem::take(&mut self.back);
        self.pool.release(front);
        self.pool.release(back);
        self.front = self.pool.acquire();
        self.back = self.pool.acquire();
    }

    /// Get the number of available vectors in the pool
    pub fn pool_available(&self) -> usize {
        self.pool.available_count()
    }
}

/// A sequence of DataPoint objects that can be efficiently updated using double buffering.
///
/// `DataPointSequence` wraps a `DoubleBuffer<DataPoint>` to provide a specialized
/// interface for managing sequences of data points. This allows for efficient
/// sequential updates where new data points can be added to the back buffer while
/// the front buffer remains available for reading.
///
/// # Examples
///
/// ```
/// use ry26::{DataPointSequence, DataPoint};
///
/// let mut sequence = DataPointSequence::new(10);
///
/// // Add data points to the sequence
/// sequence.add_point(DataPoint {
///     id: 1,
///     value: 10.0,
///     timestamp: "2025-10-27T12:00:00Z".to_string(),
/// });
///
/// // Update the sequence (swap buffers)
/// sequence.update();
///
/// // Read current sequence
/// let current = sequence.current();
/// assert_eq!(current.len(), 1);
/// ```
#[derive(Debug)]
pub struct DataPointSequence {
    buffer: DoubleBuffer<DataPoint>,
    step: usize,
}

impl DataPointSequence {
    /// Create a new DataPointSequence with the specified pool capacity
    pub fn new(pool_capacity: usize) -> Self {
        Self {
            buffer: DoubleBuffer::new(pool_capacity),
            step: 0,
        }
    }

    /// Get the current step number (number of updates performed)
    pub fn step(&self) -> usize {
        self.step
    }

    /// Add a data point to the next sequence (back buffer)
    pub fn add_point(&mut self, point: DataPoint) {
        self.buffer.back_mut().push(point);
    }

    /// Add multiple data points to the next sequence (back buffer)
    pub fn add_points(&mut self, points: impl IntoIterator<Item = DataPoint>) {
        self.buffer.back_mut().extend(points);
    }

    /// Update the sequence by swapping buffers and incrementing the step counter.
    ///
    /// This makes the back buffer (with newly added points) become the current
    /// sequence, and prepares a fresh back buffer for the next update.
    pub fn update(&mut self) {
        self.buffer.swap();
        self.step += 1;
    }

    /// Get a reference to the current sequence (front buffer)
    pub fn current(&self) -> &[DataPoint] {
        self.buffer.front()
    }

    /// Get the number of data points in the current sequence
    pub fn len(&self) -> usize {
        self.buffer.front().len()
    }

    /// Check if the current sequence is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.front().is_empty()
    }

    /// Clear all data points from both buffers and reset the step counter
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.step = 0;
    }

    /// Get the number of available vectors in the underlying object pool
    pub fn pool_available(&self) -> usize {
        self.buffer.pool_available()
    }
}
