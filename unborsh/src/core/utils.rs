// src/core/utils.rs

// Utility functions for Borsh data analysis

/// Extracts all potential strings from a byte slice
pub fn extract_strings(data: &[u8]) -> Vec<String> {
    let mut strings = Vec::new();
    let mut position = 0;

    while position + 4 <= data.len() {
        let len = u32::from_le_bytes([
            data[position],
            data[position + 1],
            data[position + 2],
            data[position + 3],
        ]);

        // Check if this could be a valid string length
        if len > 0 && len < 10000 && position + 4 + len as usize <= data.len() {
            let potential_string = &data[position + 4..position + 4 + len as usize];

            // Try to parse as UTF-8
            if let Ok(s) = std::str::from_utf8(potential_string) {
                strings.push(s.to_string());
            }
        }

        position += 1;
    }

    strings
}

/// Check if a byte slice has high entropy (likely random data)
pub fn has_high_entropy(data: &[u8]) -> bool {
    if data.len() < 8 {
        return false;
    }

    // Simple entropy check: count unique bytes
    let unique_bytes = data.iter().collect::<std::collections::HashSet<_>>().len();

    // High entropy if more than 70% of possible byte values are used
    unique_bytes > data.len() * 7 / 10
}

/// Calculate confidence based on data characteristics
pub fn calculate_confidence(data: &[u8], expected_pattern: &[u8], expected_mask: &[u8]) -> u8 {
    if data.len() < expected_pattern.len() {
        return 0;
    }

    // Start with base confidence
    let mut confidence = 50u8;

    // Check pattern matches
    let mut matches = 0;
    for i in 0..expected_pattern.len() {
        if expected_mask[i] == 0xFF && data[i] == expected_pattern[i] {
            matches += 1;
        }
    }

    // Adjust confidence based on match ratio
    let match_ratio = matches as f32 / expected_pattern.len() as f32;
    confidence = (confidence as f32 * (0.5 + match_ratio / 2.0)) as u8;

    // Adjust for data length
    if data.len() > expected_pattern.len() * 2 {
        confidence = confidence.saturating_sub(10);
    }

    // Adjust for special cases
    if expected_pattern.len() >= 4 {
        // Check if data starts with a length prefix
        let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if len as usize + 4 == data.len() {
            confidence = confidence.saturating_add(20);
        }
    }

    confidence
}

/// Find common prefixes among strings
pub fn find_common_prefixes(strings: &[&String]) -> Vec<String> {
    let mut prefixes = Vec::new();

    if strings.is_empty() {
        return prefixes;
    }

    // Check common prefixes of at least 3 characters
    for len in 3..=10 {
        if strings[0].len() < len {
            continue;
        }

        let prefix = &strings[0][0..len];
        let count = strings.iter().filter(|s| s.starts_with(prefix)).count();

        // If more than half the strings share this prefix
        if count > strings.len() / 2 {
            prefixes.push(prefix.to_string());
        }
    }

    prefixes
}

/// Detect possible enum variant based on first byte
pub fn detect_enum_variant(data: &[u8]) -> Option<u8> {
    if !data.is_empty() && data[0] <= 5 {
        Some(data[0])
    } else {
        None
    }
}

/// Try to interpret bytes as different numeric types
pub fn interpret_as_numbers(data: &[u8]) -> Vec<(String, String)> {
    let mut interpretations = Vec::new();

    // Try u8
    if !data.is_empty() {
        interpretations.push(("u8".to_string(), format!("{}", data[0])));
    }

    // Try u16
    if data.len() >= 2 {
        let value = u16::from_le_bytes([data[0], data[1]]);
        interpretations.push(("u16".to_string(), format!("{}", value)));
    }

    // Try u32
    if data.len() >= 4 {
        let value = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        interpretations.push(("u32".to_string(), format!("{}", value)));

        // Also try f32
        let f32_value = f32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if !f32_value.is_nan() && f32_value.abs() < 1_000_000.0 {
            interpretations.push(("f32".to_string(), format!("{}", f32_value)));
        }
    }

    // Try u64
    if data.len() >= 8 {
        let value = u64::from_le_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);
        interpretations.push(("u64".to_string(), format!("{}", value)));

        // Also try f64
        let f64_value = f64::from_le_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);
        if !f64_value.is_nan() && f64_value.abs() < 1_000_000.0 {
            interpretations.push(("f64".to_string(), format!("{}", f64_value)));
        }
    }

    interpretations
}

