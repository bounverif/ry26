# ry26

A simple Rust library with command line interface for data point generation and manipulation.

## Features

- Add two numbers together
- Generate random data points with timestamps
- JSON serialization and deserialization
- Command line interface for all library functions

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

## Testing

```bash
cargo test
```

## License

See LICENSE file for details.