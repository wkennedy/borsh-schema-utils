// src/core/utils.rs

/// Utility functions for Borsh data analysis

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
