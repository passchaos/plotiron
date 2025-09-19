//! Figure management and SVG generation

use crate::axes::Axes;
use crate::colors::Color;

/// Represents a figure that can contain multiple subplots
#[derive(Debug)]
pub struct Figure {
    pub width: f64,
    pub height: f64,
    pub dpi: f64,
    pub background_color: Color,
    pub subplots: Vec<Axes>,
    pub tight_layout: bool,
}

impl Figure {
    /// Create a new figure with default settings
    pub fn new() -> Self {
        Figure {
            width: 1200.0,
            height: 900.0,
            dpi: 100.0,
            background_color: Color::WHITE,
            subplots: Vec::new(),
            tight_layout: true,
        }
    }

    /// Create a new figure with specified dimensions
    pub fn with_size(width: f64, height: f64) -> Self {
        Figure {
            width,
            height,
            dpi: 100.0,
            background_color: Color::WHITE,
            subplots: Vec::new(),
            tight_layout: true,
        }
    }

    /// Set the figure size
    pub fn set_size(&mut self, width: f64, height: f64) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the DPI (dots per inch)
    pub fn set_dpi(&mut self, dpi: f64) -> &mut Self {
        self.dpi = dpi;
        self
    }

    /// Set the background color
    pub fn set_facecolor(&mut self, color: Color) -> &mut Self {
        self.background_color = color;
        self
    }

    /// Add a subplot and return a mutable reference to it
    pub fn add_subplot(&mut self) -> &mut Axes {
        let axes = Axes::new();
        self.subplots.push(axes);
        self.subplots.last_mut().unwrap()
    }

    /// Add a subplot with DOT graph content
    pub fn add_dot_subplot(&mut self, dot_content: &str) -> Result<&mut Axes, String> {
        self.add_dot_subplot_with_layout(dot_content, crate::dot::LayoutAlgorithm::Hierarchical)
    }

    /// Add a DOT subplot with specified layout algorithm
    pub fn add_dot_subplot_with_layout(
        &mut self,
        dot_content: &str,
        layout: crate::dot::LayoutAlgorithm,
    ) -> Result<&mut Axes, String> {
        let axes = self.add_subplot();

        // Parse DOT content using the advanced renderer
        let mut dot_graph = crate::dot::DotGraph::parse_dot(dot_content)?;
        dot_graph.set_layout(layout);
        dot_graph.apply_layout();

        // Render the graph to the axes
        dot_graph.render_to_axes(axes);

        Ok(axes)
    }

    /// Get a mutable reference to a subplot by index
    pub fn subplot(&mut self, index: usize) -> Option<&mut Axes> {
        self.subplots.get_mut(index)
    }

    /// Generate SVG string for the entire figure
    pub fn to_svg(&self) -> String {
        let mut svg = String::new();

        // SVG header
        svg.push_str(&format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
            self.width, self.height
        ));

