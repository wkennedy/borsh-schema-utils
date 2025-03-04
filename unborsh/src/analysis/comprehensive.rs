// src/analysis/comprehensive.rs
use crate::analysis::traits::{BorshAnalyzer, CompositeAnalyzer};
use crate::analysis::{pattern, probabilistic, recursive};
use crate::core::{AnalysisOptions, AnalysisResult, PatternDictionary, PatternMatch};

/// Analyzer that combines all approaches for the most comprehensive results
pub struct ComprehensiveAnalyzer {
    composite: CompositeAnalyzer,
}

impl ComprehensiveAnalyzer {
    /// Create a new comprehensive analyzer
    pub fn new() -> Self {
        let analyzers: Vec<Box<dyn BorshAnalyzer>> = vec![
            Box::new(pattern::PatternAnalyzer::new()),
            Box::new(recursive::RecursiveAnalyzer::new()),
            Box::new(probabilistic::ProbabilisticAnalyzer::new()),
        ];

        Self {
            composite: CompositeAnalyzer::new(
                "ComprehensiveAnalyzer",
                "Combines pattern matching, recursive analysis, and probabilistic analysis",
                analyzers,
            ),
        }
    }

    /// Custom result merging with weighted confidence
    fn merge_results(&self, results: Vec<AnalysisResult>) -> AnalysisResult {
        let _analyzer_name = self.composite.name();
        if results.is_empty() {
            return AnalysisResult::new(vec![], None, 0, "No analysis results to merge");
        }

        let mut all_matches = Vec::new();
        let mut descriptions = Vec::new();

        // Weight confidence by analyzer type (pattern matching has highest confidence)
        let weights = [
            ("PatternAnalyzer", 1.2),
            ("RecursiveAnalyzer", 1.0),
            ("ProbabilisticAnalyzer", 0.8),
        ];

        let mut weighted_confidence_sum = 0.0;
        let mut weight_sum = 0.0;

        for (i, result) in results.iter().enumerate() {
            all_matches.extend(result.matches.clone());
            descriptions.push(result.description.clone());

            // Find the weight for this analyzer
            let analyzer_name = match i {
                0 => "PatternAnalyzer",
                1 => "RecursiveAnalyzer",
                2 => "ProbabilisticAnalyzer",
                _ => "Unknown",
            };

            let weight = weights
                .iter()
                .find(|(name, _)| *name == analyzer_name)
                .map(|(_, w)| *w)
                .unwrap_or(1.0);

            weighted_confidence_sum += result.confidence as f64 * weight;
            weight_sum += weight;
        }

        // Calculate weighted average confidence
        let avg_confidence = (weighted_confidence_sum / weight_sum) as u8;

        // Deduplicate matches at the same offset by keeping the highest confidence match
        all_matches.sort_by_key(|m| (m.offset, -(m.confidence as i16)));

        let mut deduplicated = Vec::new();
        let mut seen_offsets = std::collections::HashSet::new();

        for m in all_matches {
            if !seen_offsets.contains(&m.offset) {
                seen_offsets.insert(m.offset);
                deduplicated.push(m);
            }
        }

        // Sort by offset for final output
        deduplicated.sort_by_key(|m| m.offset);

        // Generate a combined hypothesis
        let hypothesis = if deduplicated.is_empty() {
            None
        } else {
            Some(self.generate_hypothesis(&deduplicated))
        };

        let combined_description = format!(
            "Comprehensive analysis using multiple strategies.\nCombined results from {} analyzers.",
            results.len()
        );

        AnalysisResult::new(
            deduplicated,
            hypothesis,
            avg_confidence,
            &combined_description,
        )
    }

