// src/visualization/ascii.rs

//! ASCII-based visualization of Borsh analysis results

use super::{VisualizationOptions, Visualizer};
use crate::core::{AnalysisResult, PatternMatch};
use std::collections::HashMap;

/// ASCII-based visualizer
pub struct AsciiVisualizer;

impl Visualizer for AsciiVisualizer {
    fn render(
        &self,
        results: &AnalysisResult,
        data: &[u8],
        options: &VisualizationOptions,
    ) -> String {
        let mut output = String::new();

        // Add header
        output.push_str(&format!(
            "=== Borsh Structure Visualization ({} bytes) ===\n\n",
            data.len()
        ));
        output.push_str(&format!("Confidence: {}%\n", results.confidence));
        output.push_str(&format!("Analysis: {}\n\n", results.description));

        // Create a map of offset -> matches for efficient lookup
        let mut offset_map: HashMap<usize, &PatternMatch> = HashMap::new();
        for m in &results.matches {
            offset_map.insert(m.offset, m);
        }

        // Sort matches by offset for rendering
        let mut sorted_matches = results.matches.clone();
        sorted_matches.sort_by_key(|m| m.offset);

        // Determine nesting level for each match
        let nesting_levels = determine_nesting_levels(&sorted_matches);

        // Render the structure diagram
        output.push_str("Structure Diagram:\n");
        output.push_str("┌────────────────────────────────────────────────────────┐\n");

        let mut current_offset = 0;
        while current_offset < data.len() {
            if let Some(m) = offset_map.get(&current_offset) {
                // Found a match at the current offset
                let level = nesting_levels.get(&current_offset).unwrap_or(&0);
                let indent = "│ ".repeat(*level);

                // Render field name and type
                output.push_str(&format!(
                    "│ {}{} {}\n",
                    indent,
                    "●",
                    m.pattern_name
                ));

                // Show interpretation if enabled
                if options.show_interpretations {
                    output.push_str(&format!("│ {}  └─ {}\n", indent, m.interpretation));
                }

                // Show confidence if enabled
                if options.show_confidence {
                    let confidence_indicator = match m.confidence {
                        90..=100 => "★★★★★",
                        70..=89 => "★★★★☆",
                        50..=69 => "★★★☆☆",
                        30..=49 => "★★☆☆☆",
                        _ => "★☆☆☆☆",
                    };
                    output.push_str(&format!(
                        "│ {}  └─ Confidence: {}% {}\n",
                        indent, m.confidence, confidence_indicator
                    ));
                }

                // Show raw bytes if enabled
                if options.show_raw_bytes && !m.data.is_empty() {
                    let bytes_to_show = m.data.len().min(options.max_bytes_per_match);
                    let bytes_str = format_bytes(&m.data[..bytes_to_show]);
                    output.push_str(&format!("│ {}  └─ Bytes: {}\n", indent, bytes_str));

                    if bytes_to_show < m.data.len() {
                        output.push_str(&format!(
                            "│ {}     └─ ... ({} more bytes)\n",
                            indent,
                            m.data.len() - bytes_to_show
                        ));
                    }
                }

                // Add offset and length info
                output.push_str(&format!(
                    "│ {}  └─ Offset: 0x{:x}, Length: {} bytes\n",
                    indent, m.offset, m.length
                ));

                // Add separator between fields
                output.push_str("│ \n");

                // Move to the next field
                current_offset += m.length;
            } else {
                // Skip unrecognized bytes
                current_offset += 1;
            }
        }

        output.push_str("└────────────────────────────────────────────────────────┘\n\n");

        // Add the structure hypothesis if available
        if let Some(hypothesis) = &results.structure_hypothesis {
            output.push_str("Structure Hypothesis:\n");
            output.push_str("┌────────────────────────────────────────────────────────┐\n");
            for line in hypothesis.lines() {
                output.push_str(&format!("│ {}\n", line));
            }
            output.push_str("└────────────────────────────────────────────────────────┘\n");
        }

        output
    }
}

/// Helper to determine the nesting level of each match based on containing relationships
fn determine_nesting_levels(matches: &[PatternMatch]) -> HashMap<usize, usize> {
    let mut levels = HashMap::new();

    // First pass: determine potential parent-child relationships
    for (i, m) in matches.iter().enumerate() {
        let mut level = 0;

        // Check if this match is contained within any previous match
        for potential_parent in matches.iter().take(i) {
            let parent_end = potential_parent.offset + potential_parent.length;

            // If this match is fully contained within a potential parent
            if m.offset >= potential_parent.offset
                && m.offset + m.length <= parent_end
                && m.offset > potential_parent.offset
            {
                // Not the same offset
                let parent_level = *levels.get(&potential_parent.offset).unwrap_or(&0);
                level = level.max(parent_level + 1);
            }
        }

        levels.insert(m.offset, level);
    }

    levels
}

/// Helper to format bytes as a hex string with limited length
fn format_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ")
}