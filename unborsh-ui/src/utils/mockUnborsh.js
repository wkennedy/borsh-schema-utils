/**
 * Mock implementation of the Unborsh module for testing purposes
 */

export const AnalysisStrategy = {
    PATTERN: 0,
    RECURSIVE: 1,
    PROBABILISTIC: 2,
    COMPREHENSIVE: 3
};

/**
 * Mock Unborsh class with analysis methods
 */
export class Unborsh {
    /**
     * Analyze data with default settings
     */
    static analyze(data) {
        console.log('Mock analyze called with data:', data);
        return createMockResult();
    }

    /**
     * Analyze with a specific strategy
     */
    static analyzeWithStrategy(data, strategy) {
        console.log('Mock analyzeWithStrategy called with strategy:', strategy);
        return createMockResult();
    }

    /**
     * Analyze with custom options
     */
    static analyzeWithOptions(data, options) {
        console.log('Mock analyzeWithOptions called with options:', options);
        return createMockResult();
    }

    /**
     * Extract structure hypothesis
     */
    static extractStructureHypothesis(data) {
        return `struct MockStructure {
    field_0: String,
    field_1: u32,
    field_2: bool,
}`;
    }

    /**
     * Interpret as a specific pattern
     */
    static interpretAs(data, pattern) {
        return `Interpreted as ${pattern}: Mock interpretation`;
    }

    /**
     * Get available patterns
     */
    static getAvailablePatterns() {
        return ['String', 'u8', 'u32', 'bool', 'Vec<T>', 'Option<T>'];
    }

    /**
     * Get pattern details
     */
    static getPatternDetails(pattern) {
        return {
            name: pattern,
            description: `Mock description for ${pattern}`,
            example: `Example of ${pattern}`
        };
    }
}

/**
 * Create a mock analysis result
 */
function createMockResult() {
    return {
        matches: [
            {
                pattern_name: 'String',
                offset: 0,
                length: 15,
                data: new Uint8Array([11, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]),
                interpretation: 'String: "Hello World"',
                confidence: 90
            },
            {
                pattern_name: 'u32',
                offset: 15,
                length: 4,
                data: new Uint8Array([42, 0, 0, 0]),
                interpretation: 'u32: 42',
                confidence: 80
            },
            {
                pattern_name: 'bool',
                offset: 19,
                length: 1,
                data: new Uint8Array([1]),
                interpretation: 'Boolean: true',
                confidence: 85
            }
        ],
        structure_hypothesis: `struct MockStructure {
    field_0: String, // "Hello World"
    field_1: u32,    // 42
    field_2: bool,   // true
}`,
        confidence: 85,
        description: 'Mock analysis results for testing'
    };
}

/**
 * Create sample data for testing
 */
export function createSampleData() {
    // Sample Borsh-encoded string "Hello World" followed by u32(42) and bool(true)
    return new Uint8Array([
        // String "Hello World"
        11, 0, 0, 0,                         // Length prefix (11) as u32 little-endian
        72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100,  // "Hello World" in ASCII
        // u32 value (42)
        42, 0, 0, 0,
        // bool value (true)
        1
    ]);
}

/**
 * Create complex sample for testing
 */
export function createComplexSample() {
    // Same as sample data but with different values
    return new Uint8Array([
        // String "Complex"
        7, 0, 0, 0,                          // Length prefix (7) as u32 little-endian
        67, 111, 109, 112, 108, 101, 120,    // "Complex" in ASCII
        // u32 value (255)
        255, 0, 0, 0,
        // bool value (false)
        0
    ]);
}

/**
 * Utility functions for binary data
 */
export class BinaryUtils {
    /**
     * Convert bytes to base64
     */
    static toBase64(bytes) {
        // Implementation in binaryUtils.js
        let binary = '';
        const len = bytes.byteLength;
        for (let i = 0; i < len; i++) {
            binary += String.fromCharCode(bytes[i]);
        }
        return window.btoa(binary);
    }

    /**
     * Convert base64 to bytes
     */
    static fromBase64(base64) {
        // Implementation in binaryUtils.js
        const binary = window.atob(base64);
        const len = binary.length;
        const bytes = new Uint8Array(len);
        for (let i = 0; i < len; i++) {
            bytes[i] = binary.charCodeAt(i);
        }
        return bytes;
    }

    /**
     * Convert bytes to hex
     */
    static toHex(bytes) {
        // Implementation in binaryUtils.js
        return Array.from(bytes)
            .map(b => b.toString(16).padStart(2, '0'))
            .join('');
    }

    /**
     * Convert hex to bytes
     */
    static fromHex(hex) {
        // Implementation in binaryUtils.js
        const cleanHex = hex.startsWith('0x') ? hex.slice(2) : hex;
        const len = cleanHex.length / 2;
        const bytes = new Uint8Array(len);
        for (let i = 0; i < len; i++) {
            bytes[i] = parseInt(cleanHex.substr(i * 2, 2), 16);
        }
        return bytes;
    }
}