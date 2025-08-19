/**
 * Create a simple Borsh-encoded string "Hello World" for testing
 * @returns {Uint8Array} A Uint8Array containing Borsh-encoded data
 */
export function createSimpleSample() {
    // Create a simple Borsh-encoded string "Hello World"
    return new Uint8Array([
        11, 0, 0, 0,                         // Length prefix (11) as u32 little-endian
        72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100  // "Hello World" in ASCII
    ]);
}

/**
 * Create a complex sample with multiple fields for testing
 * @returns {Uint8Array} A Uint8Array containing Borsh-encoded data
 */
export function createComplexSample() {
    // Example of a struct with multiple fields:
    // struct Example {
    //   name: String,
    //   value: u32,
    //   active: bool
    // }
    return new Uint8Array([
        5, 0, 0, 0,                   // Length of "Hello" as u32 (5)
        72, 101, 108, 108, 111,       // "Hello" in ASCII
        42, 0, 0, 0,                  // u32 value (42)
        1                             // bool value (true)
    ]);
}

/**
 * Create a more complex structure with different field types
 * @returns {Uint8Array} A Uint8Array containing Borsh-encoded data
 */
export function createCustomSample() {
    return new Uint8Array([
        // First field: a string "Borsh"
        5, 0, 0, 0,                   // Length of "Borsh" as u32 (5)
        66, 111, 114, 115, 104,       // "Borsh" in ASCII

        // Second field: an array of three u32 values
        3, 0, 0, 0,                   // Length of array as u32 (3)
        10, 0, 0, 0,                  // First u32 value (10)
        20, 0, 0, 0,                  // Second u32 value (20)
        30, 0, 0, 0,                  // Third u32 value (30)

        // Third field: Option<String> with Some("test")
        1,                            // Some variant (1)
        4, 0, 0, 0,                   // Length of "test" as u32 (4)
        116, 101, 115, 116,           // "test" in ASCII

        // Fourth field: a bool
        1                             // true
    ]);
}