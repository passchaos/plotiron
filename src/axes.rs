//! Axes functionality for plots

use crate::colors::Color;
use crate::plot::Plot;
use crate::utils::{calculate_range, format_number, generate_ticks, map_range};

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
    pub equal_aspect: bool,
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
            grid_color: Color::GRID_COLOR,
            text_color: Color::TEXT_COLOR,
            font_size: 16.0,
            show_x_axis: true,
            show_y_axis: true,
            equal_aspect: false,
        }
    }

    pub fn add_plot(&mut self, mut plot: Plot) -> &mut Self {
        if plot.color.is_none() {
            plot.color = Some(crate::colors::get_cycle_color(self.plots.len()));
        }

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

    /// Enable or disable equal aspect ratio (same scale for x and y axes)
    pub fn equal_aspect(&mut self, enable: bool) -> &mut Self {
        self.equal_aspect = enable;
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
            match plot.plot_type {
                _ => {
                    // Regular plots use both x and y data
                    all_x.extend(&plot.x_data);
                    all_y.extend(&plot.y_data);
                }
            }
        }

        let x_range = self.x_limits.unwrap_or_else(|| calculate_range(&all_x));
        let y_range = self.y_limits.unwrap_or_else(|| calculate_range(&all_y));

        (x_range, y_range)
    }

    /// Generate adaptive ticks that tries to produce the target count
    fn generate_adaptive_ticks(&self, min: f64, max: f64, target_count: usize) -> Vec<f64> {
        if min >= max || target_count == 0 {
            return vec![min, max];
        }

        let range = max - min;
        let raw_step = range / (target_count - 1) as f64;

        // Find a "nice" step size, but be more flexible for target count
        let magnitude = 10.0_f64.powf(raw_step.log10().floor());
        let normalized_step = raw_step / magnitude;

        let nice_step = if normalized_step <= 1.0 {
            1.0
        } else if normalized_step <= 1.25 {
            1.25
        } else if normalized_step <= 2.0 {
            2.0
        } else if normalized_step <= 2.5 {
            2.5
        } else if normalized_step <= 5.0 {
            5.0
        } else {
            10.0
        } * magnitude;

        // Generate ticks
        let start = (min / nice_step).floor() * nice_step;
        let mut ticks = Vec::new();
        let mut current = start;

        while current <= max + nice_step * 0.001 {
            if current >= min - nice_step * 0.001 {
                ticks.push(current);
            }
            current += nice_step;
        }

        // If we don't have enough ticks, try a smaller step
        if ticks.len() < target_count && ticks.len() > 2 {
            let smaller_step = nice_step / 2.0;
            let start = (min / smaller_step).floor() * smaller_step;
            let mut new_ticks = Vec::new();
            let mut current = start;

            while current <= max + smaller_step * 0.001 {
                if current >= min - smaller_step * 0.001 {
                    new_ticks.push(current);
                }
                current += smaller_step;
            }

            if new_ticks.len() <= target_count + 2 {
                ticks = new_ticks;
            }
        }

        if ticks.is_empty() {
            vec![min, max]
        } else {
            ticks
        }
    }

    /// Generate SVG for the axes
    pub fn to_svg(&self, width: f64, height: f64) -> String {
        let margin = 60.0;
        let plot_width = width - 2.0 * margin;
        let plot_height = height - 2.0 * margin;

        let ((mut x_min, mut x_max), (mut y_min, mut y_max)) = self.calculate_data_ranges();

        // Apply equal aspect ratio if enabled
        if self.equal_aspect {
            let x_range = x_max - x_min;
            let y_range = y_max - y_min;
            let x_scale = plot_width / x_range;
            let y_scale = plot_height / y_range;

            // Use the smaller scale to ensure both axes fit
            let scale = x_scale.min(y_scale);

            // Adjust ranges to maintain equal scaling
            let new_x_range = plot_width / scale;
            let new_y_range = plot_height / scale;

            let x_center = (x_min + x_max) / 2.0;
            let y_center = (y_min + y_max) / 2.0;

            x_min = x_center - new_x_range / 2.0;
            x_max = x_center + new_x_range / 2.0;
            y_min = y_center - new_y_range / 2.0;
            y_max = y_center + new_y_range / 2.0;
        }

        let mut svg = String::new();

        // Background
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />",
            margin,
            margin,
            plot_width,
            plot_height,
            self.background_color.to_svg_string()
        ));

        // Grid (skip for pie charts)
        if self.grid {
            svg.push_str(&self.generate_grid_svg(
                x_min,
                x_max,
                y_min,
                y_max,
                margin,
                plot_width,
                plot_height,
            ));
        }

        // Plot data
        for plot in &self.plots {
            svg.push_str(&format!(
                "<g transform=\"translate({},{})\">\n",
                margin, margin
            ));
            svg.push_str(&plot.to_svg(x_min, x_max, y_min, y_max, plot_width, plot_height));
            svg.push_str("</g>\n");
        }

        // Axes (skip for pie charts)
        if self.show_x_axis || self.show_y_axis {
            svg.push_str(&self.generate_axes_svg(
                x_min,
                x_max,
                y_min,
                y_max,
                margin,
                width,
                height,
                plot_width,
                plot_height,
            ));
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

        // Outer border (matplotlib style)
        let border_color = Color::AXIS_COLOR.to_svg_string();
        svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.8\" />\n",
                margin, margin, plot_width, plot_height, border_color
            ));

        svg
    }

    fn generate_grid_svg(
        &self,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        margin: f64,
        plot_width: f64,
        plot_height: f64,
    ) -> String {
        let mut svg = String::new();
        let grid_color = self.grid_color.to_svg_string();

        // Vertical grid lines
        let x_ticks = generate_ticks(x_min, x_max, 12);
        for &tick in &x_ticks {
            let x = map_range(tick, x_min, x_max, 0.0, plot_width) + margin;
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.3\" />\n",
                x, margin, x, margin + plot_height, grid_color
            ));
        }

        // Horizontal grid lines
        let y_ticks = self.generate_adaptive_ticks(y_min, y_max, 9);
        for &tick in &y_ticks {
            let y = map_range(tick, y_min, y_max, plot_height, 0.0) + margin;
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.3\" />\n",
                margin, y, margin + plot_width, y, grid_color
            ));
        }

        svg
    }

    fn generate_axes_svg(
        &self,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        margin: f64,
        _width: f64,
        _height: f64,
        plot_width: f64,
        plot_height: f64,
    ) -> String {
        let mut svg = String::new();
        let text_color = self.text_color.to_svg_string();
        let axis_color = Color::AXIS_COLOR.to_svg_string();

        if self.show_x_axis {
            // X-axis
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.8\" />\n",
                margin, margin + plot_height, margin + plot_width, margin + plot_height, axis_color
            ));

            // X-axis ticks and labels
            let x_ticks = generate_ticks(x_min, x_max, 12);
            for &tick in &x_ticks {
                let x = map_range(tick, x_min, x_max, 0.0, plot_width) + margin;
                svg.push_str(&format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.8\" />\n",
                    x, margin + plot_height, x, margin + plot_height + 5.0, axis_color
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
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.8\" />\n",
                margin, margin, margin, margin + plot_height, axis_color
            ));

            // Y-axis ticks and labels
            let y_ticks = self.generate_adaptive_ticks(y_min, y_max, 9);
            for &tick in &y_ticks {
                let y = map_range(tick, y_min, y_max, plot_height, 0.0) + margin;
                svg.push_str(&format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.8\" />\n",
                    margin - 5.0, y, margin, y, axis_color
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
        let margin = 60.0;
        let plot_width = width - 2.0 * margin;

        // Calculate legend dimensions
        let legend_entries: Vec<_> = self.plots.iter().filter(|p| p.label.is_some()).collect();
        if legend_entries.is_empty() {
            return svg;
        }

        // Simple matplotlib-style legend parameters
        let legend_padding = 2.0; // Minimal internal padding
        let legend_border_width = 1.0;
        let line_height = 22.0; // More generous spacing between entries
        let handle_length = 35.0; // Longer handle length for better visibility
        let handle_text_gap = 8.0; // Clear gap between handle and text

        // Calculate dynamic legend dimensions based on content
        let legend_height = legend_entries.len() as f64 * line_height + 2.0 * legend_padding;

        // Calculate maximum text width to determine legend width
        let mut max_text_width = 0.0f64;
        for plot in &legend_entries {
            if let Some(ref label) = plot.label {
                // Estimate text width: approximately 0.6 * actual_font_size per character
                let actual_font_size = self.font_size * 0.9;
                let estimated_width = label.len() as f64 * actual_font_size * 0.6;
                max_text_width = max_text_width.max(estimated_width);
            }
        }

        // Calculate total legend width: padding + handle + gap + text (no right padding)
        let legend_width = legend_padding + handle_length + handle_text_gap + max_text_width;

        let legend_x = margin + plot_width - legend_width - 10.0; // Position legend within plot area (standard margin)
        let legend_y = margin + 20.0; // Start legend below the top margin

        // Simple legend background with subtle border and rounded corners
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"white\" stroke=\"#cccccc\" stroke-width=\"{}\" rx=\"3\" />\n",
            legend_x - legend_padding,
            legend_y - legend_padding,
            legend_width,
            legend_height,
            legend_border_width
        ));

        let mut current_y = legend_y + line_height * 0.7; // Adjust for text baseline

        for plot in &self.plots {
            if let Some(ref label) = plot.label {
                // Legend handle (line for line plots, rect for others)
                match plot.plot_type {
                    crate::plot::PlotType::Line => {
                        // Draw a line handle like matplotlib
                        svg.push_str(&format!(
                            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" />\n",
                            legend_x + legend_padding,
                            current_y - 3.0,
                            legend_x + legend_padding + handle_length,
                            current_y - 3.0,
                            plot.plot_color().to_svg_string()
                        ));
                    }
                    crate::plot::PlotType::Scatter => {
                        // Draw a circle marker for scatter plots
                        svg.push_str(&format!(
                            "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" />\n",
                            legend_x + legend_padding + handle_length / 2.0,
                            current_y - 3.0,
                            plot.plot_color().to_svg_string()
                        ));
                    }
                }

                // Legend text
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-size=\"{}\" font-family=\"Arial, sans-serif\" fill=\"{}\">{}</text>\n",
                    legend_x + legend_padding + handle_length + handle_text_gap,
                    current_y,
                    (self.font_size * 0.9) as i32,  // Slightly smaller font size
                    self.text_color.to_svg_string(),
                    label
                ));

                current_y += line_height;
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
