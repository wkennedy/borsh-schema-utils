// src/core/types.rs
use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Define a struct without the function pointer for serialization purposes
#[derive(Serialize, Deserialize)]
struct BorshPatternData {
    name: String,
    description: String,
    binary_pattern: Vec<u8>,
    mask: Vec<u8>,
    example: String,
}

#[derive(Debug, Clone)]
pub struct BorshPattern {
    /// Name of the pattern (e.g., "String", "u32", "Option::Some")
    pub name: String,
    /// Description of what this pattern represents
    pub description: String,
    /// Binary representation of the pattern
    pub binary_pattern: Vec<u8>,
    /// Mask indicating which bytes must match exactly (0xFF) vs. any value (0x00)
    pub mask: Vec<u8>,
    /// Example of the pattern in hex with explanation
    pub example: String,
    /// Function to extract meaning if pattern matches
    pub matcher: fn(&[u8]) -> Option<String>,
}

impl BorshPattern {
    /// Create a new Borsh pattern
    pub fn new(
        name: &str,
        description: &str,
        binary_pattern: Vec<u8>,
        mask: Vec<u8>,
        example: &str,
        matcher: fn(&[u8]) -> Option<String>
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            binary_pattern,
            mask,
            example: example.to_string(),
            matcher,
        }
    }

    /// Check if this pattern matches the given bytes
    pub fn matches(&self, bytes: &[u8]) -> bool {
        if bytes.len() < self.binary_pattern.len() {
            return false;
        }

        for i in 0..self.binary_pattern.len() {
            // For bytes where mask is 0xFF, do exact comparison
            // For bytes where mask is 0x00, accept any value
            if self.mask[i] == 0xFF && bytes[i] != self.binary_pattern[i] {
                return false;
            }
        }

        true
    }

    /// If pattern matches, extract meaning from the bytes
    pub fn extract_meaning(&self, bytes: &[u8]) -> Option<String> {
        if self.matches(bytes) {
            (self.matcher)(bytes)
        } else {
            None
        }
    }

    /// Length of the pattern in bytes
    pub fn len(&self) -> usize {
        self.binary_pattern.len()
    }

    /// Returns true if the pattern is empty
    pub fn is_empty(&self) -> bool {
        self.binary_pattern.is_empty()
    }
}

// Custom serialization implementation
impl Serialize for BorshPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = BorshPatternData {
            name: self.name.clone(),
            description: self.description.clone(),
            binary_pattern: self.binary_pattern.clone(),
            mask: self.mask.clone(),
            example: self.example.clone(),
        };
        data.serialize(serializer)
    }
}

// Registry function to retrieve matchers
fn get_matcher_for_pattern(name: &str) -> fn(&[u8]) -> Option<String> {
    // Default matcher that always returns None
    let default_matcher: fn(&[u8]) -> Option<String> = |_| None;

    // This would need to be populated with all your pattern matchers
    match name {
        // For example only - you would need to define these functions elsewhere
        "u8" => |bytes| if !bytes.is_empty() { Some(format!("u8: {}", bytes[0])) } else { None },
        "String" => |bytes| {
            if bytes.len() >= 4 {
                let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                if bytes.len() >= 4 + len as usize {
                    if let Ok(s) = std::str::from_utf8(&bytes[4..4 + len as usize]) {
                        return Some(format!("String: \"{}\"", s));
                    }
                }
            }
            None
        },
        // Add more pattern matchers here
        _ => default_matcher,
    }
}

// Custom deserialization implementation
impl<'de> Deserialize<'de> for BorshPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = BorshPatternData::deserialize(deserializer)?;

        // Look up the matcher function based on the pattern name
        let matcher = get_matcher_for_pattern(&data.name);

        Ok(BorshPattern {
            name: data.name,
            description: data.description,
            binary_pattern: data.binary_pattern,
            mask: data.mask,
            example: data.example,
            matcher,
        })
    }
}

// Implement Display for BorshPattern
impl fmt::Display for BorshPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Pattern: {}", self.name)?;
        writeln!(f, "Description: {}", self.description)?;

        // Format binary pattern with variables highlighted
        let mut pattern_str = String::new();
        for i in 0..self.binary_pattern.len() {
            if self.mask[i] == 0xFF {
                pattern_str.push_str(&format!("{:02x} ", self.binary_pattern[i]));
            } else {
                pattern_str.push_str(".. ");
            }
        }
        writeln!(f, "Binary Pattern: {}", pattern_str.trim())?;
        writeln!(f, "Example: {}", self.example)
    }
}

/// Represents a detected pattern match in Borsh data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    /// Name of the matched pattern
    pub pattern_name: String,
    /// Offset in the original data where the match was found
    pub offset: usize,
    /// Length of the matched pattern in bytes
    pub length: usize,
    /// Raw data for the match (if included)
    pub data: Vec<u8>,
    /// Human-readable interpretation of the match
    pub interpretation: String,
    /// Confidence level (0-100) of the match
    pub confidence: u8,
}

