use crate::BorshPattern;

/// Create a pattern for u8 type
pub fn create_u8_pattern() -> BorshPattern {
    BorshPattern::new(
        "u8",
        "1-byte unsigned integer",
        vec![0x00],
        vec![0x00], // Variable
        "42 -> 42u8",
        |bytes| {
            if !bytes.is_empty() {
                Some(format!("u8: {}", bytes[0]))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for u16 type
pub fn create_u16_pattern() -> BorshPattern {
    BorshPattern::new(
        "u16",
        "2-byte little-endian unsigned integer",
        vec![0x00, 0x00],
        vec![0x00, 0x00], // All variable
        "e803 -> 1000 (decimal)",
        |bytes| {
            if bytes.len() >= 2 {
                let value = u16::from_le_bytes([bytes[0], bytes[1]]);
                Some(format!("u16: {}", value))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for u32 type
pub fn create_u32_pattern() -> BorshPattern {
    BorshPattern::new(
        "u32",
        "4-byte little-endian unsigned integer",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00], // All variable
        "e8030000 -> 1000 (decimal)",
        |bytes| {
            if bytes.len() >= 4 {
                let value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Some(format!("u32: {}", value))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for u64 type
pub fn create_u64_pattern() -> BorshPattern {
    BorshPattern::new(
        "u64",
        "8-byte little-endian unsigned integer",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "e803000000000000 -> 1000 (decimal)",
        |bytes| {
            if bytes.len() >= 8 {
                let value = u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                Some(format!("u64: {}", value))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for i8 type
pub fn create_i8_pattern() -> BorshPattern {
    BorshPattern::new(
        "i8",
        "1-byte signed integer",
        vec![0x00],
        vec![0x00], // Variable
        "d6 -> -42i8",
        |bytes| {
            if !bytes.is_empty() {
                // Convert u8 to i8 by interpreting the bit pattern
                let value = bytes[0] as i8;
                Some(format!("i8: {}", value))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for i16 type
pub fn create_i16_pattern() -> BorshPattern {
    BorshPattern::new(
        "i16",
        "2-byte little-endian signed integer",
        vec![0x00, 0x00],
        vec![0x00, 0x00], // All variable
        "18fc -> -1000 (decimal)",
        |bytes| {
            if bytes.len() >= 2 {
                let value = i16::from_le_bytes([bytes[0], bytes[1]]);
                Some(format!("i16: {}", value))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for i32 type
pub fn create_i32_pattern() -> BorshPattern {
    BorshPattern::new(
        "i32",
        "4-byte little-endian signed integer",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00], // All variable
        "18fcffff -> -1000 (decimal)",
        |bytes| {
            if bytes.len() >= 4 {
                let value = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Some(format!("i32: {}", value))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for i64 type
pub fn create_i64_pattern() -> BorshPattern {
    BorshPattern::new(
        "i64",
        "8-byte little-endian signed integer",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "18fcffffffffffff -> -1000 (decimal)",
        |bytes| {
            if bytes.len() >= 8 {
                let value = i64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                Some(format!("i64: {}", value))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for f32 type
pub fn create_f32_pattern() -> BorshPattern {
    BorshPattern::new(
        "f32",
        "4-byte little-endian IEEE-754 floating point",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00], // All variable
        "0000803f -> 1.0",
        |bytes| {
            if bytes.len() >= 4 {
                let value = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                if !value.is_nan() {
                    Some(format!("f32: {}", value))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for f64 type
pub fn create_f64_pattern() -> BorshPattern {
    BorshPattern::new(
        "f64",
        "8-byte little-endian IEEE-754 floating point",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "000000000000f03f -> 1.0",
        |bytes| {
            if bytes.len() >= 8 {
                let value = f64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                if !value.is_nan() {
                    Some(format!("f64: {}", value))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for boolean true value
pub fn create_bool_true_pattern() -> BorshPattern {
    BorshPattern::new(
        "bool",
        "Boolean true value (1)",
        vec![0x01],
        vec![0xFF], // Exact match
        "01 -> true",
        |bytes| {
            if !bytes.is_empty() && bytes[0] == 1 {
                Some("Boolean: true".to_string())
            } else {
                None
            }
        },
    )
}

/// Create a pattern for boolean false value
pub fn create_bool_false_pattern() -> BorshPattern {
    BorshPattern::new(
        "bool",
        "Boolean false value (0)",
        vec![0x00],
        vec![0xFF], // Exact match
        "00 -> false",
        |bytes| {
            if !bytes.is_empty() && bytes[0] == 0 {
                Some("Boolean: false".to_string())
            } else {
                None
            }
        },
    )
}
