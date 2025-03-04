// src/analysis/traits.rs
use crate::core::{AnalysisOptions, AnalysisResult, PatternDictionary};

/// Trait for Borsh data analysis algorithms
pub trait BorshAnalyzer {
    /// Name of the analyzer
    fn name(&self) -> &'static str;

    /// Description of the analysis strategy
    fn description(&self) -> &'static str;

    /// Analyze Borsh-serialized data
    fn analyze(
        &self,
        data: &[u8],
        dictionary: &PatternDictionary,
        options: &AnalysisOptions,
    ) -> AnalysisResult;
}

/// A composable analyzer that can be built from multiple sub-analyzers
pub struct CompositeAnalyzer {
    name: &'static str,
    description: &'static str,
    analyzers: Vec<Box<dyn BorshAnalyzer>>,
}

impl CompositeAnalyzer {
    /// Create a new composite analyzer
    pub fn new(
        name: &'static str,
        description: &'static str,
        analyzers: Vec<Box<dyn BorshAnalyzer>>,
    ) -> Self {
        Self {
            name,
            description,
            analyzers,
        }
    }
}

impl BorshAnalyzer for CompositeAnalyzer {
    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn analyze(
        &self,
        data: &[u8],
        dictionary: &PatternDictionary,
        options: &AnalysisOptions,
    ) -> AnalysisResult {
        let mut results = Vec::new();

        for analyzer in &self.analyzers {
            results.push(analyzer.analyze(data, dictionary, options));
        }

        AnalysisResult::merge(results)
    }
}
