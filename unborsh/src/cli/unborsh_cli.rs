// src/bin/unborsh-cli.rs

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use unborsh::{AnalysisOptions, AnalysisResult, AnalysisStrategy, analyze_with_options, interpret_as, pattern_definitions, VisualizationFormat, ColorTheme, VisualizationOptions, visualize_to_file, visualize};

#[derive(Parser)]
#[command(name = "unborsh")]
#[command(about = "Borsh binary format analyzer", long_about = None)]
#[command(version, author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze Borsh-serialized data
    Analyze {
        /// Input source (file path, hex string, or - for stdin)
        input: String,

        /// Analysis strategy to use
        #[arg(short, long, value_enum, default_value_t = StrategyArg::Comprehensive)]
        strategy: StrategyArg,

        /// Maximum recursion depth
        #[arg(short, long, default_value_t = 5)]
        depth: usize,

        /// Minimum confidence level (0-100)
        #[arg(short, long, default_value_t = 30)]
        confidence: u8,

        /// Include raw bytes in output
        #[arg(short, long)]
        raw_bytes: bool,

        /// Output format
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,

        /// Max number of matches to show
        #[arg(short, long)]
        max_matches: Option<usize>,
    },

    /// Check if data matches a specific pattern
    Match {
        /// Input source (file path, hex string, or - for stdin)
        input: String,

        /// Pattern name to match against
        pattern: String,
    },

    /// List available patterns
    Patterns {
        /// Filter patterns by category
        #[arg(short, long, value_enum)]
        category: Option<PatternCategory>,

        /// Show detailed descriptions
        #[arg(short, long)]
        verbose: bool,
    },

    // Add this to the Commands enum
    /// Visualize Borsh-serialized data
    Visualize {
        /// Input source (file path, hex string, or - for stdin)
        input: String,

        /// Analysis strategy to use
        #[arg(short, long, value_enum, default_value_t = StrategyArg::Comprehensive)]
        strategy: StrategyArg,

        /// Maximum recursion depth
        #[arg(short, long, default_value_t = 5)]
        depth: usize,

        /// Minimum confidence level (0-100)
        #[arg(short, long, default_value_t = 30)]
        confidence: u8,

        /// Include raw bytes in output
        #[arg(short, long)]
        raw_bytes: bool,

        /// Visualization format
        #[arg(short, long, value_enum, default_value_t = VisualizationFormatArg::Html)]
        format: VisualizationFormatArg,

        /// Color theme to use
        #[arg(short, long, value_enum, default_value_t = ColorThemeArg::Dark)]
        theme: ColorThemeArg,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug)]
enum StrategyArg {
    Pattern,
    Recursive,
    Probabilistic,
    Comprehensive,
}

impl From<StrategyArg> for AnalysisStrategy {
    fn from(arg: StrategyArg) -> Self {
        match arg {
            StrategyArg::Pattern => AnalysisStrategy::Pattern,
            StrategyArg::Recursive => AnalysisStrategy::Recursive,
            StrategyArg::Probabilistic => AnalysisStrategy::Probabilistic,
            StrategyArg::Comprehensive => AnalysisStrategy::Comprehensive,
        }
    }
}

