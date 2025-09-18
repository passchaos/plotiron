# PlotIron - Rust Plotting Library

A Rust plotting library inspired by matplotlib for creating 2D charts and visualizations.

## Features

- 🎨 **Multiple Chart Types**: Line plots, scatter plots, bar charts, and more
- 🎯 **Easy to Use**: matplotlib-like API design
- 📊 **SVG Output**: Generate high-quality vector graphics
- 🎪 **Custom Styling**: Support for colors, markers, line styles, and more
- 📈 **Multiple Subplots**: Create multiple subplots in a single figure
- 🔧 **Flexible Configuration**: Configurable axes, grids, legends, and more

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
plotiron = { path = "." }
```

### Basic Usage

```rust
use plotiron::*;

fn main() {
    // Create data
    let x: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
    let y: Vec<f64> = x.iter().map(|&x| x.sin()).collect();

    // Create figure
    let mut fig = figure();
    fig.add_subplot()
        .plot(&x, &y)
        .set_title("Sine Function")
        .set_xlabel("x")
        .set_ylabel("sin(x)");
    
    // Save as SVG file
    std::fs::write("plot.svg", fig.to_svg()).unwrap();
}
```

## Examples

### Line Plot

```rust
let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let y = vec![1.0, 4.0, 2.0, 3.0, 5.0];

let mut fig = figure();
fig.add_subplot()
    .plot(&x, &y)
    .set_title("Simple Line Plot");
std::fs::write("line_plot.svg", fig.to_svg()).unwrap();
```

### Scatter Plot

```rust
let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let y = vec![2.0, 5.0, 3.0, 8.0, 7.0];

let mut fig = figure();
fig.add_subplot()
    .scatter(&x, &y)
    .set_title("Scatter Plot");
std::fs::write("scatter_plot.svg", fig.to_svg()).unwrap();
```

### Bar Chart

```rust
let categories = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let values = vec![23.0, 45.0, 56.0, 78.0, 32.0];

let mut fig = figure();
fig.add_subplot()
    .bar(&categories, &values)
    .set_title("Bar Chart");
std::fs::write("bar_plot.svg", fig.to_svg()).unwrap();
```

### Multiple Line Plot

```rust
let x: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
let y1: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
let y2: Vec<f64> = x.iter().map(|&x| x.cos()).collect();

let mut fig = figure();
fig.add_subplot()
    .plot(&x, &y1)
    .plot(&x, &y2)
    .set_title("Sine and Cosine Functions")
    .legend(true);
std::fs::write("multi_line.svg", fig.to_svg()).unwrap();
```

### Multiple Subplots

```rust
let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
let y1: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
let y2: Vec<f64> = x.iter().map(|&x| x.cos()).collect();
let scatter_x: Vec<f64> = (0..20).map(|_| rand::random::<f64>() * 10.0).collect();
let scatter_y: Vec<f64> = scatter_x.iter().map(|&x| x * 0.5 + rand::random::<f64>() * 2.0).collect();

let mut fig = figure_with_size(1200.0, 800.0);

// First subplot: sine wave
fig.add_subplot()
    .plot(&x, &y1)
    .set_title("Sine Wave")
    .set_xlabel("x")
    .set_ylabel("sin(x)");

// Second subplot: cosine wave
fig.add_subplot()
    .plot(&x, &y2)
    .set_title("Cosine Wave")
    .set_xlabel("x")
    .set_ylabel("cos(x)");

// Third subplot: scatter plot
fig.add_subplot()
    .scatter(&scatter_x, &scatter_y)
    .set_title("Random Scatter Plot")
    .set_xlabel("x")
    .set_ylabel("y");

std::fs::write("multiple_subplots.svg", fig.to_svg()).unwrap();
```

### DOT Graph Example

```rust
let dot_content = r#"
digraph G {
    rankdir=TB;
    A [label="Start"];
    B [label="Process"];
    C [label="Decision"];
    D [label="End"];
    
    A -> B;
    B -> C;
    C -> D [label="Yes"];
    C -> B [label="No"];
}
"#;

let mut fig = figure();
fig.add_dot_subplot(dot_content).unwrap()
    .set_title("Workflow Diagram");

