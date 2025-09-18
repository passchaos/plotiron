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

### Histogram

```rust
// Generate sample data
let data: Vec<f64> = (0..1000).map(|i| {
    let x = (i as f64 - 500.0) / 100.0;
    x + (rand::random::<f64>() - 0.5) * 2.0
}).collect();

let mut fig = figure();
fig.add_subplot()
    .histogram(&data, 20)  // 20 bins
    .set_title("Data Distribution")
    .set_xlabel("Value")
    .set_ylabel("Frequency");
std::fs::write("histogram.svg", fig.to_svg()).unwrap();
```

### Pie Chart

```rust
use plotiron::*;

let values = vec![30.0, 25.0, 20.0, 15.0, 10.0];
let labels = vec![
    "Product A".to_string(),
    "Product B".to_string(),
    "Product C".to_string(),
    "Product D".to_string(),
    "Product E".to_string()
];

let mut fig = figure();
fig.add_subplot()
    .pie(&values, Some(&labels))
    .set_title("Market Share Distribution");

std::fs::write("pie_chart.svg", fig.to_svg()).expect("Failed to write file");
```

### Box Plot

```rust
use plotiron::*;

let data = vec![
    12.5, 14.2, 15.8, 16.1, 17.3, 18.9, 19.2, 20.1, 21.5, 22.8,
    23.1, 24.7, 25.3, 26.9, 27.2, 28.5, 29.1, 30.8, 31.2, 32.5,
    // Add some outliers
    8.0, 38.5, 42.0
];

let mut fig = figure();
fig.add_subplot()
    .boxplot(&data)
    .set_title("Performance Distribution with Outliers")
    .set_ylabel("Score");

std::fs::write("boxplot.svg", fig.to_svg()).expect("Failed to write file");
```

### Violin Plot

```rust
use plotiron::*;

let data = vec![
    12.5, 14.2, 15.8, 16.1, 17.3, 18.9, 19.2, 20.1, 21.5, 22.8,
    23.1, 24.7, 25.3, 26.9, 27.2, 28.5, 29.1, 30.8, 31.2, 32.5,
    33.1, 34.2, 35.5, 36.8, 37.1, 38.4, 39.7, 40.2, 41.5, 42.8,
    // Add some variation for interesting distribution
    15.5, 18.2, 22.1, 25.8, 29.3, 33.7, 37.4, 41.1, 44.6, 48.2
];

let mut fig = figure();
fig.add_subplot()
    .violin(&data)
    .set_title("Data Distribution - Violin Plot")
    .set_ylabel("Values")
    .grid(true);

std::fs::write("violin.svg", fig.to_svg()).expect("Failed to write file");
```

### Contour Plot

```rust
// Create sample data for contour plot
let x: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
let y: Vec<f64> = (0..15).map(|i| i as f64 * 0.4).collect();

// Create a 2D function: z = sin(x) * cos(y)
let mut z: Vec<Vec<f64>> = Vec::new();
for &yi in &y {
    let mut row = Vec::new();
    for &xi in &x {
        let zi = (xi * 0.5).sin() * (yi * 0.3).cos() + 0.2 * (xi * yi * 0.1).sin();
        row.push(zi);
    }
    z.push(row);
}

let mut fig = figure();
fig.add_subplot()
    .contour(&x, &y, &z)
    .set_title("Mathematical Function - Contour Plot")
    .set_xlabel("X values")
    .set_ylabel("Y values");

std::fs::write("contour.svg", fig.to_svg()).expect("Failed to write file");
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
