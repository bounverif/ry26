# ry26

A simple Rust library with command line interface for data point generation and manipulation.

## Features

- Add two numbers together
- Generate random data points with timestamps
- JSON serialization and deserialization
- Command line interface for all library functions
- **Double buffering with object pool** - Efficient sequential update of object vectors using memory pooling

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

For managing sequences of DataPoints with automatic step tracking:

```rust
use ry26::{DataPointSequence, DataPoint};

// Create a sequence with object pooling
let mut sequence = DataPointSequence::new(10);

// Add data points for the next step
sequence.add_point(DataPoint {
    id: 1,
    value: 42.0,
    timestamp: "2025-10-27T12:00:00Z".to_string(),
});

// Update the sequence (swap buffers and increment step)
sequence.update();

// Read current sequence
let current = sequence.current();
println!("Step {}: {} points", sequence.step(), current.len());

// Add multiple points for next step
sequence.add_points(vec![
    DataPoint { id: 2, value: 84.0, timestamp: "2025-10-27T12:01:00Z".to_string() },
    DataPoint { id: 3, value: 126.0, timestamp: "2025-10-27T12:02:00Z".to_string() },
]);
sequence.update();
```

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

The double buffering technique allows for:
- **Non-blocking reads**: Front buffer can be read while back buffer is being written
- **Sequential updates**: Swap buffers to atomically update the collection
- **Memory efficiency**: Object pool reuses allocated vectors, reducing allocations
- **Step tracking**: DataPointSequence automatically tracks update steps

## Testing

```bash
cargo test
```

## License

See LICENSE file for details.