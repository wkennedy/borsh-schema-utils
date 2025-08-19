// src/core/mod.rs
pub mod dictionary;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use dictionary::PatternDictionary;
pub use types::{AnalysisOptions, AnalysisResult, AnalysisStrategy, BorshPattern, PatternMatch};