impl PatternMatch {
    /// Create a new pattern match
    pub fn new(
        pattern_name: &str,
        offset: usize,
        length: usize,
        data: Vec<u8>,
        interpretation: String,
        confidence: u8,
    ) -> Self {
        Self {
            pattern_name: pattern_name.to_string(),
            offset,
            length,
            data,
            interpretation,
            confidence,
        }
    }
}

impl fmt::Display for PatternMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Match: {} at offset {} ({} bytes, {}% confidence) - {}",
            self.pattern_name, self.offset, self.length, self.confidence, self.interpretation
        )
    }
}

/// Result of a Borsh data analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// List of all pattern matches found
    pub matches: Vec<PatternMatch>,
    /// Hypothesized structure based on the analysis
    pub structure_hypothesis: Option<String>,
    /// Overall confidence level (0-100) of the analysis
    pub confidence: u8,
    /// Description of the analysis results
    pub description: String,
}

impl AnalysisResult {
    /// Create a new analysis result
    pub fn new(
        matches: Vec<PatternMatch>,
        structure_hypothesis: Option<String>,
        confidence: u8,
        description: &str,
    ) -> Self {
        Self {
            matches,
            structure_hypothesis,
            confidence,
            description: description.to_string(),
        }
    }

    /// Calculate the overall confidence based on individual matches
    pub fn calculate_confidence(matches: &[PatternMatch]) -> u8 {
        if matches.is_empty() {
            return 0;
        }

        let sum: u32 = matches.iter().map(|m| m.confidence as u32).sum();
        (sum / matches.len() as u32) as u8
    }

    /// Merge multiple analysis results
    pub fn merge(results: Vec<Self>) -> Self {
        if results.is_empty() {
            return Self::new(vec![], None, 0, "No analysis results to merge");
        }

        let mut all_matches = Vec::new();
        let mut descriptions = Vec::new();
        let mut confidence_sum = 0;

        for result in &results {
            all_matches.extend(result.matches.clone());
            descriptions.push(result.description.clone());
            confidence_sum += result.confidence as u32;
        }

        // Deduplicate matches at the same offset
        all_matches.sort_by_key(|m| m.offset);
        let mut deduplicated = Vec::new();
        let mut current_offset = None;

        for m in all_matches {
            let m_offset = m.offset;
            if current_offset != Some(m_offset) {
                deduplicated.push(m);
                current_offset = Some(m_offset);
            } else if let Some(last) = deduplicated.last_mut() {
                // Keep the match with higher confidence
                if m.confidence > last.confidence {
                    *last = m;
                }
            }
        }

        // Generate a combined hypothesis if possible
        let hypothesis = if deduplicated.is_empty() {
            None
        } else {
            Some(Self::generate_hypothesis(&deduplicated))
        };

        let avg_confidence = (confidence_sum / results.len() as u32) as u8;
        let combined_description = format!("Combined analysis results:\n{}", descriptions.join("\n"));

        Self::new(deduplicated, hypothesis, avg_confidence, &combined_description)
    }

    /// Generate a structure hypothesis from matches
    fn generate_hypothesis(matches: &[PatternMatch]) -> String {
        let mut result = String::from("struct BorshStructure {\n");

        for (i, m) in matches.iter().enumerate() {
            result.push_str(&format!("    field_{}: {}, // {}\n", i, m.pattern_name, m.interpretation));
        }

        result.push_str("}");
        result
    }
}

impl fmt::Display for AnalysisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Analysis Result ({}% confidence):", self.confidence)?;
        writeln!(f, "{}", self.description)?;

        if !self.matches.is_empty() {
            writeln!(f, "\nMatches found:")?;
            for (i, m) in self.matches.iter().enumerate() {
                writeln!(f, "  {}. {}", i + 1, m)?;
            }
        }

        if let Some(hypothesis) = &self.structure_hypothesis {
            writeln!(f, "\nStructure Hypothesis:\n{}", hypothesis)?;
        }

        Ok(())
    }
}

/// Available analysis strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisStrategy {
    /// Pattern-based analysis (uses dictionary matching)
    Pattern,
    /// Recursive analysis (hierarchical breakdown)
    Recursive,
    /// Probabilistic analysis (confidence-based guessing)
    Probabilistic,
    /// Comprehensive analysis (combines all strategies)
    Comprehensive,
}

impl fmt::Display for AnalysisStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pattern => write!(f, "Pattern"),
            Self::Recursive => write!(f, "Recursive"),
            Self::Probabilistic => write!(f, "Probabilistic"),
            Self::Comprehensive => write!(f, "Comprehensive"),
        }
    }
}

/// Options for controlling the analysis process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisOptions {
    /// Analysis strategy to use
    pub strategy: AnalysisStrategy,
    /// Maximum recursion depth for recursive analysis
    pub max_depth: usize,
    /// Minimum confidence level for including matches
    pub min_confidence: u8,
    /// Whether to include raw bytes in the results
    pub include_raw_bytes: bool,
    /// Maximum number of matches to return
    pub max_matches: Option<usize>,
}

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            strategy: AnalysisStrategy::Comprehensive,
            max_depth: 5,
            min_confidence: 30,
            include_raw_bytes: false,
            max_matches: None,
        }
    }
}