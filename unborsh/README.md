# unborsh

Tools for analyzing and reverse-engineering [Borsh](https://borsh.io) serialized data without requiring the original schema.

## Overview

`unborsh` helps you inspect, analyze, and recover structure from binary data that has been serialized using the Borsh binary format. This is particularly useful for:

- Understanding unknown data structures
- Debugging serialization issues
- Working with on-chain data in Borsh-based blockchains (like Solana)
- Reverse engineering binary formats
- Recovering data when schemas are unavailable or have changed

## Features

- **Multiple Analysis Strategies**:
    - **Pattern-based**: Identify known Borsh serialization patterns
    - **Recursive**: Break down complex nested structures
    - **Probabilistic**: Make educated guesses about data structure
    - **Comprehensive**: Combine all approaches for best results

- **Rich Pattern Dictionary**:
    - Primitive types (integers, floats, booleans)
    - Collection types (String, Vec, Option, Result)
    - Complex types (enums, structs, tuples)
    - Blockchain-specific patterns (public keys, signatures, timestamps)

- **Flexible API**:
    - Simple one-line analysis for quick results
    - Detailed options for fine-grained control
    - Extensible with custom patterns and analyzers

## Installation

Add `unborsh` to your `Cargo.toml`:

```toml
[dependencies]
unborsh = "0.1.0"
```

## Usage Examples

### Basic Analysis

```rust
use unborsh::analyze;

// Analyze Borsh-serialized bytes
let data = vec![11, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]; // "Hello World"
let result = analyze(&data);
println!("{}", result);
```

### Using Different Strategies

```rust
use unborsh::{analyze_with_strategy, AnalysisStrategy};

let data = get_borsh_data();

// Pattern matching
let result_pattern = analyze_with_strategy(&data, AnalysisStrategy::Pattern);

// Recursive analysis
let result_recursive = analyze_with_strategy(&data, AnalysisStrategy::Recursive);

// Probabilistic analysis
let result_probabilistic = analyze_with_strategy(&data, AnalysisStrategy::Probabilistic);
```

### Custom Analysis Options

```rust
use unborsh::{analyze_with_options, AnalysisOptions, AnalysisStrategy};

let options = AnalysisOptions {
    strategy: AnalysisStrategy::Comprehensive,
    max_depth: 5,
    min_confidence: 60,
    include_raw_bytes: true,
    max_matches: Some(20),
};

let result = analyze_with_options(&data, options);
```

### Working with Hex Strings

```rust
use unborsh::analyze_hex;

let hex_string = "0b000000546f6b656e3a20425443";
match analyze_hex(hex_string) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("Invalid hex: {}", e),
}
```

### Working with Base64 (with feature flag)

```rust
use unborsh::analyze_base64;

let base64_string = "CwAAAEhlbGxvIFdvcmxk";
match analyze_base64(base64_string) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("Invalid base64: {}", e),
}
```

## Custom Patterns

You can extend the library with your own patterns:

```rust
use unborsh::{BorshPattern, PatternDictionary, analyze_with_dictionary, AnalysisOptions};

// Create a custom pattern
let my_pattern = BorshPattern::new(
    "MyCustomType",
    "Description of my custom type",
    vec![0x01, 0x02, 0x03, 0x04],  // Expected binary pattern
    vec![0xFF, 0x00, 0xFF, 0x00],  // Mask (0xFF = exact match, 0x00 = any value)
    "Example in hex",
    |bytes| {
        // Pattern matcher function
        if bytes.len() >= 4 && bytes[0] == 0x01 && bytes[2] == 0x03 {
            Some(format!("MyCustomType: value={}", bytes[1]))
        } else {
            None
        }
    }
);

// Create a dictionary with our pattern
let mut dictionary = PatternDictionary::new();
dictionary.add_pattern(my_pattern);

// Analyze with custom dictionary
let result = analyze_with_dictionary(&data, &dictionary, AnalysisOptions::default());
```

## Feature Flags

- `base64_support` - Enables Base64 encoding/decoding support
- `serde_support` - Enables serialization/deserialization of analysis results with serde

Enable all features:

```toml
[dependencies]
unborsh = { version = "0.1.0", features = ["all"] }
```