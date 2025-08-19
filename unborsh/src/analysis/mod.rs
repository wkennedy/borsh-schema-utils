// src/analysis/mod.rs

pub mod comprehensive;
pub mod pattern;
pub mod probabilistic;
pub mod recursive;
pub mod traits;

// Re-export analyzer trait
pub use traits::BorshAnalyzer;

// Import internal modules
use crate::core::{AnalysisOptions, AnalysisResult, AnalysisStrategy, PatternDictionary};

/// Get an analyzer instance by strategy
pub fn get_analyzer(strategy: AnalysisStrategy) -> Box<dyn BorshAnalyzer> {
    match strategy {
        AnalysisStrategy::Pattern => Box::new(pattern::PatternAnalyzer::new()),
        AnalysisStrategy::Recursive => Box::new(recursive::RecursiveAnalyzer::new()),
        AnalysisStrategy::Probabilistic => Box::new(probabilistic::ProbabilisticAnalyzer::new()),
        AnalysisStrategy::Comprehensive => Box::new(comprehensive::ComprehensiveAnalyzer::new()),
    }
}

/// Analyze Borsh-serialized data using the specified strategy
pub fn analyze(
    data: &[u8],
    dictionary: &PatternDictionary,
    options: &AnalysisOptions,
) -> AnalysisResult {
    get_analyzer(options.strategy).analyze(data, dictionary, options)
}
