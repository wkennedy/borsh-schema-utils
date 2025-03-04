// src/lib.rs

//! # unborsh
//!
//! A library for analyzing and understanding Borsh-serialized data
//! without requiring the original schema.
//!
//! `unborsh` provides tools to inspect, analyze, and reverse-engineer
//! data that has been serialized using the Borsh binary format. It
//! combines multiple analysis strategies to identify patterns and
//! recover structure from binary data.
//!
//! ## Features
//!
//! * **Pattern Matching**: Identify known Borsh serialization patterns
//! * **Recursive Analysis**: Break down complex nested structures
//! * **Probabilistic Analysis**: Make educated guesses about data structure
//! * **Comprehensive Analysis**: Combine all strategies for best results
//!
//! ## Example
//!
//! ```rust
//! use unborsh::{analyze, AnalysisOptions, AnalysisStrategy};
//!
//! // Some Borsh-serialized bytes (a serialized string "Hello World")
//! let data = vec![11, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];
//!
//! // Analyze with default options (comprehensive strategy)
//! let result = unborsh::analyze(&data);
//! println!("{}", result);
//!
//! // Analyze with a specific strategy
//! let result = unborsh::analyze_with_strategy(&data, AnalysisStrategy::Pattern);
//! println!("{}", result);
//!
//! // Analyze with custom options
//! let options = AnalysisOptions {
//!     strategy: AnalysisStrategy::Recursive,
//!     max_depth: 3,
//!     min_confidence: 50,
//!     include_raw_bytes: true,
//!     max_matches: Some(10),
//! };
//! let result = unborsh::analyze_with_options(&data, options);
//! println!("{}", result);
//! ```

// Module declarations
mod analysis;
mod core;
mod patterns;
mod visualization;

// Public exports
pub use analysis::traits::BorshAnalyzer;
pub use core::dictionary::PatternDictionary;
pub use core::types::{
    AnalysisOptions, AnalysisResult, AnalysisStrategy, BorshPattern, PatternMatch,
};
pub use core::utils;
pub use visualization::{
    visualize, visualize_to_file, ColorTheme, VisualizationFormat, VisualizationOptions,
};

// Re-export patterns module for custom pattern creation
pub mod pattern_definitions {
    pub use crate::patterns::*;
}

/// Analyze Borsh-serialized data with the given options
pub fn analyze_with_options(data: &[u8], options: AnalysisOptions) -> AnalysisResult {
    let dictionary = patterns::create_pattern_dictionary();
    analysis::analyze(data, &dictionary, &options)
}

/// Analyze Borsh-serialized data with default options (Comprehensive strategy)
pub fn analyze(data: &[u8]) -> AnalysisResult {
    analyze_with_options(data, AnalysisOptions::default())
}

/// Analyze Borsh-serialized data with a specific strategy
pub fn analyze_with_strategy(data: &[u8], strategy: AnalysisStrategy) -> AnalysisResult {
    let options = AnalysisOptions { strategy, ..Default::default() };
    analyze_with_options(data, options)
}

/// Analyze Borsh-serialized data with a custom pattern dictionary
pub fn analyze_with_dictionary(
    data: &[u8],
    dictionary: &PatternDictionary,
    options: AnalysisOptions,
) -> AnalysisResult {
    analysis::analyze(data, dictionary, &options)
}

/// Create a custom analyzer from specific strategies
pub fn create_custom_analyzer(strategies: Vec<AnalysisStrategy>) -> impl BorshAnalyzer {
    let analyzers = strategies
        .into_iter()
        .map(|strategy| analysis::get_analyzer(strategy))
        .collect();

    analysis::traits::CompositeAnalyzer::new(
        "CustomAnalyzer",
        "Custom analyzer combining multiple strategies",
        analyzers,
    )
}

/// Extract a human-readable hypothesis about the data structure
pub fn extract_structure_hypothesis(data: &[u8]) -> Option<String> {
    analyze(data).structure_hypothesis
}

/// Try to interpret the data as a specific known type
pub fn interpret_as<T: AsRef<str>>(data: &[u8], type_name: T) -> Option<String> {
    let dictionary = patterns::create_pattern_dictionary();
    let pattern = dictionary.get_pattern_by_name(type_name.as_ref())?;
    pattern.extract_meaning(data)
}

/// Convert hex string to bytes for analysis
pub fn analyze_hex(hex_str: &str) -> Result<AnalysisResult, hex::FromHexError> {
    let data = hex::decode(hex_str)?;
    Ok(analyze(&data))
}

/// Analyze Base64-encoded data
#[cfg(feature = "base64_support")]
pub fn analyze_base64(base64_str: &str) -> Result<AnalysisResult, base64::DecodeError> {
    use base64::Engine;
    let data = base64::engine::general_purpose::STANDARD.decode(base64_str)?;
    Ok(analyze(&data))
}

/// Analyze and create visualization of the results
pub fn analyze_and_visualize(
    data: &[u8],
    options: AnalysisOptions,
    viz_format: VisualizationFormat,
    viz_options: &VisualizationOptions,
) -> (AnalysisResult, String) {
    let result = analyze_with_options(data, options);
    let visualization = visualize(&result, data, viz_format, viz_options);
    (result, visualization)
}

/// Analyze and save visualization to file
pub fn analyze_and_visualize_to_file(
    data: &[u8],
    options: AnalysisOptions,
    viz_format: VisualizationFormat,
    file_path: &str,
    viz_options: &VisualizationOptions,
) -> std::io::Result<AnalysisResult> {
    let result = analyze_with_options(data, options);
    visualize_to_file(&result, data, viz_format, file_path, viz_options)?;
    Ok(result)
}
