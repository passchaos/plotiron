//! Plot types and plotting functionality

use crate::colors::{Color, get_cycle_color};
use crate::markers::Marker;
use crate::utils::map_range;

/// Different types of plots
#[derive(Debug, Clone, PartialEq)]
pub enum PlotType {
    /// Line plot
    Line,
    /// Scatter plot
    Scatter,
    /// Bar plot
    Bar,
    /// Histogram
    Histogram,
}

/// A single plot/series of data
#[derive(Debug, Clone)]
pub struct Plot {
    pub x_data: Vec<f64>,
    pub y_data: Vec<f64>,
    pub plot_type: PlotType,
    pub color: Color,
    pub marker: Marker,
    pub marker_size: f64,
    pub line_width: f64,
    pub label: Option<String>,
    pub alpha: f64,
}

impl Plot {
    /// Create a new line plot
    pub fn line(x: &[f64], y: &[f64]) -> Self {
        Plot {
            x_data: x.to_vec(),
            y_data: y.to_vec(),
            plot_type: PlotType::Line,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 6.0,
            line_width: 1.5,
            label: None,
            alpha: 1.0,
        }
    }

    /// Create a new scatter plot
    pub fn scatter(x: &[f64], y: &[f64]) -> Self {
        Plot {
            x_data: x.to_vec(),
            y_data: y.to_vec(),
            plot_type: PlotType::Scatter,
            color: get_cycle_color(0),
            marker: Marker::Circle,
            marker_size: 6.0,
            line_width: 0.0,
            label: None,
            alpha: 1.0,
        }
    }

    /// Create a new bar plot
    pub fn bar(x: &[f64], y: &[f64]) -> Self {
        Plot {
            x_data: x.to_vec(),
            y_data: y.to_vec(),
            plot_type: PlotType::Bar,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 6.0,
            line_width: 1.0,
            label: None,
            alpha: 1.0,
        }
    }

    /// Set the color of the plot
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the marker style
    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }

    /// Set the marker size
    pub fn marker_size(mut self, size: f64) -> Self {
        self.marker_size = size;
        self
    }

    /// Set the line width
    pub fn line_width(mut self, width: f64) -> Self {
        self.line_width = width;
        self
    }

    /// Set the plot label for legend
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Set the alpha (transparency) value
    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    /// Generate SVG elements for this plot
    pub fn to_svg(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, 
                  plot_width: f64, plot_height: f64) -> String {
        let mut svg = String::new();
        
        if self.x_data.len() != self.y_data.len() || self.x_data.is_empty() {
            return svg;
        }

        let color_str = self.color.to_svg_string();
        
        match self.plot_type {
            PlotType::Line => {
                if self.line_width > 0.0 {
                    svg.push_str(&self.generate_line_svg(x_min, x_max, y_min, y_max, 
                                                        plot_width, plot_height, &color_str));
                }
                if self.marker.is_visible() {
                    svg.push_str(&self.generate_markers_svg(x_min, x_max, y_min, y_max, 
                                                           plot_width, plot_height, &color_str));
                }
            },
            PlotType::Scatter => {
                svg.push_str(&self.generate_markers_svg(x_min, x_max, y_min, y_max, 
                                                       plot_width, plot_height, &color_str));
            },
            PlotType::Bar => {
                svg.push_str(&self.generate_bar_svg(x_min, x_max, y_min, y_max, 
                                                   plot_width, plot_height, &color_str));
            },
            PlotType::Histogram => {
                // TODO: Implement histogram
            },
        }
        
        svg
    }

    fn generate_line_svg(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, 
                         plot_width: f64, plot_height: f64, color: &str) -> String {
        let mut path_data = String::new();
        
        for (i, (&x, &y)) in self.x_data.iter().zip(self.y_data.iter()).enumerate() {
            let svg_x = map_range(x, x_min, x_max, 0.0, plot_width);
            let svg_y = map_range(y, y_min, y_max, plot_height, 0.0); // Flip Y axis
            
            if i == 0 {
                path_data.push_str(&format!("M {},{}", svg_x, svg_y));
            } else {
                path_data.push_str(&format!(" L {},{}", svg_x, svg_y));
            }
        }
        
        format!("<path d=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\" opacity=\"{}\"/>",
                path_data, color, self.line_width, self.alpha)
    }

    fn generate_markers_svg(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, 
                           plot_width: f64, plot_height: f64, color: &str) -> String {
        let mut svg = String::new();
        
        for (&x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let svg_x = map_range(x, x_min, x_max, 0.0, plot_width);
            let svg_y = map_range(y, y_min, y_max, plot_height, 0.0); // Flip Y axis
            
            let marker_svg = self.marker.to_svg_element(svg_x, svg_y, self.marker_size, color);
            if !marker_svg.is_empty() {
                svg.push_str(&format!("<g opacity=\"{}\">{}</g>", self.alpha, marker_svg));
                
                // Add text label if available
                if let Some(ref label) = self.label {
                    svg.push_str(&format!(
                        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12\" fill=\"black\" dy=\"0.35em\">{}</text>",
                        svg_x, svg_y, label
                    ));
                }
            }
        }
        
        svg
    }

    fn generate_bar_svg(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, 
                        plot_width: f64, plot_height: f64, color: &str) -> String {
        let mut svg = String::new();
        let data_range = self.x_data.len() as f64;
        let bar_width = if data_range > 1.0 {
            (plot_width * 0.8) / data_range
        } else {
            plot_width * 0.1
        };
        
        // Leave margins for bar chart to avoid bars exceeding boundaries
        let margin = plot_width * 0.1;
        let effective_width = plot_width - 2.0 * margin;
        
        // Ensure y_min is 0 or smaller for bar chart baseline
        let base_y = if y_min > 0.0 { 0.0 } else { y_min };
        
        for (&x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            // Recalculate x coordinate to ensure bars are within effective area
            let svg_x = map_range(x, x_min, x_max, margin, margin + effective_width) - bar_width / 2.0;
            
            // Calculate top and bottom positions of the bar
            let bar_top = map_range(y.max(base_y), base_y, y_max, plot_height, 0.0);
            let bar_bottom = map_range(base_y, base_y, y_max, plot_height, 0.0);
            let bar_height = (bar_bottom - bar_top).abs();
            
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
                svg_x, bar_top, bar_width, bar_height, color, color, self.line_width, self.alpha
            ));
        }
        
        svg
    }
}