#[derive(ValueEnum, Copy, Clone, Debug)]
enum OutputFormat {
    Text,
    Json,
    Hex,
    Compact,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
enum PatternCategory {
    Primitives,
    Collections,
    Complex,
    Blockchain,
    All,
}


// Define visualization format argument enum
#[derive(ValueEnum, Copy, Clone, Debug)]
enum VisualizationFormatArg {
    Ascii,
    Html,
}

impl From<VisualizationFormatArg> for VisualizationFormat {
    fn from(arg: VisualizationFormatArg) -> Self {
        match arg {
            VisualizationFormatArg::Ascii => VisualizationFormat::Ascii,
            VisualizationFormatArg::Html => VisualizationFormat::Html,
        }
    }
}

// Define color theme argument enum
#[derive(ValueEnum, Copy, Clone, Debug)]
enum ColorThemeArg {
    Light,
    Dark,
    HighContrast,
}

impl From<ColorThemeArg> for ColorTheme {
    fn from(arg: ColorThemeArg) -> Self {
        match arg {
            ColorThemeArg::Light => ColorTheme::Light,
            ColorThemeArg::Dark => ColorTheme::Dark,
            ColorThemeArg::HighContrast => ColorTheme::HighContrast,
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            input,
            strategy,
            depth,
            confidence,
            raw_bytes,
            format,
            max_matches,
        } => {
            let data = read_input(&input)?;
            let options = AnalysisOptions {
                strategy: strategy.into(),
                max_depth: depth,
                min_confidence: confidence,
                include_raw_bytes: raw_bytes,
                max_matches,
            };

            let result = analyze_with_options(&data, options);

            match format {
                OutputFormat::Text => print_text_output(&result),
                OutputFormat::Json => print_json_output(&result),
                OutputFormat::Hex => print_hex_output(&data, &result),
                OutputFormat::Compact => print_compact_output(&result),
            }
        }
        Commands::Match { input, pattern } => {
            let data = read_input(&input)?;
            if let Some(interpretation) = interpret_as(&data, &pattern) {
                println!("{}:\n  {}", "Pattern match found".green(), interpretation);
            } else {
                println!("{}", "No match found".red());
            }
        }
        Commands::Patterns { category, verbose } => {
            let dictionary = match category {
                Some(PatternCategory::Primitives) => pattern_definitions::create_primitive_dictionary(),
                Some(PatternCategory::Collections) => pattern_definitions::create_collection_dictionary(),
                Some(PatternCategory::Complex) => pattern_definitions::create_complex_dictionary(),
                Some(PatternCategory::Blockchain) => pattern_definitions::create_blockchain_dictionary(),
                Some(PatternCategory::All) | None => pattern_definitions::create_pattern_dictionary(),
            };

            println!("{} {} patterns available:",
                     dictionary.len(),
                     category.map_or("Total".to_string(), |c| format!("{:?}", c)));

            for (i, pattern) in dictionary.get_patterns().iter().enumerate() {
                if verbose {
                    println!("\n{}. {} - {}", i + 1, pattern.name.blue().bold(), pattern.description);
                    println!("   Example: {}", pattern.example);
                } else {
                    println!("{}. {}", i + 1, pattern.name);
                }
            }
        },
        Commands::Visualize {
            input,
            strategy,
            depth,
            confidence,
            raw_bytes,
            format,
            theme,
            output,
        } => {
            let data = read_input(&input)?;
            let options = AnalysisOptions {
                strategy: strategy.into(),
                max_depth: depth,
                min_confidence: confidence,
                include_raw_bytes: raw_bytes,
                max_matches: None,
            };

            let viz_options = VisualizationOptions {
                max_depth: depth,
                show_interpretations: true,
                show_confidence: true,
                show_raw_bytes: raw_bytes,
                max_bytes_per_match: 32,
                theme: theme.into(),
            };

            let result = analyze_with_options(&data, options);
            let viz_format = format.into();

            if let Some(output_path) = output {
                match visualize_to_file(&result, &data, viz_format, &output_path, &viz_options) {
                    Ok(_) => println!("Visualization saved to {}", output_path),
                    Err(e) => println!("Error saving visualization: {}", e),
                }
            } else {
                // Print to stdout
                let visualization = visualize(&result, &data, viz_format, &viz_options);
                println!("{}", visualization);
            }
        },
    }