    /// Generate a comprehensive structure hypothesis
    #[allow(dead_code)]
    fn generate_hypothesis(&self, matches: &[PatternMatch]) -> String {
        let mut result = String::new();

        // Check if first match is an enum variant
        let is_enum = matches
            .first()
            .map(|m| m.pattern_name == "EnumVariant")
            .unwrap_or(false);

        if is_enum {
            let variant = matches.first().unwrap();
            result.push_str(&format!(
                "enum BorshEnum {{\n    Variant{}, // {}\n    // Other variants\n}}\n\n",
                variant.data.first().unwrap_or(&0),
                variant.interpretation
            ));

            result.push_str("struct VariantData {\n");

            // Skip the enum variant itself in the struct fields
            for (i, m) in matches.iter().enumerate().skip(1) {
                result.push_str(&format!(
                    "    field_{}: {}, // {} (confidence: {}%)\n",
                    i - 1,
                    m.pattern_name,
                    m.interpretation,
                    m.confidence
                ));
            }
        } else {
            result.push_str("struct BorshStructure {\n");

            for (i, m) in matches.iter().enumerate() {
                result.push_str(&format!(
                    "    field_{}: {}, // {} (confidence: {}%)\n",
                    i, m.pattern_name, m.interpretation, m.confidence
                ));
            }
        }

        result.push_str("}\n\n");

        // Add implementation hints
        result.push_str("// Implementation hint (Rust):\n");
        result.push_str("#[derive(BorshSerialize, BorshDeserialize)]\n");

        if is_enum {
            result.push_str("enum BorshEnum {\n");
            let variant = matches.first().unwrap();
            result.push_str(&format!(
                "    Variant{}(VariantData),\n",
                variant.data.first().unwrap_or(&0)
            ));
            result.push_str("    // Other variants\n");
            result.push_str("}\n\n");

            result.push_str("#[derive(BorshSerialize, BorshDeserialize)]\n");
            result.push_str("struct VariantData {\n");

            for (i, m) in matches.iter().enumerate().skip(1) {
                let rust_type = match m.pattern_name.as_str() {
                    "String" => "String",
                    "u8" => "u8",
                    "u32" => "u32",
                    "u64" => "u64",
                    "bool" => "bool",
                    "Vec<u8>" => "Vec<u8>",
                    "Option::Some" | "Option::None" => "Option<T>",
                    _ => "/* unknown type */",
                };

                result.push_str(&format!("    field_{}: {},\n", i - 1, rust_type));
            }
        } else {
            result.push_str("struct BorshStructure {\n");

            for (i, m) in matches.iter().enumerate() {
                let rust_type = match m.pattern_name.as_str() {
                    "String" => "String",
                    "u8" => "u8",
                    "u32" => "u32",
                    "u64" => "u64",
                    "bool" => "bool",
                    "Vec<u8>" => "Vec<u8>",
                    "Option::Some" | "Option::None" => "Option<T>",
                    _ => "/* unknown type */",
                };

                result.push_str(&format!("    field_{}: {},\n", i, rust_type));
            }
        }

        result.push_str("}\n");

        result
    }
}

impl BorshAnalyzer for ComprehensiveAnalyzer {
    fn name(&self) -> &'static str {
        "ComprehensiveAnalyzer"
    }

    fn description(&self) -> &'static str {
        "Combines pattern matching, recursive analysis, and probabilistic analysis for the most comprehensive results"
    }

    fn analyze(
        &self,
        data: &[u8],
        dictionary: &PatternDictionary,
        options: &AnalysisOptions,
    ) -> AnalysisResult {
        // First use the composite analyzer to run all analyzers
        let results = vec![
            pattern::PatternAnalyzer::new().analyze(data, dictionary, options),
            recursive::RecursiveAnalyzer::new().analyze(data, dictionary, options),
            probabilistic::ProbabilisticAnalyzer::new().analyze(data, dictionary, options),
        ];

        // Then use custom merging logic
        self.merge_results(results)
    }
}

