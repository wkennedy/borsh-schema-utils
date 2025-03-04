// src/analysis/probabilistic.rs
use crate::analysis::traits::BorshAnalyzer;
use crate::core::{AnalysisOptions, AnalysisResult, PatternDictionary, PatternMatch};

/// Analyzer that makes probabilistic guesses about Borsh data structure
pub struct ProbabilisticAnalyzer;

impl ProbabilisticAnalyzer {
    /// Create a new probabilistic analyzer
    pub fn new() -> Self {
        Self
    }

    /// Identify the most likely field type at a given offset
    fn identify_most_likely_field(&self, data: &[u8]) -> (String, usize, u8, String) {
        // Collection of (field_type, field_length, confidence, description)
        let mut candidates = Vec::new();

        // Try u8 field (1 byte)
        if !data.is_empty() {
            let value = data[0];
            let mut confidence = 50; // Base confidence

            // Adjust confidence based on value
            if value <= 1 {
                confidence += 30; // Likely boolean or 0/1 enum variant
            } else if value <= 5 {
                confidence += 20; // Likely small enum variant
            }

            candidates.push(("u8".to_string(), 1, confidence, format!("value: {}", value)));
        }

        // Try u32 field (4 bytes)
        if data.len() >= 4 {
            let value = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            let mut confidence = 60; // Base confidence for u32

            // Adjust confidence based on value patterns
            if value == 0 {
                confidence -= 10; // Zero could be padding or null
            } else if value < 1000 {
                confidence += 10; // Small values are common for counters
            } else if value > 1_000_000_000 {
                confidence -= 10; // Very large values less common unless timestamps
            }

            candidates.push((
                "u32".to_string(),
                4,
                confidence,
                format!("value: {}", value),
            ));

            // Check if this could be a length prefix for a string or vector
            if value as usize + 4 <= data.len() && value < 10000 {
                let content = &data[4..4 + value as usize];

                // Check if this could be a string
                if let Ok(s) = std::str::from_utf8(content) {
                    // ASCII strings have higher confidence
                    let ascii_ratio = content.iter().filter(|&&b| (32..=126).contains(&b)).count()
                        as f32
                        / content.len() as f32;
                    let string_confidence = 70 + (ascii_ratio * 20.0) as u8;

                    let display_str = format!("\"{}\"", s);

                    candidates.push((
                        "String".to_string(),
                        4 + value as usize,
                        string_confidence,
                        display_str,
                    ));
                } else {
                    // Could be a Vec<u8> or other binary data
                    candidates.push((
                        "Vec<u8>".to_string(),
                        4 + value as usize,
                        65,
                        format!("{} bytes", value),
                    ));
                }
            }
        }

        // Try u64 field (8 bytes)
        if data.len() >= 8 {
            let value = u64::from_le_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ]);

            let mut confidence = 55; // Base confidence for u64

            // Adjust confidence based on value
            if value == 0 {
                confidence -= 5; // Zero could be padding, but sometimes legitimate
            }

