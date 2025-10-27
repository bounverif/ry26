# ry26

A simple Rust library with command line interface for data point generation and manipulation.

## Features

- Add two numbers together
- Generate random data points with timestamps
- JSON serialization and deserialization
- Command line interface for all library functions
- **Double buffering with object pool** - Efficient sequential update of object vectors using memory pooling
- **Flat object pool** - Advanced memory management with contiguous storage and begin/end pointers for better cache locality

## Installation

```bash
cargo build --release
```

## CLI Usage

The library includes a command line interface that exposes all main functions:

### Add two numbers

```bash
ry26 add 5 10
# Output: 15
```

### Generate a random data point

```bash
ry26 generate
# Output: {"id":625,"value":66.61,"timestamp":"2025-10-26T17:17:52.015300268+00:00"}
```

### Convert data point to JSON

```bash
ry26 to-json --id 42 --value 3.14 --timestamp "2025-10-26T12:00:00Z"
# Output: {"id":42,"value":3.14,"timestamp":"2025-10-26T12:00:00Z"}
```

### Parse JSON to data point

```bash
ry26 from-json '{"id":100,"value":50.5,"timestamp":"2025-10-26T15:30:00Z"}'
# Output:
# ID: 100
# Value: 50.5
# Timestamp: 2025-10-26T15:30:00Z
```

### Get help

```bash
ry26 --help
ry26 add --help
```

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
ry26 = "0.1.0"
```

Use in your code:

```rust
use ry26::{add, generate_random_data_point, to_json, from_json};

// Add numbers
let result = add(5, 10);

// Generate random data point
let data = generate_random_data_point();

// Convert to JSON
let json = to_json(&data).unwrap();

// Parse from JSON
let parsed = from_json(&json).unwrap();
```

### Double Buffering with Object Pool

The library includes a double buffering system with object pooling for efficient management of collections:

#### DataPointSequence - High-Level API

For managing immutable, append-only sequences of DataPoints with automatic step tracking:

```rust
use ry26::{DataPointSequence, DataPoint};

// Create a sequence with buffer size 1000 and pool capacity 10
let mut sequence = DataPointSequence::new(1000, 10);

// Step 1: Add initial data point
sequence.add_point(DataPoint {
    id: 1,
    value: 42.0,
    timestamp: "2025-10-27T12:00:00Z".to_string(),
});
sequence.update();

// Read current sequence (contains 1 point)
let current = sequence.current();
println!("Step {}: {} points", sequence.step(), current.len()); // Step 1: 1 points

// Step 2: Add more points (accumulates, not replaces!)
sequence.add_points(vec![
    DataPoint { id: 2, value: 84.0, timestamp: "2025-10-27T12:01:00Z".to_string() },
    DataPoint { id: 3, value: 126.0, timestamp: "2025-10-27T12:02:00Z".to_string() },
]);
sequence.update();

// Now contains all 3 points from both steps
println!("Step {}: {} points", sequence.step(), current.len()); // Step 2: 3 points
```

**Key Features:**
- **Immutable objects**: DataPoints are never modified once added
- **Append-only**: Objects accumulate over steps, never erased
- **Flat buffer**: Uses FlatObjectPool for efficient contiguous storage

#### DoubleBuffer - Low-Level API

For more control over double buffering with any type:

```rust
use ry26::{DoubleBuffer, DataPoint};

// Create a double buffer with object pooling
let mut buffer: DoubleBuffer<DataPoint> = DoubleBuffer::new(10);

// Write to back buffer
buffer.back_mut().push(DataPoint {
    id: 1,
    value: 42.0,
    timestamp: "2025-10-27T12:00:00Z".to_string(),
});

// Swap buffers - back becomes front, old front returns to pool
buffer.swap();

// Read from front buffer while writing to back
let front_data = buffer.front();
buffer.back_mut().push(DataPoint {
    id: 2,
    value: 84.0,
    timestamp: "2025-10-27T12:01:00Z".to_string(),
});
```

#### ObjectPool - Memory Management

Use the object pool independently for efficient vector reuse:

```rust
use ry26::ObjectPool;

let mut pool: ObjectPool<i32> = ObjectPool::new(5);
let vec = pool.acquire();
pool.release(vec);  // Returns vector to pool for reuse
```

#### FlatObjectPool - Advanced Memory Management

For even better cache locality and reduced fragmentation, use the flat object pool with begin/end pointers:

```rust
use ry26::FlatObjectPool;

// Create a flat pool with buffer size 1000 and capacity for 10 free ranges
let mut pool: FlatObjectPool<i32> = FlatObjectPool::new(1000, 10);

// Acquire a slice of 50 elements
let (begin, end) = pool.acquire(50);

// Work with the slice
for i in begin..end {
    pool.set(i, i as i32 * 2);
}

// Get values
let slice = pool.get_slice(begin, end);
println!("Slice length: {}", slice.len());

// Release back to pool when done
pool.release(begin, end);
```

The double buffering technique allows for:
- **Non-blocking reads**: Front buffer can be read while back buffer is being written
- **Sequential updates**: Swap buffers to atomically update the collection
- **Memory efficiency**: Object pool reuses allocated vectors, reducing allocations
- **Flat memory layout**: FlatObjectPool provides better cache locality with contiguous storage
- **Append-only semantics**: DataPointSequence accumulates immutable objects over time
- **Step tracking**: DataPointSequence automatically tracks update steps

## Testing

```bash
cargo test
```

## License

See LICENSE file for details.