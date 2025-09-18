//! Axes functionality for plots

use crate::colors::Color;
use crate::plot::Plot;
use crate::utils::{calculate_range, generate_ticks, format_number, map_range};

/// Represents a set of axes for plotting
#[derive(Debug)]
pub struct Axes {
    pub plots: Vec<Plot>,
    pub custom_svg_elements: Vec<String>, // Store custom SVG elements like arrows
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub title: Option<String>,
    pub x_limits: Option<(f64, f64)>,
    pub y_limits: Option<(f64, f64)>,
    pub grid: bool,
    pub legend: bool,
    pub background_color: Color,
    pub grid_color: Color,
    pub text_color: Color,
    pub font_size: f64,
    pub show_x_axis: bool,
    pub show_y_axis: bool,
}

impl Axes {
    /// Create new axes
    pub fn new() -> Self {
        Axes {
            plots: Vec::new(),
            custom_svg_elements: Vec::new(),
            x_label: None,
            y_label: None,
            title: None,
            x_limits: None,
            y_limits: None,
            grid: true,
            legend: false,
            background_color: Color::WHITE,
            grid_color: Color::LIGHTGRAY,
            text_color: Color::BLACK,
            font_size: 12.0,
            show_x_axis: true,
            show_y_axis: true,
        }
    }

    /// Add a line plot
    pub fn plot(&mut self, x: &[f64], y: &[f64]) -> &mut Self {
        let color_index = self.plots.len();
        let mut plot = Plot::line(x, y);
        plot.color = crate::colors::get_cycle_color(color_index);
        self.plots.push(plot);
        self
    }

    /// Add a scatter plot
    pub fn scatter(&mut self, x: &[f64], y: &[f64]) -> &mut Self {
        let color_index = self.plots.len();
        let mut plot = Plot::scatter(x, y);
        plot.color = crate::colors::get_cycle_color(color_index);
        self.plots.push(plot);
        self
    }

    /// Add a bar plot
    pub fn bar(&mut self, x: &[f64], y: &[f64]) -> &mut Self {
        let color_index = self.plots.len();
        let mut plot = Plot::bar(x, y);
        plot.color = crate::colors::get_cycle_color(color_index);
        self.plots.push(plot);
        self
    }

    /// Add custom SVG element
    pub fn add_svg_element(&mut self, svg_element: String) {
        self.custom_svg_elements.push(svg_element);
    }

    /// Set the x-axis label
    pub fn set_xlabel(&mut self, label: &str) -> &mut Self {
        self.x_label = Some(label.to_string());
        self
    }

    /// Set the y-axis label
    pub fn set_ylabel(&mut self, label: &str) -> &mut Self {
        self.y_label = Some(label.to_string());
        self
    }

    /// Set the plot title
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set x-axis limits
    pub fn set_xlim(&mut self, min: f64, max: f64) -> &mut Self {
        self.x_limits = Some((min, max));
        self
    }

    /// Set y-axis limits
    pub fn set_ylim(&mut self, min: f64, max: f64) -> &mut Self {
        self.y_limits = Some((min, max));
        self
    }

    /// Enable or disable grid
    pub fn grid(&mut self, enable: bool) -> &mut Self {
        self.grid = enable;
        self
    }

    /// Enable or disable legend
    pub fn legend(&mut self, enable: bool) -> &mut Self {
        self.legend = enable;
        self
    }

    /// Enable or disable x-axis display
    pub fn show_x_axis(&mut self, show: bool) -> &mut Self {
        self.show_x_axis = show;
        self
    }

    /// Enable or disable y-axis display
    pub fn show_y_axis(&mut self, show: bool) -> &mut Self {
        self.show_y_axis = show;
        self
    }

    /// Calculate the data ranges for all plots
    fn calculate_data_ranges(&self) -> ((f64, f64), (f64, f64)) {
        if self.plots.is_empty() {
            return ((0.0, 1.0), (0.0, 1.0));
        }

        let mut all_x: Vec<f64> = Vec::new();
        let mut all_y: Vec<f64> = Vec::new();

        for plot in &self.plots {
            all_x.extend(&plot.x_data);
            all_y.extend(&plot.y_data);
        }

        let x_range = self.x_limits.unwrap_or_else(|| calculate_range(&all_x));
        let y_range = self.y_limits.unwrap_or_else(|| calculate_range(&all_y));

        (x_range, y_range)
    }