            candidates.push((
                "u64".to_string(),
                8,
                confidence,
                format!("value: {}", value),
            ));
        }

        // Try boolean (1 byte but only values 0 or 1)
        if !data.is_empty() && (data[0] == 0 || data[0] == 1) {
            let value = data[0] == 1;
            candidates.push(("bool".to_string(), 1, 80, format!("value: {}", value)));
        }

        // Sort candidates by confidence (descending)
        candidates.sort_by(|a, b| b.2.cmp(&a.2));

        // Return the most likely candidate, or a default if none found
        candidates
            .first()
            .map(|(t, l, c, d)| (t.clone(), *l, *c, d.clone()))
            .unwrap_or((
                "unknown".to_string(),
                1,
                0,
                "Unknown field type".to_string(),
            ))
    }

    /// Identify known patterns in the data
    fn identify_known_patterns(&self, data: &[u8]) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        // Check for enum variant pattern
        if !data.is_empty() && data[0] <= 5 {
            matches.push(PatternMatch {
                pattern_name: "EnumVariant".to_string(),
                offset: 0,
                length: 1,
                data: vec![data[0]],
                interpretation: format!("Enum variant {}", data[0]),
                confidence: 80,
            });
        }

        // Check for Option<T> pattern
        if !data.is_empty() {
            match data[0] {
                0 => matches.push(PatternMatch {
                    pattern_name: "Option::None".to_string(),
                    offset: 0,
                    length: 1,
                    data: vec![0],
                    interpretation: "None".to_string(),
                    confidence: 85,
                }),
                1 if data.len() > 1 => {
                    matches.push(PatternMatch {
                        pattern_name: "Option::Some".to_string(),
                        offset: 0,
                        length: 1,
                        data: vec![1],
                        interpretation: "Some(...)".to_string(),
                        confidence: 85,
                    });

                    // Recursively analyze the contained value
                    if data.len() > 1 {
                        let (inner_type, inner_len, confidence, desc) =
                            self.identify_most_likely_field(&data[1..]);
                        matches.push(PatternMatch {
                            pattern_name: inner_type,
                            offset: 1,
                            length: inner_len,
                            data: data[1..].iter().take(inner_len).cloned().collect(),
                            interpretation: desc,
                            confidence,
                        });
                    }
                }
                _ => {}
            }
        }

        // Check for length-prefixed collections (String, Vec)
        if data.len() >= 4 {
            let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            if len as usize + 4 <= data.len() {
                let content = &data[4..4 + len as usize];

                // Try string interpretation
                if let Ok(s) = std::str::from_utf8(content) {
                    let display_str = if s.len() > 30 {
                        format!("\"{}\"", &s[0..30])
                    } else {
                        format!("\"{}\"", s)
                    };

                    matches.push(PatternMatch {
                        pattern_name: "String".to_string(),
                        offset: 0,
                        length: 4 + len as usize,
                        data: data[..4 + len as usize].to_vec(),
                        interpretation: display_str,
                        confidence: 90,
                    });
                } else {
                    // Vec<u8>
                    matches.push(PatternMatch {
                        pattern_name: "Vec<u8>".to_string(),
                        offset: 0,
                        length: 4 + len as usize,
                        data: data[..4 + len as usize].to_vec(),
                        interpretation: format!("{} bytes", len),
                        confidence: 75,
                    });
                }
            }
        }

        matches
    }

    /// Generate structure hypothesis from probabilistic analysis
    fn generate_hypothesis(
        &self,
        field_candidates: &[(usize, String, usize, u8, String)],
    ) -> Option<String> {
        if field_candidates.is_empty() {
            return None;
        }

        let mut result = String::from("struct ProbableStructure {\n");

        for (i, (_, field_type, _, confidence, _)) in field_candidates.iter().enumerate() {
            // Skip very low confidence fields
            if *confidence < 30 {
                result.push_str(&format!(
                    "    field_{}: {}, // Low confidence ({}%)\n",
                    i, field_type, confidence
                ));
            } else {
                result.push_str(&format!("    field_{}: {},\n", i, field_type));
            }
        }

        result.push('}');

        // Check if first byte looks like enum variant
        if !field_candidates.is_empty() {
            let (offset, field_type, _, _, _) = field_candidates[0].clone();
            if offset == 0 && field_type == "EnumVariant" {
                result.push_str("\n\n// Alternative: This could be an enum variant");
            }
        }

        Some(result)
    }
}

impl BorshAnalyzer for ProbabilisticAnalyzer {
    fn name(&self) -> &'static str {
        "ProbabilisticAnalyzer"
    }

    fn description(&self) -> &'static str {
        "Analyzes Borsh data using probabilistic pattern detection"
    }

    fn analyze(
        &self,
        data: &[u8],
        _dictionary: &PatternDictionary,
        options: &AnalysisOptions,
    ) -> AnalysisResult {
        let mut field_candidates = Vec::new();
        let mut current_offset = 0;

        // Try to identify known patterns first
        let known_patterns = self.identify_known_patterns(data);
        let mut matches: Vec<PatternMatch> = Vec::new();

        // If we found known patterns that cover a significant portion of the data,
        // use those instead of field-by-field analysis
        let known_coverage: usize = known_patterns.iter().map(|p| p.length).sum();
        if !known_patterns.is_empty() && known_coverage > data.len() / 2 {
            matches = known_patterns;
        } else {
            // Continue analyzing field by field until we've covered the entire data
            while current_offset < data.len() {
                let remaining = &data[current_offset..];

                // Skip if we have too little data left
                if remaining.is_empty() {
                    break;
                }

                // Identify the most likely field type at the current offset
                let (field_type, field_length, confidence, description) =
                    self.identify_most_likely_field(remaining);

                // Record this candidate
                field_candidates.push((
                    current_offset,
                    field_type.clone(),
                    field_length,
                    confidence,
                    description.clone(),
                ));

                // Add as a match
                if confidence >= options.min_confidence {
                    matches.push(PatternMatch {
                        pattern_name: field_type,
                        offset: current_offset,
                        length: field_length,
                        data: if options.include_raw_bytes {
                            remaining[..field_length.min(remaining.len())].to_vec()
                        } else {
                            Vec::new()
                        },
                        interpretation: description,
                        confidence,
                    });
                }

                // Move to next field
                current_offset += field_length;
            }
        }

        // Limit number of matches if specified
        let limited_matches = if let Some(max_matches) = options.max_matches {
            matches.into_iter().take(max_matches).collect()
        } else {
            matches
        };

        // Calculate overall confidence
        let confidence = AnalysisResult::calculate_confidence(&limited_matches);

        // Generate structure hypothesis
        let structure_hypothesis = self.generate_hypothesis(&field_candidates);

        AnalysisResult {
            matches: limited_matches,
            structure_hypothesis,
            confidence,
            description: format!(
                "Probabilistic analysis found {} likely fields",
                field_candidates.len()
            ),
        }
    }
}

impl Default for ProbabilisticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