    Ok(())
}

fn read_input(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if input == "-" {
        // Read from stdin
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        Ok(buffer)
    } else if input.starts_with("0x") || (input.len() > 4 && input.chars().all(|c| c.is_ascii_hexdigit())) {
        // Handle hex input
        let hex_str = if input.starts_with("0x") {
            &input[2..]
        } else {
            input
        };
        Ok(hex::decode(hex_str)?)
    } else {
        // Assume it's a file path
        let mut file = File::open(PathBuf::from(input))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

fn print_text_output(result: &AnalysisResult) {
    println!("{} ({}% confidence):", "Analysis Results".green().bold(), result.confidence);
    println!("{}", result.description);

    if !result.matches.is_empty() {
        println!("\n{}:", "Matches Found".blue().bold());
        for (i, m) in result.matches.iter().enumerate() {
            println!("  {}. {} at offset {} ({} bytes, {}% confidence)",
                     (i + 1).to_string().yellow(),
                     m.pattern_name.green(),
                     m.offset,
                     m.length,
                     m.confidence);
            println!("     {}", m.interpretation);
        }
    }

    if let Some(hypothesis) = &result.structure_hypothesis {
        println!("\n{}:", "Structure Hypothesis".blue().bold());
        for line in hypothesis.lines() {
            println!("  {}", line);
        }
    }
}

fn print_json_output(result: &AnalysisResult) {
    let json = serde_json::to_string_pretty(result).unwrap_or_else(|_| {
        serde_json::json!({
            "error": "Failed to serialize result to JSON"
        }).to_string()
    });
    println!("{}", json);
}

fn print_hex_output(data: &[u8], result: &AnalysisResult) {
    println!("{} ({}% confidence):", "Analysis Results".green().bold(), result.confidence);

    // Create a vector to track which bytes are matched
    let mut byte_matches = vec![None; data.len()];

    // Fill in match information
    for m in &result.matches {
        for i in 0..m.length {
            if m.offset + i < byte_matches.len() {
                byte_matches[m.offset + i] = Some(m);
            }
        }
    }

    // Print the hex view with highlights
    const BYTES_PER_LINE: usize = 16;

    for (line_idx, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
        // Print offset
        print!("{:08x}  ", line_idx * BYTES_PER_LINE);

        // Print hex values
        for (i, &byte) in chunk.iter().enumerate() {
            let byte_idx = line_idx * BYTES_PER_LINE + i;
            let byte_str = format!("{:02x} ", byte);

            match byte_matches[byte_idx] {
                Some(m) if m.confidence >= 70 => print!("{}", byte_str.green()),
                Some(m) if m.confidence >= 50 => print!("{}", byte_str.yellow()),
                Some(_) => print!("{}", byte_str.red()),
                None => print!("{}", byte_str),
            }
        }

        // Padding for alignment if line is shorter than BYTES_PER_LINE
        for _ in chunk.len()..BYTES_PER_LINE {
            print!("   ");
        }

        // Print ASCII representation
        print!(" |");
        for (i, &byte) in chunk.iter().enumerate() {
            let byte_idx = line_idx * BYTES_PER_LINE + i;
            let char_to_print = if byte >= 32 && byte <= 126 {
                byte as char
            } else {
                '.'
            };

            match byte_matches[byte_idx] {
                Some(m) if m.confidence >= 70 => print!("{}", char_to_print.to_string().green()),
                Some(m) if m.confidence >= 50 => print!("{}", char_to_print.to_string().yellow()),
                Some(_) => print!("{}", char_to_print.to_string().red()),
                None => print!("{}", char_to_print),
            }
        }
        println!("|");
    }

    // Print matches
    if !result.matches.is_empty() {
        println!("\n{}:", "Identified Patterns".blue().bold());
        for (i, m) in result.matches.iter().enumerate() {
            println!("  {}. {} at 0x{:x}..0x{:x} ({}% confidence)",
                     (i + 1).to_string().yellow(),
                     m.pattern_name.green(),
                     m.offset,
                     m.offset + m.length,
                     m.confidence);
            println!("     {}", m.interpretation);
        }
    }
}

fn print_compact_output(result: &AnalysisResult) {
    println!("Confidence: {}%", result.confidence);

    if !result.matches.is_empty() {
        for m in &result.matches {
            println!("{}:{:x}:{}:{}:{}%",
                     m.pattern_name,
                     m.offset,
                     m.length,
                     m.interpretation.replace(':', "\\:"),
                     m.confidence);
        }
    }

    if let Some(hypothesis) = &result.structure_hypothesis {
        let simplified = hypothesis
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join(" ");
        println!("Structure: {}", simplified);
    }
}