    /// Generate SVG for the axes
    pub fn to_svg(&self, width: f64, height: f64) -> String {
        let margin = 60.0;
        let plot_width = width - 2.0 * margin;
        let plot_height = height - 2.0 * margin;
        
        let ((x_min, x_max), (y_min, y_max)) = self.calculate_data_ranges();
        
        let mut svg = String::new();
        
        // Background
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />",
            margin, margin, plot_width, plot_height, self.background_color.to_svg_string()
        ));
        
        // Grid
        if self.grid {
            svg.push_str(&self.generate_grid_svg(x_min, x_max, y_min, y_max, 
                                                margin, plot_width, plot_height));
        }
        
        // Plot data
        for plot in &self.plots {
            svg.push_str(&format!("<g transform=\"translate({},{})\">\n", margin, margin));
            svg.push_str(&plot.to_svg(x_min, x_max, y_min, y_max, plot_width, plot_height));
            svg.push_str("</g>\n");
        }
        
        // Axes
        if self.show_x_axis || self.show_y_axis {
            svg.push_str(&self.generate_axes_svg(x_min, x_max, y_min, y_max, 
                                                margin, width, height, plot_width, plot_height));
        }
        
        // Labels and title
        svg.push_str(&self.generate_labels_svg(width, height, margin));
        
        // Custom SVG elements
        for element in &self.custom_svg_elements {
            svg.push_str(element);
            svg.push_str("\n");
        }
        
        // Legend
        if self.legend {
            svg.push_str(&self.generate_legend_svg(width, height));
        }
        
        svg
    }

    fn generate_grid_svg(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64,
                        margin: f64, plot_width: f64, plot_height: f64) -> String {
        let mut svg = String::new();
        let grid_color = self.grid_color.to_svg_string();
        
        // Vertical grid lines
        let x_ticks = generate_ticks(x_min, x_max, 8);
        for &tick in &x_ticks {
            let x = map_range(tick, x_min, x_max, 0.0, plot_width) + margin;
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\" />\n",
                x, margin, x, margin + plot_height, grid_color
            ));
        }
        
        // Horizontal grid lines
        let y_ticks = generate_ticks(y_min, y_max, 6);
        for &tick in &y_ticks {
            let y = map_range(tick, y_min, y_max, plot_height, 0.0) + margin;
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\" />\n",
                margin, y, margin + plot_width, y, grid_color
            ));
        }
        
        svg
    }

    fn generate_axes_svg(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64,
                        margin: f64, _width: f64, _height: f64, plot_width: f64, plot_height: f64) -> String {
        let mut svg = String::new();
        let text_color = self.text_color.to_svg_string();
        
        if self.show_x_axis {
            // X-axis
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" />\n",
                margin, margin + plot_height, margin + plot_width, margin + plot_height, text_color
            ));
            
            // X-axis ticks and labels
            let x_ticks = generate_ticks(x_min, x_max, 8);
            for &tick in &x_ticks {
                let x = map_range(tick, x_min, x_max, 0.0, plot_width) + margin;
                svg.push_str(&format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" />\n",
                    x, margin + plot_height, x, margin + plot_height + 5.0, text_color
                ));
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                    x, margin + plot_height + 20.0, self.font_size, text_color, format_number(tick)
                ));
            }
        }
        
        if self.show_y_axis {
            // Y-axis
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" />\n",
                margin, margin, margin, margin + plot_height, text_color
            ));
            
            // Y-axis ticks and labels
            let y_ticks = generate_ticks(y_min, y_max, 6);
            for &tick in &y_ticks {
                let y = map_range(tick, y_min, y_max, plot_height, 0.0) + margin;
                svg.push_str(&format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" />\n",
                    margin - 5.0, y, margin, y, text_color
                ));
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"{}\" fill=\"{}\" dy=\"0.35em\">{}</text>\n",
                    margin - 10.0, y, self.font_size, text_color, format_number(tick)
                ));
            }
        }
        
        svg
    }

    fn generate_labels_svg(&self, width: f64, height: f64, _margin: f64) -> String {
        let mut svg = String::new();
        let text_color = self.text_color.to_svg_string();
        
        // Title
        if let Some(ref title) = self.title {
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\" font-weight=\"bold\" fill=\"{}\">{}</text>\n",
                width / 2.0, 30.0, self.font_size + 4.0, text_color, title
            ));
        }
        
        // X-axis label
        if let Some(ref xlabel) = self.x_label {
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                width / 2.0, height - 10.0, self.font_size, text_color, xlabel
            ));
        }
        
        // Y-axis label
        if let Some(ref ylabel) = self.y_label {
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\" fill=\"{}\" transform=\"rotate(-90, {}, {})\">{}</text>\n",
                20.0, height / 2.0, self.font_size, text_color, 20.0, height / 2.0, ylabel
            ));
        }
        
        svg
    }

    fn generate_legend_svg(&self, width: f64, _height: f64) -> String {
        let mut svg = String::new();
        let legend_x = width - 150.0;
        let mut legend_y = 60.0;
        
        for plot in &self.plots {
            if let Some(ref label) = plot.label {
                // Legend box
                svg.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"15\" height=\"15\" fill=\"{}\" />\n",
                    legend_x, legend_y - 10.0, plot.color.to_svg_string()
                ));
                
                // Legend text
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n",
                    legend_x + 20.0, legend_y, self.font_size, self.text_color.to_svg_string(), label
                ));
                
                legend_y += 25.0;
            }
        }
        
        svg
    }
}

impl Default for Axes {
    fn default() -> Self {
        Self::new()
    }
}