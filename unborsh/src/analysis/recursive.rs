// src/analysis/recursive.rs
use crate::analysis::traits::BorshAnalyzer;
use crate::core::utils::detect_enum_variant;
use crate::core::{AnalysisOptions, AnalysisResult, PatternDictionary, PatternMatch};

/// Analyzer that recursively breaks down Borsh data structures
pub struct RecursiveAnalyzer;

impl RecursiveAnalyzer {
    /// Create a new recursive analyzer
    pub fn new() -> Self {
        Self
    }

    /// Recursively analyze a data structure
    fn analyze_recursively(
        &self,
        data: &[u8],
        dictionary: &PatternDictionary,
        options: &AnalysisOptions,
        depth: usize,
        parent_offset: usize,
    ) -> Vec<PatternMatch> {
        // Avoid exceeding maximum depth
        if depth > options.max_depth || data.is_empty() {
            return Vec::new();
        }

        let mut matches = Vec::new();

        // Check for enum variant
        if let Some(variant) = detect_enum_variant(data) {
            matches.push(PatternMatch {
                pattern_name: "EnumVariant".to_string(),
                offset: parent_offset,
                length: 1,
                data: if options.include_raw_bytes {
                    vec![variant]
                } else {
                    Vec::new()
                },
                interpretation: format!("Enum variant {}", variant),
                confidence: 80,
            });

            // Recursively analyze the variant data
            if data.len() > 1 {
                let variant_data = &data[1..];
                let variant_matches = self.analyze_recursively(
                    variant_data,
                    dictionary,
                    options,
                    depth + 1,
                    parent_offset + 1,
                );
                matches.extend(variant_matches);
            }

            return matches;
        }

        // Check for length-prefixed data (String or Vec)
        if data.len() >= 4 {
            let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

            if len as usize + 4 <= data.len() {
                let content = &data[4..4 + len as usize];
                let remainder = &data[4 + len as usize..];

                // Try to interpret as string
                if let Ok(s) = std::str::from_utf8(content) {
                    matches.push(PatternMatch {
                        pattern_name: "String".to_string(),
                        offset: parent_offset,
                        length: 4 + len as usize,
                        data: if options.include_raw_bytes {
                            data[..4 + len as usize].to_vec()
                        } else {
                            Vec::new()
                        },
                        interpretation: format!("String: \"{}\"", s),
                        confidence: 90,
                    });

                    // Recursively analyze any remaining data
                    if !remainder.is_empty() {
                        let remainder_matches = self.analyze_recursively(
                            remainder,
                            dictionary,
                            options,
                            depth + 1,
                            parent_offset + 4 + len as usize,
                        );
                        matches.extend(remainder_matches);
                    }

                    return matches;
                }

                // Otherwise, treat as generic Vec
                matches.push(PatternMatch {
                    pattern_name: "Vec".to_string(),
                    offset: parent_offset,
                    length: 4 + len as usize,
                    data: if options.include_raw_bytes {
                        data[..4 + len as usize].to_vec()
                    } else {
                        Vec::new()
                    },
                    interpretation: format!("Vec with {} elements", len),
                    confidence: 80,
                });

                // Try to analyze elements if there are not too many
                if len > 0 && len <= 20 {
                    // Try to determine element size by looking at content size
                    let elem_size = if len > 0 {
                        content.len() / len as usize
                    } else {
                        0
                    };

                    if elem_size > 0 {
                        for i in 0..len as usize {
                            let start = i * elem_size;
                            if start + elem_size <= content.len() {
                                let element = &content[start..start + elem_size];
                                let element_matches = self.analyze_recursively(
                                    element,
                                    dictionary,
                                    options,
                                    depth + 2,
                                    parent_offset + 4 + start,
                                );

                                // Add element matches with adjusted confidence
                                for mut match_item in element_matches {
                                    match_item.confidence =
                                        match_item.confidence.saturating_sub(10);
                                    matches.push(match_item);
                                }
                            }
                        }
                    }
                }

                // Recursively analyze any remaining data
                if !remainder.is_empty() {
                    let remainder_matches = self.analyze_recursively(
                        remainder,
                        dictionary,
                        options,
                        depth + 1,
                        parent_offset + 4 + len as usize,
                    );
                    matches.extend(remainder_matches);
                }

                return matches;
            }
        }

        // Try fixed-size interpretations
        if data.len() >= 4 {
            let u32_value = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            matches.push(PatternMatch {
                pattern_name: "u32".to_string(),
                offset: parent_offset,
                length: 4,
                data: if options.include_raw_bytes {
                    data[..4].to_vec()
                } else {
                    Vec::new()
                },
                interpretation: format!("u32: {}", u32_value),
                confidence: 60,
            });

            if data.len() > 4 {
                let remainder_matches = self.analyze_recursively(
                    &data[4..],
                    dictionary,
                    options,
                    depth + 1,
                    parent_offset + 4,
                );
                matches.extend(remainder_matches);
            }
        }

        matches
    }

    /// Generate structure hypothesis based on matches
    fn generate_hypothesis(&self, matches: &[PatternMatch]) -> Option<String> {
        if matches.is_empty() {
            return None;
        }

        // Check if first byte looks like enum variant
        let mut result = String::new();

        if matches
            .first()
            .map_or(false, |m| m.pattern_name == "EnumVariant")
        {
            let variant = matches.first().unwrap().interpretation.clone();
            result.push_str(&format!(
                "enum ProbableEnum {{\n    {}, // {}\n    // Other variants\n}}\n\n",
                matches.first().unwrap().pattern_name,
                variant
            ));

            result.push_str("struct EnumData {\n");
        } else {
            result.push_str("struct RecursiveStructure {\n");
        }

        // Add fields, skipping the enum variant if present
        let start_idx = if matches
            .first()
            .map_or(false, |m| m.pattern_name == "EnumVariant")
        {
            1
        } else {
            0
        };

        for (i, m) in matches.iter().enumerate().skip(start_idx) {
            result.push_str(&format!(
                "    field_{}: {}, // {}\n",
                i, m.pattern_name, m.interpretation
            ));
        }

        result.push_str("}");
        Some(result)
    }
}

impl BorshAnalyzer for RecursiveAnalyzer {
    fn name(&self) -> &'static str {
        "RecursiveAnalyzer"
    }

    fn description(&self) -> &'static str {
        "Analyzes Borsh data by recursively breaking down structures"
    }

    fn analyze(
        &self,
        data: &[u8],
        dictionary: &PatternDictionary,
        options: &AnalysisOptions,
    ) -> AnalysisResult {
        let matches = self.analyze_recursively(data, dictionary, options, 0, 0);

        // Filter matches by confidence threshold
        let filtered_matches: Vec<_> = matches
            .into_iter()
            .filter(|m| m.confidence >= options.min_confidence)
            .collect();

        // Limit number of matches if specified
        let limited_matches = if let Some(max_matches) = options.max_matches {
            filtered_matches.into_iter().take(max_matches).collect()
        } else {
            filtered_matches
        };

        // Calculate overall confidence
        let confidence = AnalysisResult::calculate_confidence(&limited_matches);

        // Generate structure hypothesis
        let structure_hypothesis = self.generate_hypothesis(&limited_matches);

        let limited_matches_len = limited_matches.len();
        AnalysisResult {
            matches: limited_matches,
            structure_hypothesis,
            confidence,
            description: format!(
                "Recursive analysis found {} structures",
                limited_matches_len
            ),
        }
    }
}

impl Default for RecursiveAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
