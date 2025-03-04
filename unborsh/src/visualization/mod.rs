// src/visualization/mod.rs

//! Visualization tools for Borsh analysis results
//!
//! This module provides tools to visualize the structure of Borsh-serialized data
//! and the results of analysis.

mod ascii;
mod html;


use crate::core::AnalysisResult;
use std::io::Write;

/// Configuration options for visualization
#[derive(Debug, Clone)]
pub struct VisualizationOptions {
    /// Maximum depth to render in the visualization
    pub max_depth: usize,
    /// Whether to include match interpretations
    pub show_interpretations: bool,
    /// Whether to include confidence scores
    pub show_confidence: bool,
    /// Whether to show raw bytes (hex)
    pub show_raw_bytes: bool,
    /// Maximum number of bytes to show per match
    pub max_bytes_per_match: usize,
    /// Color theme to use (for formats that support it)
    pub theme: ColorTheme,
}

impl Default for VisualizationOptions {
    fn default() -> Self {
        Self {
            max_depth: 10,
            show_interpretations: true,
            show_confidence: true,
            show_raw_bytes: true,
            max_bytes_per_match: 32,
            theme: ColorTheme::default(),
        }
    }
}

/// Color theme for visualizations
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorTheme {
    /// Light theme (dark text on light background)
    #[default]
    Light,
    /// Dark theme (light text on dark background)
    Dark,
    /// High contrast theme
    HighContrast,
}

/// Visualization format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizationFormat {
    /// ASCII text visualization
    Ascii,
    /// HTML visualization
    Html,
}

/// Common trait for visualizers
pub trait Visualizer {
    /// Render a visualization of analysis results
    fn render(
        &self,
        results: &AnalysisResult,
        data: &[u8],
        options: &VisualizationOptions,
    ) -> String;

    /// Render to a writer (file, stdout, etc.)
    fn render_to(
        &self,
        results: &AnalysisResult,
        data: &[u8],
        writer: &mut dyn Write,
        options: &VisualizationOptions,
    ) -> std::io::Result<()> {
        let output = self.render(results, data, options);
        writer.write_all(output.as_bytes())?;
        Ok(())
    }
}

/// Get a visualizer for the specified format
pub fn get_visualizer(format: VisualizationFormat) -> Box<dyn Visualizer> {
    match format {
        VisualizationFormat::Ascii => Box::new(ascii::AsciiVisualizer {}),
        VisualizationFormat::Html => Box::new(html::HtmlVisualizer {}),
    }
}

/// Create a visualization of analysis results
pub fn visualize(
    results: &AnalysisResult,
    data: &[u8],
    format: VisualizationFormat,
    options: &VisualizationOptions,
) -> String {
    get_visualizer(format).render(results, data, options)
}

/// Create a visualization and write it to a file
pub fn visualize_to_file(
    results: &AnalysisResult,
    data: &[u8],
    format: VisualizationFormat,
    file_path: &str,
    options: &VisualizationOptions,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create(file_path)?;
    get_visualizer(format).render_to(results, data, &mut file, options)
}
