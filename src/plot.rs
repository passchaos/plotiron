//! Plot types and plotting functionality

use crate::IntoVec;
use crate::colors::Color;
use crate::markers::Marker;
use crate::utils::map_range;

/// Different types of plots
#[derive(Debug, Clone, PartialEq)]
pub enum PlotType {
    /// Line plot
    Line,
    /// Scatter plot
    Scatter,
}

/// A single plot/series of data
#[derive(Debug, Clone)]
pub struct Plot {
    pub x_data: Vec<f64>,
    pub y_data: Vec<f64>,
    pub z_data: Option<Vec<Vec<f64>>>, // For contour plots and 3D data
    pub plot_type: PlotType,
    pub color: Option<Color>,
    pub marker: Marker,
    pub marker_size: f64,
    pub line_width: f64,
    pub label: Option<String>,
    pub alpha: f64,
}

impl Plot {
    /// Create a new line plot
    pub fn line<X, Y>(x: X, y: Y) -> Self
    where
        X: IntoVec<f64>,
        Y: IntoVec<f64>,
    {
        Plot {
            x_data: x.into_vec(),
            y_data: y.into_vec(),
            z_data: None,
            plot_type: PlotType::Line,
            color: None,
            marker: Marker::None,
            marker_size: 6.0,
            line_width: 2.0,
            label: None,
            alpha: 1.0,
        }
    }

    /// Create a new scatter plot
    pub fn scatter<X, Y>(x: X, y: Y) -> Self
    where
        X: IntoVec<f64>,
        Y: IntoVec<f64>,
    {
        Plot {
            x_data: x.into_vec(),
            y_data: y.into_vec(),
            z_data: None,
            plot_type: PlotType::Scatter,
            color: None,
            marker: Marker::Circle,
            marker_size: 4.0,
            line_width: 0.0,
            label: None,
            alpha: 1.0,
        }
    }

    /// Set the color of the plot
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
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

    pub fn plot_color(&self) -> Color {
        self.color.unwrap_or(Color::BLACK)
    }

    /// Generate SVG elements for this plot
    pub fn to_svg(
        &self,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        plot_width: f64,
        plot_height: f64,
    ) -> String {
        let mut svg = String::new();

        // Skip length check for special plot types that don't require matching x/y data lengths
        if self.x_data.len() != self.y_data.len() || self.x_data.is_empty() {
            return svg;
        }

        let color_str = self.plot_color().to_svg_string();

        match self.plot_type {
            PlotType::Line => {
                if self.line_width > 0.0 {
                    svg.push_str(&self.generate_line_svg(
                        x_min,
                        x_max,
                        y_min,
                        y_max,
                        plot_width,
                        plot_height,
                        &color_str,
                    ));
                }
                if self.marker.is_visible() {
                    svg.push_str(&self.generate_markers_svg(
                        x_min,
                        x_max,
                        y_min,
                        y_max,
                        plot_width,
                        plot_height,
                        &color_str,
                    ));
                }
            }
            PlotType::Scatter => {
                svg.push_str(&self.generate_markers_svg(
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    plot_width,
                    plot_height,
                    &color_str,
                ));
            }
        }

        svg
    }

    fn generate_line_svg(
        &self,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        plot_width: f64,
        plot_height: f64,
        color: &str,
    ) -> String {
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

        format!(
            "<path d=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\" opacity=\"{}\"/>",
            path_data, color, self.line_width, self.alpha
        )
    }

    fn generate_markers_svg(
        &self,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        plot_width: f64,
        plot_height: f64,
        color: &str,
    ) -> String {
        let mut svg = String::new();

        for (&x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let svg_x = map_range(x, x_min, x_max, 0.0, plot_width);
            let svg_y = map_range(y, y_min, y_max, plot_height, 0.0); // Flip Y axis

            let marker_svg = self
                .marker
                .to_svg_element(svg_x, svg_y, self.marker_size, color);
            if !marker_svg.is_empty() {
                svg.push_str(&format!("<g opacity=\"{}\">{}</g>", self.alpha, marker_svg));

                // // Add text label if available
                // if let Some(ref label) = self.label {
                //     svg.push_str(&format!(
                //         "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12\" fill=\"black\" dy=\"0.35em\">{}</text>",
                //         svg_x, svg_y, label
                //     ));
                // }
            }
        }

        svg
    }
}