std::fs::write("workflow_diagram.svg", fig.to_svg()).unwrap();
```

### Mixed Subplots: Scatter Plot and DOT Graph

```rust
// Create scatter plot data
let scatter_x: Vec<f64> = (0..30).map(|_| rand::random::<f64>() * 10.0).collect();
let scatter_y: Vec<f64> = scatter_x.iter().map(|&x| x * 1.5 + rand::random::<f64>() * 3.0).collect();

// DOT graph content
let dot_content = r#"
digraph Network {
    rankdir=LR;
    node [shape=circle];
    
    A -> B -> C;
    A -> D -> C;
    B -> E;
    D -> E;
    E -> F;
    C -> F;
}
"#;

let mut fig = figure_with_size(1200.0, 600.0);

// First subplot: scatter plot
fig.add_subplot()
    .scatter(&scatter_x, &scatter_y)
    .set_title("Data Distribution")
    .set_xlabel("Input Values")
    .set_ylabel("Output Values")
    .grid(true);

// Second subplot: DOT graph
fig.add_dot_subplot(dot_content).unwrap()
    .set_title("Network Topology")
    .show_x_axis(false)
    .show_y_axis(false);

std::fs::write("mixed_subplots.svg", fig.to_svg()).unwrap();
```

## API Reference

### Figure

- `figure()` - Create a figure with default size
- `figure_with_size(width, height)` - Create a figure with specified size
- `add_subplot()` - Add a subplot
- `add_dot_subplot(dot_content)` - Add a subplot with DOT graph
- `add_dot_subplot_with_layout(dot_content, layout)` - Add DOT subplot with specific layout
- `to_svg()` - Generate SVG string
- `show()` - Display figure (print SVG to console)

### Axes

- `plot(x, y)` - Add line plot
- `scatter(x, y)` - Add scatter plot
- `bar(x, y)` - Add bar chart
- `set_title(title)` - Set title
- `set_xlabel(label)` - Set X-axis label
- `set_ylabel(label)` - Set Y-axis label
- `set_xlim(min, max)` - Set X-axis range
- `set_ylim(min, max)` - Set Y-axis range
- `grid(enable)` - Enable/disable grid
- `legend(enable)` - Enable/disable legend
- `show_x_axis(enable)` - Show/hide X-axis
- `show_y_axis(enable)` - Show/hide Y-axis

### DOT Layout Algorithms

- `LayoutAlgorithm::Circular` - Arrange nodes in a circle
- `LayoutAlgorithm::Hierarchical` - Hierarchical top-down layout
- `LayoutAlgorithm::ForceDirected` - Force-directed layout
- `LayoutAlgorithm::Grid` - Grid-based layout

### Colors

Supported predefined colors:
- `Color::RED`, `Color::BLUE`, `Color::GREEN`, etc.
- Hex colors: `Color::from_hex("#FF0000")`
- String colors: `Color::from("red")`

### Marker Styles

- `Marker::Circle` - Circle
- `Marker::Square` - Square
- `Marker::TriangleUp` - Upward triangle
- `Marker::Diamond` - Diamond
- `Marker::Plus` - Plus sign
- `Marker::Cross` - Cross
- `Marker::Star` - Star

## Saving Plots

To save plots as SVG files, use the `to_svg()` method combined with `std::fs::write()`:

```rust
let mut fig = figure();
fig.add_subplot()
    .plot(&x, &y)
    .set_title("My Plot");

// Save to file
std::fs::write("my_plot.svg", fig.to_svg()).unwrap();
```

## Running Examples

```bash
cargo run --example demo
```

This will generate several example SVG files in the `output/` directory:
- `sine_plot.svg` - Sine function plot
- `multi_line_plot.svg` - Multi-line plot
- `scatter_plot.svg` - Scatter plot
- `bar_plot.svg` - Bar chart

## Architecture

```
src/
├── lib.rs          # Main library entry point
├── main.rs         # Example program
├── figure.rs       # Figure management
├── axes.rs         # Axes functionality
├── plot.rs         # Plot types
├── colors.rs       # Color definitions
├── markers.rs      # Marker styles
└── utils.rs        # Utility functions
```

## Dependencies

- `svg` - SVG generation
- `num-traits` - Numeric traits
- `rand` - Random number generation (examples only)

## License

MIT License

## Contributing

Issues and Pull Requests are welcome!

## TODO

- [ ] More chart types (histograms, pie charts, etc.)
- [ ] Animation support
- [ ] Interactive charts
- [ ] 3D chart support
- [ ] Better error handling
- [ ] Performance optimization
