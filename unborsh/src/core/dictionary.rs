// src/core/dictionary.rs

use std::collections::HashMap;
use std::fmt;

use super::types::BorshPattern;

/// A dictionary of Borsh serialization patterns for pattern matching
#[derive(Debug)]
pub struct PatternDictionary {
    /// Collection of all patterns in the dictionary
    patterns: Vec<BorshPattern>,
    /// Maps pattern names to their index in the patterns vector
    pattern_map: HashMap<String, usize>,
}

impl PatternDictionary {
    /// Create a new empty pattern dictionary
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            pattern_map: HashMap::new(),
        }
    }

    /// Add a pattern to the dictionary
    pub fn add_pattern(&mut self, pattern: BorshPattern) {
        let index = self.patterns.len();
        self.pattern_map.insert(pattern.name.clone(), index);
        self.patterns.push(pattern);
    }

    /// Get a slice of all patterns in the dictionary
    pub fn get_patterns(&self) -> &[BorshPattern] {
        &self.patterns
    }

    /// Get a pattern by name
    pub fn get_pattern_by_name(&self, name: &str) -> Option<&BorshPattern> {
        self.pattern_map
            .get(name)
            .map(|&index| &self.patterns[index])
    }

    /// Get mutable access to a pattern by name
    pub fn get_pattern_by_name_mut(&mut self, name: &str) -> Option<&mut BorshPattern> {
        if let Some(&index) = self.pattern_map.get(name) {
            self.patterns.get_mut(index)
        } else {
            None
        }
    }

    /// Identify all patterns that match a byte slice
    pub fn identify_patterns(&self, bytes: &[u8]) -> Vec<(String, String)> {
        let mut results = Vec::new();

        for pattern in &self.patterns {
            if let Some(meaning) = pattern.extract_meaning(bytes) {
                results.push((pattern.name.clone(), meaning));
            }
        }

        results
    }

    /// Create a dictionary with default patterns
    pub fn from_defaults() -> Self {
        // This is just a placeholder - actual implementation will
        // reference patterns from the patterns module
        // let dict = Self::new();

        // Placeholder for importing pattern definitions
        // In the actual implementation, this would be:
        // use crate::patterns::primitives::*;
        // use crate::patterns::collections::*;
        //
        // dict.add_pattern(create_u8_pattern());
        // dict.add_pattern(create_u16_pattern());
        // etc.

        Self::new()
    }

    /// Merge multiple dictionaries into one
    pub fn merge(dictionaries: Vec<Self>) -> Self {
        let mut merged = Self::new();

        for dict in dictionaries {
            for pattern in dict.patterns {
                merged.add_pattern(pattern);
            }
        }

        merged
    }

    /// Create a dictionary with only the specified patterns
    pub fn with_patterns(patterns: Vec<BorshPattern>) -> Self {
        let mut dict = Self::new();

        for pattern in patterns {
            dict.add_pattern(pattern);
        }

        dict
    }

    /// Get the number of patterns in the dictionary
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Returns true if the dictionary is empty
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

impl fmt::Display for PatternDictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Borsh Pattern Dictionary ({} patterns)",
            self.patterns.len()
        )?;

        for (i, pattern) in self.patterns.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", pattern)?;
        }

        Ok(())
    }
}

impl Default for PatternDictionary {
    fn default() -> Self {
        Self::from_defaults()
    }
}
