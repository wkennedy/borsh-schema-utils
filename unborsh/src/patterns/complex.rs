// src/patterns/complex.rs

//! Pattern definitions for complex Borsh types

use crate::core::BorshPattern;

/// Create a pattern for enum types
pub fn create_enum_pattern() -> BorshPattern {
    BorshPattern::new(
        "Enum",
        "Enum variant discriminant byte followed by variant data",
        vec![0x00],
        vec![0x00], // Variable discriminant
        "00[...] -> Variant 0",
        |bytes| {
            if !bytes.is_empty() {
                let variant = bytes[0];
                if variant <= 5 {
                    Some(format!("Enum variant {}", variant))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for struct types
pub fn create_struct_pattern() -> BorshPattern {
    BorshPattern::new(
        "Struct",
        "Sequence of serialized fields",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "{field1: u32, field2: String} -> [01000000, 05000000, 68656C6C6F]",
        |_bytes| {
            // Structs are difficult to detect generically
            Some("Possible struct data".to_string())
        },
    )
}

/// Create a pattern for nested structs
pub fn create_nested_struct_pattern() -> BorshPattern {
    BorshPattern::new(
        "NestedStruct",
        "Struct containing other complex types",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "Complex nested structure",
        |_bytes| {
            // Generic placeholder
            Some("Possible nested structure".to_string())
        },
    )
}

/// Create a pattern for fields with optional values
pub fn create_optional_field_pattern() -> BorshPattern {
    BorshPattern::new(
        "OptionalField",
        "Field that may be absent (Option<T>)",
        vec![0x00],
        vec![0xFF], // First byte is Option discriminant
        "00 -> None, 01[...] -> Some(...)",
        |bytes| {
            if !bytes.is_empty() {
                match bytes[0] {
                    0 => Some("Optional field: None".to_string()),
                    1 => Some("Optional field: Some(...)".to_string()),
                    _ => None,
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for fields with variable types
pub fn create_variant_field_pattern() -> BorshPattern {
    BorshPattern::new(
        "VariantField",
        "Field that can have multiple types (enum or tagged union)",
        vec![0x00, 0x00],
        vec![0xFF, 0x00], // First byte is variant discriminant
        "00[...] -> TypeA, 01[...] -> TypeB",
        |bytes| {
            if !bytes.is_empty() {
                let variant = bytes[0];
                if variant <= 5 {
                    Some(format!("Variant field: type {}", variant))
                } else {
                    None
                }
            } else {
                None
            }
        },
    )
}

/// Create a pattern for a map entry (key-value pair)
pub fn create_map_entry_pattern() -> BorshPattern {
    BorshPattern::new(
        "MapEntry",
        "Key-value pair for maps",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "key-value pair",
        |_bytes| {
            // Generic placeholder
            Some("Possible map entry".to_string())
        },
    )
}

/// Create a pattern for custom data types with a tag
pub fn create_tagged_data_pattern() -> BorshPattern {
    BorshPattern::new(
        "TaggedData",
        "Data prefixed with a type tag or identifier",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0xFF, 0xFF, 0xFF, 0xFF], // Exact match on tag
        "Tagged data block",
        |bytes| {
            if bytes.len() >= 4 {
                let tag = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Some(format!("Tagged data with tag 0x{:08x}", tag))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for recursive data structures
pub fn create_recursive_pattern() -> BorshPattern {
    BorshPattern::new(
        "RecursiveData",
        "Data structure that contains references to its own type",
        vec![0x00, 0x00, 0x00, 0x00],
        vec![0x00, 0x00, 0x00, 0x00], // Variable
        "Linked list, tree node, etc.",
        |bytes| {
            if bytes.len() >= 4 {
                // Generic placeholder
                Some("Possible recursive data structure".to_string())
            } else {
                None
            }
        },
    )
}

/// Create a pattern for an object ID or reference
pub fn create_object_id_pattern() -> BorshPattern {
    BorshPattern::new(
        "ObjectID",
        "Unique identifier for an object",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "Object identifier",
        |bytes| {
            if bytes.len() >= 8 {
                let id = u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                Some(format!("Possible Object ID: {}", id))
            } else if bytes.len() >= 4 {
                let id = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Some(format!("Possible Object ID: {}", id))
            } else {
                None
            }
        },
    )
}

/// Create a pattern for custom entity types
pub fn create_entity_pattern() -> BorshPattern {
    BorshPattern::new(
        "Entity",
        "Entity type with ID and properties",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "Entity with ID and properties",
        |bytes| {
            if bytes.len() >= 12 {
                // Try to detect an ID field followed by a length-prefixed field
                if bytes.len() >= 8 {
                    let potential_id = u64::from_le_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
                        bytes[7],
                    ]);

                    if bytes.len() >= 12 {
                        let potential_length =
                            u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

                        if potential_length < 1000 && bytes.len() >= 12 + potential_length as usize
                        {
                            return Some(format!(
                                "Possible Entity (ID: {}, with {} properties)",
                                potential_id, potential_length
                            ));
                        }
                    }
                }
            }
            None
        },
    )
}

/// Create a pattern for complex nested collections
pub fn create_nested_collection_pattern() -> BorshPattern {
    BorshPattern::new(
        "NestedCollection",
        "Collection containing other collections",
        vec![0x00, 0x00, 0x00, 0x00], // Length prefix
        vec![0x00, 0x00, 0x00, 0x00], // Variable
        "Vec<Vec<T>> or similar",
        |bytes| {
            if bytes.len() >= 8 {
                let outer_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

                // If the outer length is reasonable
                if outer_len > 0 && outer_len < 100 && bytes.len() >= 8 {
                    let inner_len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

                    // If there appears to be an inner length prefix
                    if inner_len < 1000 {
                        return Some(format!(
                            "Possible nested collection with {} outer elements",
                            outer_len
                        ));
                    }
                }
            }
            None
        },
    )
}

/// Create a pattern for custom message or packet formats
pub fn create_message_pattern() -> BorshPattern {
    BorshPattern::new(
        "Message",
        "Message structure with header and payload",
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Placeholder
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // All variable
        "Message with type and payload",
        |bytes| {
            if bytes.len() >= 8 {
                let msg_type = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                let payload_len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

                if payload_len as usize + 8 <= bytes.len() && payload_len < 100000 {
                    return Some(format!(
                        "Possible message (type: {}, payload: {} bytes)",
                        msg_type, payload_len
                    ));
                }
            }
            None
        },
    )
}

/// Create a pattern for custom protocol versioning information
pub fn create_versioned_data_pattern() -> BorshPattern {
    BorshPattern::new(
        "VersionedData",
        "Data with version information",
        vec![0x00, 0x00, 0x00, 0x00], // Version
        vec![0xFF, 0xFF, 0x00, 0x00], // First bytes are version
        "Versioned data",
        |bytes| {
            if bytes.len() >= 2 {
                let major = bytes[0];
                let minor = bytes[1];
                return Some(format!("Possible versioned data (v{}.{})", major, minor));
            }
            None
        },
    )
}