impl Default for ComprehensiveAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{AnalysisOptions, AnalysisResult, PatternMatch};
    use crate::patterns;

    #[test]
    fn test_comprehensive_analyzer_new() {
        // Test that we can create a new ComprehensiveAnalyzer
        let analyzer = ComprehensiveAnalyzer::new();
        assert_eq!(analyzer.name(), "ComprehensiveAnalyzer");
        assert!(analyzer.description().contains("Combines pattern matching"));
    }

    #[test]
    fn test_merge_results_empty() {
        // Test merging with empty results array
        let analyzer = ComprehensiveAnalyzer::new();
        let results = Vec::new();
        let merged = analyzer.merge_results(results);

        assert_eq!(merged.matches.len(), 0);
        assert_eq!(merged.confidence, 0);
        assert!(merged.description.contains("No analysis results"));
        assert!(merged.structure_hypothesis.is_none());
    }

    #[test]
    fn test_merge_results_weighted_confidence() {
        // Test that confidence is properly weighted
        let analyzer = ComprehensiveAnalyzer::new();

        // Create test results with different confidence values
        let results = vec![
            AnalysisResult::new(vec![], None, 80, "Pattern analysis"),   // PatternAnalyzer (weight 1.2)
            AnalysisResult::new(vec![], None, 60, "Recursive analysis"), // RecursiveAnalyzer (weight 1.0)
            AnalysisResult::new(vec![], None, 40, "Probabilistic analysis"), // ProbabilisticAnalyzer (weight 0.8)
        ];

        let merged = analyzer.merge_results(results);

        // Calculate expected weighted confidence:
        // (80 * 1.2 + 60 * 1.0 + 40 * 0.8) / (1.2 + 1.0 + 0.8) = (96 + 60 + 32) / 3.0 = 188 / 3.0 = 62.67 -> 62
        assert_eq!(merged.confidence, 62);
    }

    #[test]
    fn test_merge_results_deduplication() {
        // Test that matches at the same offset are deduplicated
        let analyzer = ComprehensiveAnalyzer::new();

        // Create test matches at the same offset but with different confidence
        let match1 = PatternMatch {
            pattern_name: "String".to_string(),
            offset: 0,
            length: 10,
            data: vec![],
            interpretation: "Test String".to_string(),
            confidence: 90,
        };

        let match2 = PatternMatch {
            pattern_name: "Vec<u8>".to_string(),
            offset: 0, // Same offset as match1
            length: 12,
            data: vec![],
            interpretation: "Test Vec".to_string(),
            confidence: 60,
        };

        let match3 = PatternMatch {
            pattern_name: "u32".to_string(),
            offset: 10, // Different offset
            length: 4,
            data: vec![],
            interpretation: "Test u32".to_string(),
            confidence: 80,
        };

        let results = vec![
            AnalysisResult::new(vec![match1.clone()], None, 80, "Pattern analysis"),
            AnalysisResult::new(vec![match2.clone()], None, 60, "Recursive analysis"),
            AnalysisResult::new(vec![match3.clone()], None, 40, "Probabilistic analysis"),
        ];

        let merged = analyzer.merge_results(results);

        // Should have 2 matches (match1 and match3)
        // match2 should be deduplicated since it has the same offset as match1 but lower confidence
        assert_eq!(merged.matches.len(), 2);
        assert_eq!(merged.matches[0].offset, 0); // First match should be at offset 0
        assert_eq!(merged.matches[0].pattern_name, "String"); // Should keep match1 (higher confidence)
        assert_eq!(merged.matches[1].offset, 10); // Second match should be at offset 10
    }

    #[test]
    fn test_generate_hypothesis_struct() {
        // Test hypothesis generation for a regular struct
        let analyzer = ComprehensiveAnalyzer::new();

        let matches = vec![
            PatternMatch {
                pattern_name: "String".to_string(),
                offset: 0,
                length: 10,
                data: vec![],
                interpretation: "Name".to_string(),
                confidence: 90,
            },
            PatternMatch {
                pattern_name: "u32".to_string(),
                offset: 10,
                length: 4,
                data: vec![],
                interpretation: "Age".to_string(),
                confidence: 80,
            },
        ];

        let hypothesis = analyzer.generate_hypothesis(&matches);

        // Check that it correctly generates a struct
        assert!(hypothesis.contains("struct BorshStructure"));
        assert!(hypothesis.contains("field_0: String")); // First field should be String
        assert!(hypothesis.contains("field_1: u32"));    // Second field should be u32
        assert!(hypothesis.contains("#[derive(BorshSerialize, BorshDeserialize)]")); // Should include derive macro
    }

    #[test]
    fn test_generate_hypothesis_enum() {
        // Test hypothesis generation for an enum variant
        let analyzer = ComprehensiveAnalyzer::new();

        let matches = vec![
            PatternMatch {
                pattern_name: "EnumVariant".to_string(),
                offset: 0,
                length: 1,
                data: vec![2], // Variant 2
                interpretation: "Variant 2".to_string(),
                confidence: 90,
            },
            PatternMatch {
                pattern_name: "String".to_string(),
                offset: 1,
                length: 10,
                data: vec![],
                interpretation: "Name".to_string(),
                confidence: 80,
            },
        ];

        let hypothesis = analyzer.generate_hypothesis(&matches);

        // Check that it correctly generates an enum with variants
        assert!(hypothesis.contains("enum BorshEnum"));
        assert!(hypothesis.contains("Variant2")); // Should reference variant 2
        assert!(hypothesis.contains("struct VariantData")); // Should generate struct for variant data
        assert!(hypothesis.contains("field_0: String")); // First field of variant data should be String
    }

    #[test]
    fn test_analyze() {
        // Test the analyze method with some sample data
        let analyzer = ComprehensiveAnalyzer::new();
        let dictionary = patterns::create_pattern_dictionary();
        let options = AnalysisOptions::default();

        // Simple string in Borsh format: length (11) + "Hello World"
        let data = vec![11, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];

        let result = analyzer.analyze(&data, &dictionary, &options);

        // Should identify at least one match (the string)
        assert!(!result.matches.is_empty());
        assert!(result.confidence > 0);
        assert!(result.structure_hypothesis.is_some());

        // Should find a String pattern match
        let has_string_match = result.matches.iter().any(|m| m.pattern_name == "String");
        assert!(has_string_match, "Should identify 'String' pattern");
    }

    #[test]
    fn test_analyze_with_empty_data() {
        // Test analysis with empty data
        let analyzer = ComprehensiveAnalyzer::new();
        let dictionary = patterns::create_pattern_dictionary();
        let options = AnalysisOptions::default();

        let empty_data: Vec<u8> = vec![];

        let result = analyzer.analyze(&empty_data, &dictionary, &options);

        // Should not crash but produce empty result
        assert!(result.matches.is_empty());
    }

    #[test]
    fn test_analyzer_respects_options() {
        // Test that analyzer respects max_matches option
        let analyzer = ComprehensiveAnalyzer::new();
        let dictionary = patterns::create_pattern_dictionary();

        // Create an options with max_matches = 1
        let mut options = AnalysisOptions::default();
        options.max_matches = Some(1);

        // A more complex structure (enum variant + string + u32 + bool)
        let data = vec![
            1, // Enum variant discriminant
            5, 0, 0, 0, 72, 101, 108, 108, 111, // String "Hello"
            42, 0, 0, 0, // u32 value 42
            1, // bool true
        ];

        let result = analyzer.analyze(&data, &dictionary, &options);

        // Should respect max_matches setting and have at most 1 match
        assert!(result.matches.len() <= 1);
    }
}