        // Background
        svg.push_str(&format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
            self.width,
            self.height,
            self.background_color.to_svg_string()
        ));

        // Render subplots
        if self.subplots.len() == 1 {
            // Single subplot takes the full figure
            svg.push_str(&self.subplots[0].to_svg(self.width, self.height));
        } else if !self.subplots.is_empty() {
            // Multiple subplots - simple grid layout
            let cols = (self.subplots.len() as f64).sqrt().ceil() as usize;
            let rows = (self.subplots.len() + cols - 1) / cols;

            let subplot_width = self.width / cols as f64;
            let subplot_height = self.height / rows as f64;

            for (i, subplot) in self.subplots.iter().enumerate() {
                let col = i % cols;
                let row = i / cols;
                let x = col as f64 * subplot_width;
                let y = row as f64 * subplot_height;

                svg.push_str(&format!("<g transform=\"translate({},{})\">\n", x, y));
                svg.push_str(&subplot.to_svg(subplot_width, subplot_height));
                svg.push_str("</g>\n");
            }
        }

        svg.push_str("</svg>");
        svg
    }

    /// Display the figure (prints SVG to stdout for now)
    pub fn show(&self) {
        let svg = self.to_svg();
        crate::viewer::show_svg(svg);
    }

    /// Clear all subplots
    pub fn clear(&mut self) {
        self.subplots.clear();
    }

    /// Set tight layout
    pub fn tight_layout(&mut self, enable: bool) -> &mut Self {
        self.tight_layout = enable;
        self
    }

    /// Create a figure from DOT markup language
    pub fn from_dot(dot_content: &str) -> Result<Self, String> {
        let mut figure = Figure::new();
        let axes = figure.add_subplot();

        // Parse DOT content
        let lines: Vec<&str> = dot_content.lines().collect();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for line in lines {
            let line = line.trim();
            if line.is_empty()
                || line.starts_with("//")
                || line.starts_with("digraph")
                || line.starts_with("graph")
                || line == "{"
                || line == "}"
            {
                continue;
            }

            if line.contains("->") {
                // Edge definition
                let parts: Vec<&str> = line.split("->").collect();
                if parts.len() == 2 {
                    let from = parts[0].trim().trim_matches('"');
                    let to = parts[1].trim().trim_end_matches(';').trim_matches('"');
                    edges.push((from.to_string(), to.to_string()));
                }
            } else if line.contains("--") {
                // Undirected edge
                let parts: Vec<&str> = line.split("--").collect();
                if parts.len() == 2 {
                    let from = parts[0].trim().trim_matches('"');
                    let to = parts[1].trim().trim_end_matches(';').trim_matches('"');
                    edges.push((from.to_string(), to.to_string()));
                }
            } else if line.contains('[') && line.contains(']') {
                // Node with attributes
                let node_name = line.split('[').next().unwrap().trim().trim_matches('"');
                if !node_name.is_empty() {
                    nodes.push(node_name.to_string());
                }
            } else if line.ends_with(';') {
                // Simple node definition
                let node_name = line.trim_end_matches(';').trim().trim_matches('"');
                if !node_name.is_empty() {
                    nodes.push(node_name.to_string());
                }
            }
        }

        // Collect all unique nodes from edges
        for (from, to) in &edges {
            if !nodes.contains(from) {
                nodes.push(from.clone());
            }
            if !nodes.contains(to) {
                nodes.push(to.clone());
            }
        }

        if nodes.is_empty() {
            return Err("No nodes found in DOT content".to_string());
        }

        // Create a simple layout for nodes
        let node_count = nodes.len();
        let mut x_coords = Vec::new();
        let mut y_coords = Vec::new();

        if node_count == 1 {
            x_coords.push(0.5);
            y_coords.push(0.5);
        } else {
            // Arrange nodes in a circle
            for i in 0..node_count {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / node_count as f64;
                let x = 0.5 + 0.3 * angle.cos();
                let y = 0.5 + 0.3 * angle.sin();
                x_coords.push(x);
                y_coords.push(y);
            }
        }

        // Plot nodes as scatter points
        axes.scatter(x_coords.as_slice(), y_coords.as_slice());
        if let Some(last_plot) = axes.plots.last_mut() {
            last_plot.marker = crate::markers::Marker::Circle;
            last_plot.marker_size = 10.0;
            last_plot.color = crate::colors::Color::BLUE;
        }

        // Draw edges as lines
        for (from, to) in edges {
            if let (Some(from_idx), Some(to_idx)) = (
                nodes.iter().position(|n| n == &from),
                nodes.iter().position(|n| n == &to),
            ) {
                let x_line = vec![x_coords[from_idx], x_coords[to_idx]];
                let y_line = vec![y_coords[from_idx], y_coords[to_idx]];
                axes.plot(x_line, y_line);
                if let Some(last_plot) = axes.plots.last_mut() {
                    last_plot.color = crate::colors::Color::BLACK;
                    last_plot.line_width = 1.0;
                }
            }
        }

        axes.set_title("Graph from DOT");
        axes.set_xlabel("X");
        axes.set_ylabel("Y");

        Ok(figure)
    }
}

impl Default for Figure {
    fn default() -> Self {
        Self::new()
    }
}