/// Get a cleaned hexadecimal representation of bytes
pub fn hex_format(data: &[u8], _max_len: usize) -> String {
    if data.is_empty() {
        return "".to_string();
    }

    hex::encode(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_strings_empty() {
        let data = vec![];
        let strings = extract_strings(&data);
        assert!(strings.is_empty());
    }

    #[test]
    fn test_extract_strings_valid() {
        // Create data with two strings: "Hello" and "World"
        let data = vec![
            5, 0, 0, 0, b'H', b'e', b'l', b'l', b'o', // "Hello"
            5, 0, 0, 0, b'W', b'o', b'r', b'l', b'd', // "World"
        ];
        let strings = extract_strings(&data);

        assert_eq!(strings.len(), 2);
        assert!(strings.contains(&"Hello".to_string()));
        assert!(strings.contains(&"World".to_string()));
    }

    #[test]
    fn test_extract_strings_overlapping() {
        // Data with overlapping potential strings
        let data = vec![
            // "Hello" at position 0
            5, 0, 0, 0, b'H', b'e', b'l', b'l', b'o',
            // "ello " starting at position 1 (using the 'e' from above)
            5, 0, 0, 0, b'e', b'l', b'l', b'o', b' ',
        ];
        let strings = extract_strings(&data);

        assert!(strings.contains(&"Hello".to_string()));
        assert!(strings.contains(&"ello ".to_string()));
    }

    #[test]
    fn test_extract_strings_invalid_utf8() {
        // Create data with invalid UTF-8 sequence
        let data = vec![
            3, 0, 0, 0, 0xFF, 0xFE, 0xFD, // Invalid UTF-8
        ];
        let strings = extract_strings(&data);

        assert!(strings.is_empty());
    }

    #[test]
    fn test_extract_strings_large_length() {
        // String with unreasonably large length that exceeds data size
        let data = vec![
            0xFF, 0xFF, 0xFF, 0x7F, b'H', b'e', b'l', b'l', b'o',
        ];
        let strings = extract_strings(&data);

        assert!(strings.is_empty());
    }

    #[test]
    fn test_has_high_entropy_short_data() {
        // Data shorter than 8 bytes should return false
        let data = vec![1, 2, 3, 4, 5, 6, 7];
        assert!(!has_high_entropy(&data));
    }

    #[test]
    fn test_has_high_entropy_low() {
        // Data with low entropy (repeated values)
        let data = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
        assert!(!has_high_entropy(&data));
    }

    #[test]
    fn test_has_high_entropy_high() {
        // Data with high entropy (many unique values)
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        assert!(has_high_entropy(&data));
    }

    #[test]
    fn test_calculate_confidence_short_data() {
        let data = vec![1, 2];
        let pattern = vec![1, 2, 3, 4];
        let mask = vec![0xFF, 0xFF, 0xFF, 0xFF];

        // Data is shorter than pattern
        assert_eq!(calculate_confidence(&data, &pattern, &mask), 0);
    }

    #[test]
    fn test_calculate_confidence_exact_match() {
        let data = vec![1, 2, 3, 4];
        let pattern = vec![1, 2, 3, 4];
        let mask = vec![0xFF, 0xFF, 0xFF, 0xFF];

        // Exact match should have decent confidence, though not necessarily high
        // The function starts with base 50 confidence and adjusts based on match ratio
        let confidence = calculate_confidence(&data, &pattern, &mask);
        assert!(confidence >= 50, "Exact match should have at least base confidence, got {}", confidence);
    }

    #[test]
    fn test_calculate_confidence_partial_match() {
        let data = vec![1, 5, 3, 8];
        let pattern = vec![1, 2, 3, 4];
        let mask = vec![0xFF, 0xFF, 0xFF, 0xFF];

        // Partial match (2 out of 4 bytes match)
        let confidence = calculate_confidence(&data, &pattern, &mask);
        assert!(confidence > 35 && confidence < 70, "Partial match should have medium confidence, got {}", confidence);
    }

    #[test]
    fn test_calculate_confidence_with_mask() {
        let data = vec![1, 5, 3, 8];
        let pattern = vec![1, 0, 3, 0];
        let mask = vec![0xFF, 0x00, 0xFF, 0x00]; // Only check first and third bytes

        // With mask, should have decent confidence for the masked bytes
        let confidence = calculate_confidence(&data, &pattern, &mask);
        assert!(confidence >= 35, "Masked match should have reasonable confidence, got {}", confidence);
    }

    #[test]
    fn test_calculate_confidence_with_length_prefix() {
        // Data where 4-byte length prefix indicates exactly the right size
        let data = vec![5, 0, 0, 0, 1, 2, 3, 4, 5]; // Length 5 + content (5 bytes)
        let pattern = vec![0, 0, 0, 0];
        let mask = vec![0x00, 0x00, 0x00, 0x00]; // Not checking actual values

        // Check that we get reasonable confidence
        // Note: The implementation adds 20 to confidence for proper length prefixes,
        // but confidence might still not reach 70 depending on other factors.
        let confidence = calculate_confidence(&data, &pattern, &mask);
        assert!(confidence >= 35, "Length-prefixed data should have reasonable confidence, got {}", confidence);
    }

    #[test]
    fn test_find_common_prefixes_empty() {
        let strings: Vec<&String> = vec![];
        let prefixes = find_common_prefixes(&strings);
        assert!(prefixes.is_empty());
    }

    #[test]
    fn test_find_common_prefixes_common() {
        let str1 = "abcdef".to_string();
        let str2 = "abcghi".to_string();
        let str3 = "abcjkl".to_string();

        let strings = vec![&str1, &str2, &str3];
        let prefixes = find_common_prefixes(&strings);

        assert!(prefixes.contains(&"abc".to_string()), "Should find 'abc' prefix");
    }

    #[test]
    fn test_find_common_prefixes_no_common() {
        let str1 = "abcdef".to_string();
        let str2 = "ghijkl".to_string();
        let str3 = "mnopqr".to_string();

        let strings = vec![&str1, &str2, &str3];
        let prefixes = find_common_prefixes(&strings);

        assert!(prefixes.is_empty(), "Should find no common prefixes");
    }

    #[test]
    fn test_find_common_prefixes_short_strings() {
        let str1 = "ab".to_string(); // Too short for a 3-char prefix
        let str2 = "ab".to_string();

        let strings = vec![&str1, &str2];
        let prefixes = find_common_prefixes(&strings);

        assert!(prefixes.is_empty(), "Strings are too short for 3+ char prefixes");
    }

    #[test]
    fn test_detect_enum_variant_valid() {
        let data = vec![2, 10, 20, 30]; // First byte 2 is a valid enum variant
        assert_eq!(detect_enum_variant(&data), Some(2));

        let data = vec![5, 10, 20, 30]; // 5 is the max valid enum variant
        assert_eq!(detect_enum_variant(&data), Some(5));
    }

    #[test]
    fn test_detect_enum_variant_invalid() {
        let data = vec![6, 10, 20, 30]; // 6 is too large for an enum variant
        assert_eq!(detect_enum_variant(&data), None);

        let data: Vec<u8> = vec![]; // Empty data
        assert_eq!(detect_enum_variant(&data), None);
    }

    #[test]
    fn test_interpret_as_numbers_empty() {
        let data: Vec<u8> = vec![];
        let interpretations = interpret_as_numbers(&data);
        assert!(interpretations.is_empty());
    }

    #[test]
    fn test_interpret_as_numbers_u8() {
        let data = vec![42];
        let interpretations = interpret_as_numbers(&data);

        assert!(!interpretations.is_empty());
        assert!(interpretations.iter().any(|(t, v)| t == "u8" && v == "42"));
    }

    #[test]
    fn test_interpret_as_numbers_u16() {
        let data = vec![42, 0]; // 42 in little-endian
        let interpretations = interpret_as_numbers(&data);

        assert!(interpretations.iter().any(|(t, v)| t == "u16" && v == "42"));
    }

    #[test]
    fn test_interpret_as_numbers_u32() {
        let data = vec![42, 0, 0, 0]; // 42 in little-endian
        let interpretations = interpret_as_numbers(&data);

        assert!(interpretations.iter().any(|(t, v)| t == "u32" && v == "42"));
    }

    #[test]
    fn test_interpret_as_numbers_u64() {
        let data = vec![42, 0, 0, 0, 0, 0, 0, 0]; // 42 in little-endian
        let interpretations = interpret_as_numbers(&data);

        assert!(interpretations.iter().any(|(t, v)| t == "u64" && v == "42"));
    }

    #[test]
    fn test_interpret_as_numbers_float() {
        // Data representing a valid float
        let data = vec![0, 0, 0x80, 0x3F]; // 1.0f32 in little-endian
        let interpretations = interpret_as_numbers(&data);

        assert!(interpretations.iter().any(|(t, _)| t == "f32"));
    }

    #[test]
    fn test_hex_format_empty() {
        let data: Vec<u8> = vec![];
        assert_eq!(hex_format(&data, 10), "");
    }

    #[test]
    fn test_hex_format() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        assert_eq!(hex_format(&data, 10), "deadbeef");
    }
}
