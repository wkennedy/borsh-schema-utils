// src/visualization/html.rs

//! HTML-based visualization of Borsh analysis results

use super::{ColorTheme, VisualizationOptions, Visualizer};
use crate::core::{AnalysisResult, PatternMatch};
use std::collections::HashMap;

/// HTML-based visualizer
pub struct HtmlVisualizer;

impl Visualizer for HtmlVisualizer {
    fn render(
        &self,
        results: &AnalysisResult,
        data: &[u8],
        options: &VisualizationOptions,
    ) -> String {
        let mut output = String::new();

        // Start HTML document
        output.push_str("<!DOCTYPE html>\n");
        output.push_str("<html lang=\"en\">\n");
        output.push_str("<head>\n");
        output.push_str("    <meta charset=\"UTF-8\">\n");
        output.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        output.push_str("    <title>Borsh Structure Visualization</title>\n");

        // Add CSS based on the selected theme
        output.push_str("    <style>\n");
        output.push_str(match options.theme {
            ColorTheme::Dark => include_str!("templates/dark.css"),
            ColorTheme::Light => include_str!("templates/light.css"),
            ColorTheme::HighContrast => include_str!("templates/high_contrast.css"),
        });
        output.push_str("    </style>\n");
        output.push_str("</head>\n");
        output.push_str("<body>\n");

        // Add header
        output.push_str("    <div class=\"header\">\n");
        output.push_str("        <h1>Borsh Structure Visualization</h1>\n");
        output.push_str(&format!(
            "        <div class=\"info\">Data Size: {} bytes | Confidence: {}%</div>\n",
            data.len(),
            results.confidence
        ));
        output.push_str(&format!(
            "        <div class=\"description\">{}</div>\n",
            results.description
        ));
        output.push_str("    </div>\n");

        // Start main content area
        output.push_str("    <div class=\"container\">\n");

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
        output.push_str("        <div class=\"structure-diagram\">\n");
        output.push_str("            <h2>Data Structure</h2>\n");
        output.push_str("            <div class=\"tree-container\">\n");

        let mut current_offset = 0;
        while current_offset < data.len() {
            if let Some(m) = offset_map.get(&current_offset) {
                // Found a match at the current offset
                let level = nesting_levels.get(&current_offset).unwrap_or(&0);
                let indent_px = level * 20; // 20px per level

                // Get a CSS class based on confidence
                let confidence_class = match m.confidence {
                    90..=100 => "high-confidence",
                    70..=89 => "good-confidence",
                    50..=69 => "medium-confidence",
                    30..=49 => "low-confidence",
                    _ => "very-low-confidence",
                };

                // Render field node
                output.push_str(&format!(
                    "                <div class=\"tree-node {}\" style=\"margin-left: {}px;\">\n",
                    confidence_class, indent_px
                ));

                // Render field name and type
                output.push_str(&format!(
                    "                    <div class=\"node-header\">{}</div>\n",
                    m.pattern_name
                ));

                // Render node content
                output.push_str("                    <div class=\"node-content\">\n");

                // Show interpretation if enabled
                if options.show_interpretations {
                    output.push_str(&format!(
                        "                        <div class=\"interpretation\">{}</div>\n",
                        html_escape(&m.interpretation)
                    ));
                }

                // Show confidence if enabled
                if options.show_confidence {
                    output.push_str(&format!(
                        "                        <div class=\"confidence\">Confidence: {}%</div>\n",
                        m.confidence
                    ));
                }

                // Show raw bytes if enabled
                if options.show_raw_bytes && !m.data.is_empty() {
                    let bytes_to_show = m.data.len().min(options.max_bytes_per_match);
                    let bytes_str = format_bytes_html(&m.data[..bytes_to_show]);
                    output.push_str(&format!(
                        "                        <div class=\"raw-bytes\">Bytes: {}</div>\n",
                        bytes_str
                    ));

                    if bytes_to_show < m.data.len() {
                        output.push_str(&format!("                        <div class=\"bytes-more\">... ({} more bytes)</div>\n",
                                                 m.data.len() - bytes_to_show));
                    }
                }

                // Add offset and length info
                output.push_str(&format!("                        <div class=\"offset-info\">Offset: 0x{:x}, Length: {} bytes</div>\n",
                                         m.offset, m.length));

                output.push_str("                    </div>\n"); // Close node-content
                output.push_str("                </div>\n"); // Close tree-node

                // Move to the next field
                current_offset += m.length;
            } else {
                // Skip unrecognized bytes
                current_offset += 1;
            }
        }

        output.push_str("            </div>\n"); // Close tree-container
        output.push_str("        </div>\n"); // Close structure-diagram

        // Add hex view
        output.push_str("        <div class=\"hex-view\">\n");
        output.push_str("            <h2>Hex View</h2>\n");
        output.push_str("            <div class=\"hex-container\">\n");
        render_hex_view(&mut output, data, &offset_map, &results.matches);
        output.push_str("            </div>\n"); // Close hex-container
        output.push_str("        </div>\n"); // Close hex-view

        // Add the structure hypothesis if available
        if let Some(hypothesis) = &results.structure_hypothesis {
            output.push_str("        <div class=\"structure-hypothesis\">\n");
            output.push_str("            <h2>Structure Hypothesis</h2>\n");
            output.push_str("            <pre><code>");
            output.push_str(&html_escape(hypothesis));
            output.push_str("</code></pre>\n");
            output.push_str("        </div>\n");
        }

        // Close main content area
        output.push_str("    </div>\n"); // Close container

        // Add interactive JavaScript
        output.push_str("    <script>\n");
        output.push_str("        document.addEventListener('DOMContentLoaded', function() {\n");
        output.push_str("            const nodes = document.querySelectorAll('.tree-node');\n");
        output.push_str("            nodes.forEach(node => {\n");
        output.push_str("                node.addEventListener('click', function() {\n");
        output.push_str("                    this.classList.toggle('expanded');\n");
        output.push_str("                });\n");
        output.push_str("            });\n");
        output.push_str("        });\n");
        output.push_str("    </script>\n");

        // End HTML document
        output.push_str("</body>\n");
        output.push_str("</html>\n");

        output
    }
}

