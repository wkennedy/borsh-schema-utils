// src/core/mod.rs
pub mod types;
pub mod dictionary;
pub mod utils;

// Re-export commonly used types
pub use types::{
    BorshPattern,
    PatternMatch,
    AnalysisResult,
    AnalysisStrategy,
    AnalysisOptions,
};
pub use dictionary::PatternDictionary;