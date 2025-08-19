import * as wasm from '/wasm/unborsh_wasm.js';

// Initialize the WASM module (this ensures the panic hook is set)
await wasm.default();

/**
 * Analysis strategy enum
 */
export const AnalysisStrategy = {
    PATTERN: 0,
    RECURSIVE: 1,
    PROBABILISTIC: 2,
    COMPREHENSIVE: 3
};

/**
 * Unborsh - JavaScript wrapper for unborsh-wasm WebAssembly module
 */
export class Unborsh {
    /**
     * Analyze Borsh-serialized data with default options (Comprehensive strategy)
     * @param {Uint8Array} data - The Borsh serialized data to analyze
     * @returns {Object} - The analysis results
     */
    static analyze(data) {
        const jsonStr = wasm.analyze_borsh(data);
        return JSON.parse(jsonStr);
    }

    /**
     * Analyze Borsh-serialized data with a specific strategy
     * @param {Uint8Array} data - The Borsh serialized data to analyze
     * @param {number} strategy - The analysis strategy to use (see AnalysisStrategy enum)
     * @returns {Object} - The analysis results
     */
    static analyzeWithStrategy(data, strategy) {
        const jsonStr = wasm.analyze_borsh_with_strategy(data, strategy);
        return JSON.parse(jsonStr);
    }

    /**
     * Analyze Borsh-serialized data with custom options
     * @param {Uint8Array} data - The Borsh serialized data to analyze
     * @param {Object} options - Analysis options object
     * @returns {Object} - The analysis results
     */
    static analyzeWithOptions(data, options) {
        try {
            const strategy = options.strategy !== undefined ? options.strategy : AnalysisStrategy.COMPREHENSIVE;
            const maxDepth = options.maxDepth !== undefined ? options.maxDepth : 5;
            const minConfidence = options.minConfidence !== undefined ? options.minConfidence : 30;
            const includeRawBytes = options.includeRawBytes !== undefined ? options.includeRawBytes : false;
            const maxMatches = options.maxMatches !== undefined ? options.maxMatches : null;

            const jsonStr = wasm.analyze_borsh_with_options(
                data,
                strategy,
                maxDepth,
                minConfidence,
                includeRawBytes,
                maxMatches
            );
            return JSON.parse(jsonStr);
        } catch (err) {
            console.error("Error analyzing with options:", err);
            throw new Error(`Failed to analyze: ${err.message || "Unknown error"}`);
        }
    }

    /**
     * Analyze base64-encoded Borsh data
     * @param {string} base64 - Base64-encoded Borsh data
     * @returns {Object} - The analysis results
     */
    static analyzeBase64(base64) {
        const jsonStr = wasm.analyze_borsh_base64(base64);
        return JSON.parse(jsonStr);
    }

    /**
     * Analyze hex-encoded Borsh data
     * @param {string} hex - Hex-encoded Borsh data
     * @returns {Object} - The analysis results
     */
    static analyzeHex(hex) {
        const jsonStr = wasm.analyze_borsh_hex(hex);
        return JSON.parse(jsonStr);
    }

    /**
     * Extract a structure hypothesis about the data
     * @param {Uint8Array} data - The Borsh serialized data
     * @returns {string} - Structure hypothesis as Rust-like code
     */
    static extractStructureHypothesis(data) {
        return wasm.extract_structure_hypothesis(data);
    }

    /**
     * Try to interpret the data as a specific type
     * @param {Uint8Array} data - The Borsh serialized data
     * @param {string} typeName - Name of the type to interpret as
     * @returns {string} - Interpretation of the data
     */
    static interpretAs(data, typeName) {
        return wasm.interpret_as(data, typeName);
    }

    /**
     * Get all available pattern names
     * @returns {Array<string>} - List of pattern names
     */
    static getAvailablePatterns() {
        return wasm.get_available_patterns();
    }

    /**
     * Get pattern details by name
     * @param {string} patternName - Name of the pattern
     * @returns {Object} - Pattern details (name, description, example)
     */
    static getPatternDetails(patternName) {
        return wasm.get_pattern_details(patternName);
    }
}

/**
 * Utility functions for working with binary data
 */
export class BinaryUtils {
    /**
     * Convert a Uint8Array to a Base64 string
     * @param {Uint8Array} bytes - The byte array to convert
     * @returns {string} - Base64 encoded string
     */
    static toBase64(bytes) {
        let binary = '';
        const len = bytes.byteLength;
        for (let i = 0; i < len; i++) {
            binary += String.fromCharCode(bytes[i]);
        }
        return btoa(binary);
    }

    /**
     * Convert a Base64 string to a Uint8Array
     * @param {string} base64 - The Base64 string to convert
     * @returns {Uint8Array} - The decoded byte array
     */
    static fromBase64(base64) {
        const binary = atob(base64);
        const len = binary.length;
        const bytes = new Uint8Array(len);
        for (let i = 0; i < len; i++) {
            bytes[i] = binary.charCodeAt(i);
        }
        return bytes;
    }

    /**
     * Convert a Uint8Array to a hexadecimal string
     * @param {Uint8Array} bytes - The byte array to convert
     * @returns {string} - Hex encoded string
     */
    static toHex(bytes) {
        return Array.from(bytes)
            .map(b => b.toString(16).padStart(2, '0'))
            .join('');
    }

    /**
     * Convert a hexadecimal string to a Uint8Array
     * @param {string} hex - The hex string to convert
     * @returns {Uint8Array} - The decoded byte array
     */
    static fromHex(hex) {
        // Remove 0x prefix if present
        const cleanHex = hex.startsWith('0x') ? hex.slice(2) : hex;

        const len = cleanHex.length / 2;
        const bytes = new Uint8Array(len);
        for (let i = 0; i < len; i++) {
            bytes[i] = parseInt(cleanHex.substr(i * 2, 2), 16);
        }
        return bytes;
    }
}

// Export utility function to create sample data
export function createSampleData() {
    // Create a simple Borsh-encoded string "Hello World"
    const bytes = new Uint8Array([
        11, 0, 0, 0,                         // Length prefix (11) as u32 little-endian
        72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100  // "Hello World" in ASCII
    ]);
    return bytes;
}

// Export a more complex sample with multiple fields
export function createComplexSample() {
    // Example of a struct with multiple fields:
    // struct Example {
    //   name: String,
    //   value: u32,
    //   active: bool
    // }
    const bytes = new Uint8Array([
        5, 0, 0, 0,                   // Length of "Hello" as u32 (5)
        72, 101, 108, 108, 111,       // "Hello" in ASCII
        42, 0, 0, 0,                  // u32 value (42)
        1                             // bool value (true)
    ]);
    return bytes;
}