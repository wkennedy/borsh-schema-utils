# unborsh-wasm

WebAssembly bindings for [unborsh](https://github.com/yourusername/unborsh) - a tool for analyzing and reverse-engineering [Borsh](https://borsh.io) serialized data.

## Overview

`unborsh-wasm` allows you to use the powerful Borsh analysis and reverse-engineering tools from the `unborsh` library in JavaScript environments like browsers and Node.js. This makes it easy to:

- Analyze unknown Borsh-serialized data
- Extract structure and type information
- Identify patterns in binary data
- Generate structure hypotheses
- Debug serialization issues
- Understand on-chain data from Borsh-based blockchains

## Installation

### NPM

```bash
npm install unborsh-wasm
```

### Yarn

```bash
yarn add unborsh-wasm
```

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/unborsh.git
cd unborsh/unborsh-wasm

# Install development dependencies
npm install

# Build the WebAssembly module for all targets
npm run build

# Run the demo
npm run demo
```

## Usage

### Basic Usage

```javascript
import { Unborsh, BinaryUtils, createSampleData } from 'unborsh-wasm';

// Analyze Borsh-encoded data
const data = createSampleData(); // A simple "Hello World" string in Borsh format
const result = Unborsh.analyze(data);

console.log('Analysis result:', result);
console.log('Structure hypothesis:', Unborsh.extractStructureHypothesis(data));
```

### Analyzing Different Data Formats

```javascript
// From Uint8Array
const binaryData = new Uint8Array([11, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);
const resultBinary = Unborsh.analyze(binaryData);

// From Hex string
const hexData = "0b00000048656c6c