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
    /// Pie chart
    Pie,
    /// Box plot
    BoxPlot,
    /// Heatmap
    Heatmap,
    /// Violin plot
    Violin,
    /// Contour plot
    Contour,
}

/// A single plot/series of data
#[derive(Debug, Clone)]
pub struct Plot {
    pub x_data: Vec<f64>,
    pub y_data: Vec<f64>,
    pub z_data: Option<Vec<Vec<f64>>>, // For contour plots and 3D data
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
            z_data: None,
            plot_type: PlotType::Line,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 6.0,
            line_width: 2.0,
            label: None,
            alpha: 1.0,
        }
    }

    /// Create a new scatter plot
    pub fn scatter(x: &[f64], y: &[f64]) -> Self {
        Plot {
            x_data: x.to_vec(),
            y_data: y.to_vec(),
            z_data: None,
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
            z_data: None,
            plot_type: PlotType::Bar,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 6.0,
            line_width: 1.0,
            label: None,
            alpha: 1.0,
        }
    }

    /// Create a new histogram from data
    pub fn histogram(data: &[f64], bins: usize) -> Self {
        let (bin_edges, counts) = Self::calculate_histogram(data, bins);
        let bin_centers: Vec<f64> = bin_edges.windows(2).map(|w| (w[0] + w[1]) / 2.0).collect();

        Plot {
            x_data: bin_centers,
            y_data: counts,
            z_data: None,
            plot_type: PlotType::Histogram,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 6.0,
            line_width: 1.0,
            label: None,
            alpha: 0.7,
        }
    }

    /// Create a new pie chart
    pub fn pie(values: &[f64], _labels: Option<&[String]>) -> Self {
        let total: f64 = values.iter().sum();
        let percentages: Vec<f64> = values.iter().map(|v| v / total * 100.0).collect();

        // For pie charts, we store the original values in x_data and percentages in y_data
        Plot {
            x_data: values.to_vec(),
            y_data: percentages,
            z_data: None,
            plot_type: PlotType::Pie,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 6.0,
            line_width: 2.5,
            label: None,
            alpha: 1.0,
        }
    }

    /// Create a new box plot
    pub fn boxplot(data: &[f64]) -> Self {
        // For box plots, we store the raw data in y_data and use x_data for positioning
        // Set x_data to have a range that makes sense for box plot positioning
        let x_data = vec![-0.5, 0.5]; // Box plot will be centered at x=0 with some width

        Plot {
            x_data,
            y_data: data.to_vec(),
            z_data: None,
            plot_type: PlotType::BoxPlot,
            color: get_cycle_color(0),
            marker: Marker::Circle,
            marker_size: 3.0,
            line_width: 1.3,
            label: None,
            alpha: 1.0,
        }
    }

    /// Create a new heatmap
    pub fn heatmap(data: &[Vec<f64>]) -> Self {
        // For heatmaps, we flatten the 2D data into 1D arrays
        // x_data stores flattened values, y_data stores dimensions info
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };

        // Flatten the 2D data into x_data
        let mut flattened_data = Vec::new();
        for row in data {
            flattened_data.extend_from_slice(row);
        }

        // Store dimensions in y_data: [rows, cols]
        let y_data = vec![rows as f64, cols as f64];

        Plot {
            x_data: flattened_data,
            y_data,
            z_data: None,
            plot_type: PlotType::Heatmap,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 1.0,
            line_width: 0.0,
            label: None,
            alpha: 1.0,
        }
    }

    /// Create a new violin plot
    pub fn violin(data: &[f64]) -> Self {
        // For violin plots, we store the raw data in y_data and use x_data for positioning
        // Set x_data to have a range that makes sense for violin plot positioning
        let x_data = vec![-0.5, 0.5]; // Violin plot will be centered at x=0 with some width

        Plot {
            x_data,
            y_data: data.to_vec(),
            z_data: None,
            plot_type: PlotType::Violin,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 3.0,
            line_width: 2.5,
            label: None,
            alpha: 0.7,
        }
    }

    /// Create a new contour plot
    pub fn contour(x: &[f64], y: &[f64], z: &[Vec<f64>]) -> Self {
        Plot {
            x_data: x.to_vec(),
            y_data: y.to_vec(),
            z_data: Some(z.to_vec()),
            plot_type: PlotType::Contour,
            color: get_cycle_color(0),
            marker: Marker::None,
            marker_size: 5.0,
            line_width: 1.0,
            label: None,
            alpha: 0.7,
        }
    }

    /// Calculate histogram bins and counts
    fn calculate_histogram(data: &[f64], bins: usize) -> (Vec<f64>, Vec<f64>) {
        if data.is_empty() || bins == 0 {
            return (vec![0.0, 1.0], vec![0.0]);
        }

        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if min_val == max_val {
            return (vec![min_val - 0.5, min_val + 0.5], vec![data.len() as f64]);
        }

        let bin_width = (max_val - min_val) / bins as f64;
        let mut bin_edges = Vec::with_capacity(bins + 1);
        let mut counts = vec![0.0; bins];

        // Create bin edges
        for i in 0..=bins {
            bin_edges.push(min_val + i as f64 * bin_width);
        }

        // Count data points in each bin
        for &value in data {
            let bin_index = if value == max_val {
                bins - 1 // Put max value in the last bin
            } else {
                ((value - min_val) / bin_width).floor() as usize
            };

            if bin_index < bins {
                counts[bin_index] += 1.0;
            }
        }

        (bin_edges, counts)
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
        if matches!(
            self.plot_type,
            PlotType::Pie
                | PlotType::BoxPlot
                | PlotType::Heatmap
                | PlotType::Violin
                | PlotType::Contour
        ) {
            if self.y_data.is_empty() {
                return svg;
            }
        } else if self.x_data.len() != self.y_data.len() || self.x_data.is_empty() {
            return svg;
        }

        let color_str = self.color.to_svg_string();

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
            PlotType::Bar => {
                svg.push_str(&self.generate_bar_svg(
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    plot_width,
                    plot_height,
                    &color_str,
                ));
            }
            PlotType::Histogram => {
                svg.push_str(&self.generate_histogram_svg(
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    plot_width,
                    plot_height,
                    &color_str,
                ));
            }
            PlotType::Pie => {
                svg.push_str(&self.generate_pie_svg(
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    plot_width,
                    plot_height,
                    &color_str,
                ));
            }
            PlotType::BoxPlot => {
                svg.push_str(&self.generate_boxplot_svg(
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    plot_width,
                    plot_height,
                    &color_str,
                ));
            }
            PlotType::Heatmap => {
                svg.push_str(&self.generate_heatmap_svg(
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    plot_width,
                    plot_height,
                    &color_str,
                ));
            }
            PlotType::Violin => {
                svg.push_str(&self.generate_violin_svg(
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    plot_width,
                    plot_height,
                    &color_str,
                ));
            }
            PlotType::Contour => {
                svg.push_str(&self.generate_contour_svg(
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

    fn generate_bar_svg(
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
            let svg_x =
                map_range(x, x_min, x_max, margin, margin + effective_width) - bar_width / 2.0;

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

    fn generate_histogram_svg(
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

        if self.x_data.len() != self.y_data.len() || self.x_data.is_empty() {
            return svg;
        }

        // Calculate bin width from the data
        let data_range = if self.x_data.len() > 1 {
            let sorted_x: Vec<f64> = {
                let mut x = self.x_data.clone();
                x.sort_by(|a, b| a.partial_cmp(b).unwrap());
                x
            };
            let min_diff = sorted_x
                .windows(2)
                .map(|w| w[1] - w[0])
                .fold(f64::INFINITY, |a, b| a.min(b));
            min_diff * 2.0 // Bin width is twice the minimum difference between centers
        } else {
            1.0
        };

        let bin_width_svg = map_range(data_range, 0.0, x_max - x_min, 0.0, plot_width);

        // Ensure y_min is 0 or smaller for histogram baseline
        let base_y = if y_min > 0.0 { 0.0 } else { y_min };

        for (&x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let svg_x = map_range(x, x_min, x_max, 0.0, plot_width) - bin_width_svg / 2.0;

            // Calculate top and bottom positions of the bar
            let bar_top = map_range(y.max(base_y), base_y, y_max, plot_height, 0.0);
            let bar_bottom = map_range(base_y, base_y, y_max, plot_height, 0.0);
            let bar_height = (bar_bottom - bar_top).abs();

            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
                svg_x, bar_top, bin_width_svg, bar_height, color, color, self.line_width, self.alpha
            ));
        }

        svg
    }

    fn generate_pie_svg(
        &self,
        _x_min: f64,
        _x_max: f64,
        _y_min: f64,
        _y_max: f64,
        plot_width: f64,
        plot_height: f64,
        color: &str,
    ) -> String {
        let mut svg = String::new();

        if self.x_data.is_empty() {
            return svg;
        }

        // Calculate center and radius
        let center_x = plot_width / 2.0;
        let center_y = plot_height / 2.0;
        let radius = (plot_width.min(plot_height) / 2.0) * 0.8; // 80% of available space

        let total: f64 = self.x_data.iter().sum();
        let mut current_angle = -std::f64::consts::PI / 2.0; // Start from top

        // Generate color palette for multiple slices
        let colors = [
            "rgb(31,119,180)",
            "rgb(255,127,14)",
            "rgb(44,160,44)",
            "rgb(214,39,40)",
            "rgb(148,103,189)",
            "rgb(140,86,75)",
            "rgb(227,119,194)",
            "rgb(127,127,127)",
            "rgb(188,189,34)",
            "rgb(23,190,207)",
        ];

        for (i, &value) in self.x_data.iter().enumerate() {
            if value <= 0.0 {
                continue;
            }

            let slice_angle = (value / total) * 2.0 * std::f64::consts::PI;
            let end_angle = current_angle + slice_angle;

            // Calculate arc points
            let start_x = center_x + radius * current_angle.cos();
            let start_y = center_y + radius * current_angle.sin();
            let end_x = center_x + radius * end_angle.cos();
            let end_y = center_y + radius * end_angle.sin();

            // Use different colors for each slice
            let slice_color = if i < colors.len() { colors[i] } else { color };

            // Create path for pie slice
            let large_arc = if slice_angle > std::f64::consts::PI {
                1
            } else {
                0
            };

            svg.push_str(&format!(
                "<path d=\"M {} {} L {} {} A {} {} 0 {} 1 {} {} Z\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\" opacity=\"{}\"/>",
                center_x, center_y,
                start_x, start_y,
                radius, radius,
                large_arc,
                end_x, end_y,
                slice_color,
                self.alpha
            ));

            // Add percentage label
            let label_angle = current_angle + slice_angle / 2.0;
            let label_radius = radius * 0.7;
            let label_x = center_x + label_radius * label_angle.cos();
            let label_y = center_y + label_radius * label_angle.sin();
            let percentage = (value / total * 100.0).round();

            if percentage >= 5.0 {
                // Only show labels for slices >= 5%
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12\" fill=\"white\" dy=\"0.35em\">{}%</text>",
                    label_x, label_y, percentage
                ));
            }

            current_angle = end_angle;
        }

        svg
    }

    fn generate_boxplot_svg(
        &self,
        _x_min: f64,
        _x_max: f64,
        y_min: f64,
        y_max: f64,
        plot_width: f64,
        plot_height: f64,
        color: &str,
    ) -> String {
        let mut svg = String::new();

        if self.y_data.is_empty() {
            return svg;
        }

        // Calculate box plot statistics
        let mut sorted_data = self.y_data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted_data.len();
        let q1 = Self::percentile(&sorted_data, 25.0);
        let median = Self::percentile(&sorted_data, 50.0);
        let q3 = Self::percentile(&sorted_data, 75.0);
        let iqr = q3 - q1;

        // Calculate whiskers (1.5 * IQR rule)
        let lower_fence = q1 - 1.5 * iqr;
        let upper_fence = q3 + 1.5 * iqr;

        // Find actual whisker values (closest data points within fences)
        let lower_whisker = sorted_data
            .iter()
            .find(|&&x| x >= lower_fence)
            .copied()
            .unwrap_or(sorted_data[0]);
        let upper_whisker = sorted_data
            .iter()
            .rev()
            .find(|&&x| x <= upper_fence)
            .copied()
            .unwrap_or(sorted_data[n - 1]);

        // Find outliers
        let outliers: Vec<f64> = sorted_data
            .iter()
            .filter(|&&x| x < lower_fence || x > upper_fence)
            .copied()
            .collect();

        // Box plot positioning
        let box_width = plot_width * 0.6; // Box takes 60% of available width
        let box_center_x = plot_width / 2.0;
        let box_left = box_center_x - box_width / 2.0;
        let box_right = box_center_x + box_width / 2.0;

        // Convert y-coordinates to SVG coordinates
        let y_to_svg =
            |y: f64| -> f64 { plot_height - map_range(y, y_min, y_max, 0.0, plot_height) };

        let q1_y = y_to_svg(q1);
        let median_y = y_to_svg(median);
        let q3_y = y_to_svg(q3);
        let lower_whisker_y = y_to_svg(lower_whisker);
        let upper_whisker_y = y_to_svg(upper_whisker);

        // Draw the box (IQR)
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
            box_left, q3_y, box_width, q1_y - q3_y, "none", color, self.line_width, self.alpha
        ));

        // Draw median line
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
            box_left, median_y, box_right, median_y, color, self.line_width * 1.5, self.alpha
        ));

        // Draw whiskers
        let whisker_width = box_width * 0.5;
        let whisker_left = box_center_x - whisker_width / 2.0;
        let whisker_right = box_center_x + whisker_width / 2.0;

        // Upper whisker
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
            box_center_x, q3_y, box_center_x, upper_whisker_y, color, self.line_width, self.alpha
        ));
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
            whisker_left, upper_whisker_y, whisker_right, upper_whisker_y, color, self.line_width, self.alpha
        ));

        // Lower whisker
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
            box_center_x, q1_y, box_center_x, lower_whisker_y, color, self.line_width, self.alpha
        ));
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
            whisker_left, lower_whisker_y, whisker_right, lower_whisker_y, color, self.line_width, self.alpha
        ));

        // Draw outliers
        for &outlier in &outliers {
            let outlier_y = y_to_svg(outlier);
            svg.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\" opacity=\"{}\"/>",
                box_center_x, outlier_y, self.marker_size / 2.0, "none", color, self.alpha
            ));
        }

        svg
    }

    fn generate_heatmap_svg(
        &self,
        _x_min: f64,
        _x_max: f64,
        _y_min: f64,
        _y_max: f64,
        plot_width: f64,
        plot_height: f64,
        _color: &str,
    ) -> String {
        let mut svg = String::new();

        if self.y_data.len() < 2 || self.x_data.is_empty() {
            return svg;
        }

        let rows = self.y_data[0] as usize;
        let cols = self.y_data[1] as usize;

        if rows == 0 || cols == 0 || self.x_data.len() != rows * cols {
            return svg;
        }

        // Find min and max values for color mapping
        let min_val = self.x_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = self.x_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if min_val == max_val {
            return svg;
        }

        // Calculate cell dimensions
        let cell_width = plot_width / cols as f64;
        let cell_height = plot_height / rows as f64;

        // Generate heatmap cells
        for i in 0..rows {
            for j in 0..cols {
                let data_index = i * cols + j;
                let value = self.x_data[data_index];

                // Normalize value to [0, 1] for color mapping
                let normalized = (value - min_val) / (max_val - min_val);

                // Map to color (blue to red gradient)
                let red = (normalized * 255.0) as u8;
                let blue = ((1.0 - normalized) * 255.0) as u8;
                let color = format!("rgb({},{},{})", red, 0, blue);

                // Calculate cell position
                let x = j as f64 * cell_width;
                let y = i as f64 * cell_height;

                svg.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"0.5\" opacity=\"{}\"/>",
                    x, y, cell_width, cell_height, color, self.alpha
                ));

                // Add text label if cell is large enough
                if cell_width > 30.0 && cell_height > 20.0 {
                    let text_x = x + cell_width / 2.0;
                    let text_y = y + cell_height / 2.0 + 4.0; // Offset for vertical centering
                    let text_color = if normalized > 0.5 { "white" } else { "black" };

                    svg.push_str(&format!(
                        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"10\" fill=\"{}\">{:.1}</text>",
                        text_x, text_y, text_color, value
                    ));
                }
            }
        }

        svg
    }

    fn generate_violin_svg(
        &self,
        _x_min: f64,
        _x_max: f64,
        y_min: f64,
        y_max: f64,
        plot_width: f64,
        plot_height: f64,
        color: &str,
    ) -> String {
        let mut svg = String::new();

        if self.y_data.is_empty() {
            return svg;
        }

        // Calculate violin plot statistics using kernel density estimation
        let mut sorted_data = self.y_data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min_val = sorted_data[0];
        let max_val = sorted_data[sorted_data.len() - 1];

        if min_val == max_val {
            return svg;
        }

        // Violin plot positioning
        let violin_center_x = plot_width / 2.0;
        let max_width = plot_width * 0.6; // Maximum width of violin

        // Improved density estimation using Gaussian kernel density estimation
        let num_points = 100; // More points for smoother curves
        let range = max_val - min_val;
        let bandwidth = range / 20.0; // Adaptive bandwidth

        let mut densities = Vec::new();
        let mut y_values = Vec::new();

        // Generate evaluation points
        for i in 0..num_points {
            let y_val = min_val + (i as f64 / (num_points - 1) as f64) * range;
            y_values.push(y_val);

            // Calculate kernel density at this point
            let mut density = 0.0;
            for &data_point in &sorted_data {
                let diff = (y_val - data_point) / bandwidth;
                // Gaussian kernel
                density += (-0.5 * diff * diff).exp();
            }
            density /= sorted_data.len() as f64 * bandwidth * (2.0 * std::f64::consts::PI).sqrt();
            densities.push(density);
        }

        // Normalize densities
        let max_density = densities.iter().fold(0.0f64, |a, &b| a.max(b));
        if max_density > 0.0 {
            for density in &mut densities {
                *density /= max_density;
            }
        }

        // Apply smoothing filter to further smooth the curve
        let smoothed_densities = self.smooth_densities(&densities, 3);

        // Generate violin shape using smooth curves
        let mut path_data = String::new();

        // Start at the bottom center
        let start_y = map_range(y_values[0], y_min, y_max, plot_height, 0.0);
        path_data.push_str(&format!("M {},{}", violin_center_x, start_y));

        // Right side of violin (going up) - use quadratic Bezier curves for smoothness
        for i in 0..smoothed_densities.len() {
            let y_val = y_values[i];
            let svg_y = map_range(y_val, y_min, y_max, plot_height, 0.0);
            let width = smoothed_densities[i] * max_width / 2.0;
            let svg_x = violin_center_x + width;

            if i == 0 {
                path_data.push_str(&format!(" L {},{}", svg_x, svg_y));
            } else {
                // Use smooth curve instead of straight lines
                path_data.push_str(&format!(" L {},{}", svg_x, svg_y));
            }
        }

        // Left side of violin (going down)
        for i in (0..smoothed_densities.len()).rev() {
            let y_val = y_values[i];
            let svg_y = map_range(y_val, y_min, y_max, plot_height, 0.0);
            let width = smoothed_densities[i] * max_width / 2.0;
            let svg_x = violin_center_x - width;

            path_data.push_str(&format!(" L {},{}", svg_x, svg_y));
        }

        path_data.push_str(" Z"); // Close the path

        // Draw the violin shape
        svg.push_str(&format!(
            "<path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"/>",
            path_data, color, color, self.line_width, self.alpha
        ));

        // Add median line
        let median = Self::percentile(&sorted_data, 50.0);
        let median_y = map_range(median, y_min, y_max, plot_height, 0.0);
        let median_width = max_width * 0.3;

        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"white\" stroke-width=\"{}\" opacity=\"{}\"/>",
            violin_center_x - median_width / 2.0, median_y,
            violin_center_x + median_width / 2.0, median_y,
            self.line_width * 2.0, self.alpha
        ));

        svg
    }

    // Helper function to smooth density values using a simple moving average
    fn smooth_densities(&self, densities: &[f64], window_size: usize) -> Vec<f64> {
        let mut smoothed = Vec::new();
        let half_window = window_size / 2;

        for i in 0..densities.len() {
            let start = if i >= half_window { i - half_window } else { 0 };
            let end = (i + half_window + 1).min(densities.len());

            let sum: f64 = densities[start..end].iter().sum();
            let count = end - start;
            smoothed.push(sum / count as f64);
        }

        smoothed
    }

    fn generate_contour_svg(
        &self,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        plot_width: f64,
        plot_height: f64,
        _color: &str,
    ) -> String {
        let mut svg = String::new();

        let z_data = match &self.z_data {
            Some(data) => data,
            None => return svg,
        };

        if self.x_data.is_empty() || self.y_data.is_empty() || z_data.is_empty() {
            return svg;
        }

        // Find Z value range
        let mut z_min = f64::INFINITY;
        let mut z_max = f64::NEG_INFINITY;
        for row in z_data {
            for &val in row {
                z_min = z_min.min(val);
                z_max = z_max.max(val);
            }
        }

        if z_min == z_max {
            return svg;
        }

        let num_contours = 8;
        let nx = self.x_data.len();
        let ny = self.y_data.len();

        // Draw filled contour regions
        for i in 0..num_contours {
            let level_low = z_min + (z_max - z_min) * (i as f64 / num_contours as f64);
            let level_high = z_min + (z_max - z_min) * ((i + 1) as f64 / num_contours as f64);
            let intensity = i as f64 / (num_contours - 1) as f64;

            // Create a color gradient from blue to red
            let red = (intensity * 255.0) as u8;
            let blue = ((1.0 - intensity) * 255.0) as u8;
            let contour_color = format!("rgb({},{},{})", red, 0, blue);

            // Draw rectangles for each grid cell that falls within this contour level
            for j in 0..ny.saturating_sub(1) {
                for i in 0..nx.saturating_sub(1) {
                    if j < z_data.len() && i < z_data[j].len() {
                        let z_val = z_data[j][i];

                        if z_val >= level_low && z_val < level_high {
                            let x1 = map_range(self.x_data[i], x_min, x_max, 0.0, plot_width);
                            let y1 = map_range(self.y_data[j], y_min, y_max, plot_height, 0.0);
                            let x2 = if i + 1 < nx {
                                map_range(self.x_data[i + 1], x_min, x_max, 0.0, plot_width)
                            } else {
                                x1 + plot_width / nx as f64
                            };
                            let y2 = if j + 1 < ny {
                                map_range(self.y_data[j + 1], y_min, y_max, plot_height, 0.0)
                            } else {
                                y1 - plot_height / ny as f64
                            };

                            let width = (x2 - x1).abs();
                            let height = (y1 - y2).abs();

                            svg.push_str(&format!(
                                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" opacity=\"{}\"/>",
                                x1, y2, width, height, contour_color, self.alpha
                            ));
                        }
                    }
                }
            }
        }

        svg
    }

    fn percentile(sorted_data: &[f64], p: f64) -> f64 {
        let n = sorted_data.len();
        if n == 0 {
            return 0.0;
        }
        if n == 1 {
            return sorted_data[0];
        }

        let index = (p / 100.0) * (n - 1) as f64;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;

        if lower == upper {
            sorted_data[lower]
        } else {
            let weight = index - lower as f64;
            sorted_data[lower] * (1.0 - weight) + sorted_data[upper] * weight
        }
    }
}