/// Helper to render a hex view of the data with match highlighting
fn render_hex_view(
    output: &mut String,
    data: &[u8],
    _offset_map: &HashMap<usize, &PatternMatch>,
    matches: &[PatternMatch],
) {
    // Create a map of offsets to pattern matches for coloring
    let mut byte_matches = vec![None; data.len()];
    for m in matches {
        for i in 0..m.length {
            if m.offset + i < byte_matches.len() {
                byte_matches[m.offset + i] = Some(m);
            }
        }
    }

    const BYTES_PER_LINE: usize = 16;
    output.push_str("                <table class=\"hex-table\">\n");
    output.push_str("                    <tr><th>Offset</th><th colspan=\"8\">Hex (first half)</th><th colspan=\"8\">Hex (second half)</th><th>ASCII</th></tr>\n");

    for (line_idx, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
        let line_offset = line_idx * BYTES_PER_LINE;
        output.push_str(&format!(
            "                    <tr><td class=\"offset\">{:08x}</td>",
            line_offset
        ));

        // Render hex values
        for i in 0..16 {
            if i == 8 {
                // Separator between first and second half
                output.push_str("<td class=\"separator\"></td>");
            }

            if i < chunk.len() {
                let byte_idx = line_offset + i;
                let byte = chunk[i];

                // Determine CSS classes based on match confidence
                let css_class = match byte_matches[byte_idx] {
                    Some(m) if m.confidence >= 90 => "hex-byte high-confidence",
                    Some(m) if m.confidence >= 70 => "hex-byte good-confidence",
                    Some(m) if m.confidence >= 50 => "hex-byte medium-confidence",
                    Some(m) if m.confidence >= 30 => "hex-byte low-confidence",
                    Some(_) => "hex-byte very-low-confidence",
                    None => "hex-byte",
                };

                output.push_str(&format!("<td class=\"{}\">{:02x}</td>", css_class, byte));
            } else {
                output.push_str("<td></td>"); // Empty cell for padding
            }
        }

        // Render ASCII representation
        output.push_str("<td class=\"ascii\">");
        for (i, byte) in chunk.iter().enumerate() {
            let byte_idx = line_offset + i;
            // let byte = chunk[i];
            let char_to_print = if (32..=126).contains(byte) {
                *byte as char
            } else {
                '.'
            };

            // Determine CSS classes based on match confidence
            let css_class = match byte_matches[byte_idx] {
                Some(m) if m.confidence >= 90 => "high-confidence",
                Some(m) if m.confidence >= 70 => "good-confidence",
                Some(m) if m.confidence >= 50 => "medium-confidence",
                Some(m) if m.confidence >= 30 => "low-confidence",
                Some(_) => "very-low-confidence",
                None => "",
            };

            output.push_str(&format!(
                "<span class=\"{}\">{}</span>",
                css_class, char_to_print
            ));
        }
        output.push_str("</td></tr>\n");
    }

    output.push_str("                </table>\n");
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
fn format_bytes_html(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("<span class=\"byte\">{:02x}</span>", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Helper to escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
