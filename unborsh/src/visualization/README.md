# Borsh Structure Visualization

The visualization module of unborsh allows you to generate interactive visualizations of Borsh-serialized data structures. This makes it easier to understand and explore the structure of binary data in a visual format.

## Features

- **Multiple Formats**: Generate visualizations in ASCII (for terminal display) or HTML (for interactive viewing)
- **Hierarchical View**: Visualize nested structures with proper indentation
- **Hex View**: See the raw bytes with highlighting based on confidence
- **Customizable Themes**: Choose between light, dark, and high-contrast themes
- **Interactive Elements**: Collapsible nodes in HTML visualization for easier exploration
- **Confidence Indicators**: Visual indicators of analysis confidence levels
- **Structure Hypothesis**: Visual representation of the inferred data structure

## Usage

### Basic Visualization

```rust
use unborsh::{
    analyze, VisualizationOptions, VisualizationFormat, ColorTheme, visualize
};

// Some Borsh-serialized data
let data = vec![11, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];

// Analyze the data
let result = analyze(&data);

// Create visualization options
let viz_options = VisualizationOptions {
    max_depth: 5,
    show_interpretations: true,
    show_confidence: true,
    show_raw_bytes: true,
    max_bytes_per_match: 32,
    theme: ColorTheme::Dark,
};

// Generate ASCII visualization for terminal display
let ascii_viz = visualize(&result, &data, VisualizationFormat::Ascii, &viz_options);
println!("{}", ascii_viz);

// Generate HTML visualization and save to file
unborsh::visualize_to_file(
    &result,
    &data,
    VisualizationFormat::Html,
    "visualization.html",
    &viz_options,
).expect("Failed to save visualization");
```

### Using the CLI

The CLI interface also supports generating visualizations:

```bash
# Generate an HTML visualization and save to file
unborsh visualize data.bin --format html --output visualization.html --theme dark

# Generate an ASCII visualization and print to terminal
unborsh visualize data.bin --format ascii
```

## Visualization Options

The `VisualizationOptions` struct allows customization of the visualization:

```rust
let options = VisualizationOptions {
    // Maximum nesting depth to visualize
    max_depth: 5,
    
    // Whether to show interpretations of matches
    show_interpretations: true,
    
    // Whether to show confidence scores
    show_confidence: true,
    
    // Whether to show raw bytes in hex
    show_raw_bytes: true,
    
    // Maximum number of bytes to show per match
    max_bytes_per_match: 32,
    
    // Color theme to use
    theme: ColorTheme::Dark,
};
```

### Available Themes

- `ColorTheme::Light` - Light background with dark text (good for printing)
- `ColorTheme::Dark` - Dark background with light text (easier on the eyes)
- `ColorTheme::HighContrast` - High contrast theme for accessibility

## Visualization Formats

### ASCII Format

ASCII format generates a text-based visualization suitable for terminal display. It includes:

- Structure diagram with indentation for nested fields
- Confidence indicators with star ratings
- Raw bytes in hex format
- Structure hypothesis in plain text

### HTML Format

HTML format generates an interactive web page that allows exploration of complex structures:

- Collapsible tree view of the data structure
- Color-coded confidence levels
- Interactive hex view with highlighted matches
- Syntax-highlighted structure hypothesis
- Responsive design that works on different screen sizes

## Examples

See the `examples/visualization_example.rs` file for a complete example of generating visualizations for different Borsh-serialized data structures.

## Customizing the Visualization

You can create custom visualizers by implementing the `Visualizer` trait:

```rust
pub trait Visualizer {
    fn render(&self, results: &AnalysisResult, data: &[u8], options: &VisualizationOptions) -> String;
    
    fn render_to<W: Write>(&self, results: &AnalysisResult, data: &[u8], writer: &mut W, options: &VisualizationOptions) -> std::io::Result<()>;
}
```

This allows you to create visualizations in formats beyond the built-in ASCII and HTML formats, such as SVG, PDF, or other formats suited to your needs.