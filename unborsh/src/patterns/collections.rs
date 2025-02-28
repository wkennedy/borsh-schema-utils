// src/patterns/collections.rs

//! Pattern definitions for collection types in Borsh

use crate::core::BorshPattern;

/// Create a pattern for String type
pub fn create_string_pattern() -> BorshPattern {
    BorshPattern::new(
        "String",
        "Length-prefixed UTF-8 string: [length: u32 LE][utf8 bytes]",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "0b000000536f6d6520636f6e74656e74 -> \"Some content\"",
        |bytes| {
            if bytes.len() >= 4 {
                let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                if len as usize + 4 <= bytes.len() {
                    let str_bytes = &bytes[4..4 + len as usize];
                    if let Ok(s) = std::str::from_utf8(str_bytes) {
                        let display_str = if s.len() > 30 {
                            format!("\"{}...\"", &s[0..30])
                        } else {
                            format!("\"{}\"", s)
                        };
                        return Some(format!("String: {}", display_str));
                    }
                }
            }
            None
        }
    )
}

/// Create a pattern for Vec type
pub fn create_vec_pattern() -> BorshPattern {
    BorshPattern::new(
        "Vec",
        "Length-prefixed vector: [length: u32 LE][elements...]",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "03000000010203 -> Vec with 3 elements: [1, 2, 3]",
        |bytes| {
            if bytes.len() >= 4 {
                let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                if len as usize + 4 <= bytes.len() {
                    let element_bytes = &bytes[4..4 + len as usize];

                    // Try to guess element type based on length
                    let element_type = match element_bytes.len() {
                        0 => "empty Vec".to_string(),
                        n if n == len as usize => "Vec<u8>".to_string(),
                        n if n == len as usize * 2 => "possible Vec<u16>".to_string(),
                        n if n == len as usize * 4 => "possible Vec<u32>".to_string(),
                        n if n == len as usize * 8 => "possible Vec<u64>".to_string(),
                        _ => "Vec<T>".to_string(),
                    };

                    return Some(format!("{} with {} elements", element_type, len));
                }
            }
            None
        }
    )
}

/// Create a pattern for Option::None
pub fn create_option_none_pattern() -> BorshPattern {
    BorshPattern::new(
        "Option::None",
        "Option enum discriminant for None",
        vec![0x00],
        vec![0xFF], // Exact match
        "00 -> None",
        |bytes| {
            if !bytes.is_empty() && bytes[0] == 0 {
                Some("Option::None".to_string())
            } else {
                None
            }
        }
    )
}

/// Create a pattern for Option::Some
pub fn create_option_some_pattern() -> BorshPattern {
    BorshPattern::new(
        "Option::Some",
        "Option enum discriminant for Some followed by inner value",
        vec![0x01],
        vec![0xFF], // Exact match on discriminant
        "01[...] -> Some(...)",
        |bytes| {
            if !bytes.is_empty() && bytes[0] == 1 {
                Some("Option::Some followed by value".to_string())
            } else {
                None
            }
        }
    )
}

/// Create a pattern for Result::Ok
pub fn create_result_ok_pattern() -> BorshPattern {
    BorshPattern::new(
        "Result::Ok",
        "Result enum discriminant for Ok followed by inner value",
        vec![0x01],
        vec![0xFF], // Exact match on discriminant
        "01[...] -> Ok(...)",
        |bytes| {
            if !bytes.is_empty() && bytes[0] == 1 {
                Some("Result::Ok followed by value".to_string())
            } else {
                None
            }
        }
    )
}

/// Create a pattern for Result::Err
pub fn create_result_err_pattern() -> BorshPattern {
    BorshPattern::new(
        "Result::Err",
        "Result enum discriminant for Err followed by error value",
        vec![0x00],
        vec![0xFF], // Exact match on discriminant
        "00[...] -> Err(...)",
        |bytes| {
            if !bytes.is_empty() && bytes[0] == 0 {
                Some("Result::Err followed by error value".to_string())
            } else {
                None
            }
        }
    )
}

/// Create a pattern for HashMap
pub fn create_hashmap_pattern() -> BorshPattern {
    BorshPattern::new(
        "HashMap",
        "Length-prefixed HashMap: [length: u32 LE][key-value pairs...]",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00], // Variable
        "02000000[...] -> HashMap with 2 entries",
        |bytes| {
            if bytes.len() >= 4 {
                let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Some(format!("HashMap with {} entries", len))
            } else {
                None
            }
        }
    )
}

/// Create a pattern for HashSet
pub fn create_hashset_pattern() -> BorshPattern {
    BorshPattern::new(
        "HashSet",
        "Length-prefixed HashSet: [length: u32 LE][elements...]",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00], // Variable
        "03000000[...] -> HashSet with 3 elements",
        |bytes| {
            if bytes.len() >= 4 {
                let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Some(format!("HashSet with {} elements", len))
            } else {
                None
            }
        }
    )
}

/// Create a pattern for BTreeMap
pub fn create_btreemap_pattern() -> BorshPattern {
    BorshPattern::new(
        "BTreeMap",
        "Length-prefixed BTreeMap: [length: u32 LE][key-value pairs...]",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00], // Variable
        "02000000[...] -> BTreeMap with 2 entries",
        |bytes| {
            if bytes.len() >= 4 {
                let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Some(format!("BTreeMap with {} entries", len))
            } else {
                None
            }
        }
    )
}

/// Create a pattern for BTreeSet
pub fn create_btreeset_pattern() -> BorshPattern {
    BorshPattern::new(
        "BTreeSet",
        "Length-prefixed BTreeSet: [length: u32 LE][elements...]",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00], // Variable
        "03000000[...] -> BTreeSet with 3 elements",
        |bytes| {
            if bytes.len() >= 4 {
                let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Some(format!("BTreeSet with {} elements", len))
            } else {
                None
            }
        }
    )
}

/// Create a pattern for arrays
pub fn create_array_pattern() -> BorshPattern {
    BorshPattern::new(
        "Array",
        "Fixed-length array of elements",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "[01, 02, 03, 04] -> [u8; 4]",
        |bytes| {
            // This is more of a heuristic since fixed arrays don't have a length prefix
            // Try to detect common array sizes
            let common_sizes = [1, 2, 4, 8, 16, 32, 64];
            for &size in &common_sizes {
                if bytes.len() == size {
                    return Some(format!("Possible [T; {}]", size));
                }
            }
            None
        }
    )
}

/// Create a pattern for tuples
pub fn create_tuple_pattern() -> BorshPattern {
    BorshPattern::new(
        "Tuple",
        "Fixed collection of heterogeneous elements",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "(u8, u32, String) -> [01, 0A000000, 03000000646F67]",
        |_bytes| {
            // Tuples are difficult to detect without context
            // This is more of a placeholder
            Some("Possible tuple of elements".to_string())
        }
    )
}