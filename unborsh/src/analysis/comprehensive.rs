// src/analysis/comprehensive.rs

use crate::analysis::traits::{BorshAnalyzer, CompositeAnalyzer};
use crate::analysis::{pattern, recursive, probabilistic};
use crate::core::{
    AnalysisOptions,
    AnalysisResult,
    PatternDictionary,
    PatternMatch,
};

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

            let weight = weights.iter()
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

        AnalysisResult::new(deduplicated, hypothesis, avg_confidence, &combined_description)
    }

    /// Generate a comprehensive structure hypothesis
    fn generate_hypothesis(&self, matches: &[PatternMatch]) -> String {
        let mut result = String::new();

        // Check if first match is an enum variant
        let is_enum = matches.first()
            .map(|m| m.pattern_name == "EnumVariant")
            .unwrap_or(false);

        if is_enum {
            let variant = matches.first().unwrap();
            result.push_str(&format!("enum BorshEnum {{\n    Variant{}, // {}\n    // Other variants\n}}\n\n",
                                     variant.data.first().unwrap_or(&0), variant.interpretation));

            result.push_str("struct VariantData {\n");

            // Skip the enum variant itself in the struct fields
            for (i, m) in matches.iter().enumerate().skip(1) {
                result.push_str(&format!("    field_{}: {}, // {} (confidence: {}%)\n",
                                         i - 1, m.pattern_name, m.interpretation, m.confidence));
            }
        } else {
            result.push_str("struct BorshStructure {\n");

            for (i, m) in matches.iter().enumerate() {
                result.push_str(&format!("    field_{}: {}, // {} (confidence: {}%)\n",
                                         i, m.pattern_name, m.interpretation, m.confidence));
            }
        }

        result.push_str("}\n\n");

        // Add implementation hints
        result.push_str("// Implementation hint (Rust):\n");
        result.push_str("#[derive(BorshSerialize, BorshDeserialize)]\n");

        if is_enum {
            result.push_str("enum BorshEnum {\n");
            let variant = matches.first().unwrap();
            result.push_str(&format!("    Variant{}(VariantData),\n", variant.data.first().unwrap_or(&0)));
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
                    _ => "/* unknown type */"
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
                    _ => "/* unknown type */"
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

    fn analyze(&self, data: &[u8], dictionary: &PatternDictionary, options: &AnalysisOptions) -> AnalysisResult {
        // First use the composite analyzer to run all analyzers
        let results = vec![
            pattern::PatternAnalyzer::new().analyze(data, dictionary, options),
            recursive::RecursiveAnalyzer::new().analyze(data, dictionary, options),
            probabilistic::ProbabilisticAnalyzer::new().analyze(data, dictionary, options)
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