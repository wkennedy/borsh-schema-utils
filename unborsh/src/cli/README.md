# unborsh CLI Tool

The `unborsh` command-line interface provides a powerful way to analyze Borsh-serialized data directly from the terminal.

## Installation

```bash
# Install with CLI support
cargo install unborsh --features cli

# Or install with all features
cargo install unborsh --features all
```

## Basic Usage

```bash
# Analyze a Borsh file
unborsh analyze data.bin

# Analyze hex data
unborsh analyze 0b000000546f6b656e3a20425443

# Analyze data from stdin
cat data.bin | unborsh analyze -
```

## Command Reference

### analyze

Analyze Borsh-serialized data with various strategies and options.

```bash
unborsh analyze [OPTIONS] <INPUT>
```

**Arguments:**
- `<INPUT>`: Path to a file, hex string, or `-` for stdin

**Options:**
- `-s, --strategy <STRATEGY>`: Analysis strategy to use [default: comprehensive]
    - `pattern`: Use pattern matching only
    - `recursive`: Use recursive structure analysis
    - `probabilistic`: Use probabilistic analysis
    - `comprehensive`: Combine all strategies (default)
- `-d, --depth <DEPTH>`: Maximum recursion depth [default: 5]
- `-c, --confidence <CONFIDENCE>`: Minimum confidence threshold (0-100) [default: 30]
- `-r, --raw-bytes`: Include raw bytes in output
- `-f, --format <FORMAT>`: Output format [default: text]
    - `text`: Human-readable text output
    - `json`: JSON format for machine processing
    - `hex`: Hexdump with highlighted matches
    - `compact`: Compact output for scripting
- `-m, --max-matches <MAX_MATCHES>`: Maximum number of matches to display

### match

Check if data matches a specific pattern.

```bash
unborsh match <INPUT> <PATTERN>
```

**Arguments:**
- `<INPUT>`: Path to a file, hex string, or `-` for stdin
- `<PATTERN>`: Pattern name to match against (e.g., "String", "u32", "PublicKey")

### patterns

List available pattern definitions.

```bash
unborsh patterns [OPTIONS]
```

**Options:**
- `-c, --category <CATEGORY>`: Filter patterns by category
    - `primitives`: Primitive types (u8, i32, bool, etc.)
    - `collections`: Collection types (String, Vec, Option, etc.)
    - `complex`: Complex types (Enum, Struct, etc.)
    - `blockchain`: Blockchain-specific types
    - `all`: All pattern types (default)
- `-v, --verbose`: Show detailed pattern descriptions

## Examples

### Analyze a file with default options

```bash
unborsh analyze my_data.bin
```

### Analyze with specific strategy and format

```bash
unborsh analyze my_data.bin --strategy recursive --format json
```

### Use hexdump output with highlighted matches

```bash
unborsh analyze my_data.bin --format hex
```

### Check specific pattern match

```bash
unborsh match my_data.bin "PublicKey"
```

### Analyze string data from a hex representation

```bash
unborsh analyze 0b000000546f6b656e3a20425443 --strategy pattern
```

### List all blockchain-specific patterns

```bash
unborsh patterns --category blockchain --verbose
```

## Output Formats

### Text (default)

Human-readable output with colorized sections showing:
- Overall confidence score
- List of matches with interpretations
- Structure hypothesis

### JSON

Machine-readable JSON output containing:
- Full analysis results
- All matched patterns
- Structure hypothesis
- Confidence scores

### Hex

Hexdump view with color-highlighted matches:
- Green: High confidence matches (70-100%)
- Yellow: Medium confidence matches (50-69%)
- Red: Low confidence matches (<50%)

### Compact

Compact output designed for scripting or parsing:
- One line per match in format: `pattern:offset:length:interpretation:confidence%`
- Simplified structure hypothesis