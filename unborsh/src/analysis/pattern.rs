// src/analysis/pattern.rs
use crate::analysis::traits::BorshAnalyzer;
use crate::core::{AnalysisOptions, AnalysisResult, PatternDictionary, PatternMatch};

/// Analyzer that uses pattern matching against a dictionary
pub struct PatternAnalyzer;

impl PatternAnalyzer {
    /// Create a new pattern analyzer
    pub fn new() -> Self {
        Self
    }

    /// Try to match patterns at the current offset
    fn match_patterns_at_offset(
        &self,
        data: &[u8],
        offset: usize,
        dictionary: &PatternDictionary,
        include_raw_bytes: bool,
    ) -> Option<PatternMatch> {
        if offset >= data.len() {
            return None;
        }

        let remaining = &data[offset..];
        let pattern_matches = dictionary.identify_patterns(remaining);

        if pattern_matches.is_empty() {
            return None;
        }

        // Use the first match (usually the most confident)
        let (pattern_name, interpretation) = pattern_matches[0].clone();

        // Get the pattern to determine its length
        let pattern = dictionary.get_pattern_by_name(&pattern_name)?;
        let length = pattern.len();

        // Extract the matched data
        let matched_data = if include_raw_bytes {
            remaining[..length.min(remaining.len())].to_vec()
        } else {
            Vec::new()
        };

        Some(PatternMatch {
            pattern_name,
            offset,
            length,
            data: matched_data,
            interpretation,
            confidence: 80, // Default confidence for pattern matches
        })
    }

    /// Generate a structure hypothesis based on matches
    fn generate_hypothesis(&self, matches: &[PatternMatch]) -> Option<String> {
        if matches.is_empty() {
            return None;
        }

        let mut result = String::from("struct DetectedStructure {\n");

        for (i, m) in matches.iter().enumerate() {
            result.push_str(&format!(
                "    field_{}: {}, // {}\n",
                i, m.pattern_name, m.interpretation
            ));
        }

        result.push_str("}");
        Some(result)
    }
}

impl BorshAnalyzer for PatternAnalyzer {
    fn name(&self) -> &'static str {
        "PatternAnalyzer"
    }

    fn description(&self) -> &'static str {
        "Analyzes Borsh data by matching against known patterns in a dictionary"
    }

    fn analyze(
        &self,
        data: &[u8],
        dictionary: &PatternDictionary,
        options: &AnalysisOptions,
    ) -> AnalysisResult {
        let mut matches = Vec::new();
        let mut offset = 0;

        // Try to find matches at each offset
        while offset < data.len() {
            if let Some(pattern_match) =
                self.match_patterns_at_offset(data, offset, dictionary, options.include_raw_bytes)
            {
                // Only include matches that meet the confidence threshold
                let pattern_match_len = pattern_match.length;
                if pattern_match.confidence >= options.min_confidence {
                    matches.push(pattern_match);
                    offset += pattern_match_len;
                } else {
                    offset += 1;
                }
            } else {
                offset += 1;
            }

            // Limit number of matches if specified
            if let Some(max_matches) = options.max_matches {
                if matches.len() >= max_matches {
                    break;
                }
            }
        }

        // Calculate overall confidence
        let confidence = AnalysisResult::calculate_confidence(&matches);

        // Generate structure hypothesis
        let structure_hypothesis = self.generate_hypothesis(&matches);

        let matches_len = matches.len();
        AnalysisResult {
            matches,
            structure_hypothesis,
            confidence,
            description: format!("Pattern-based analysis found {} matches", matches_len),
        }
    }
}

impl Default for PatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
