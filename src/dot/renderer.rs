//! DOT graph rendering functionality

use super::types::*;
use crate::axes::Axes;
use crate::colors::Color;
use crate::markers::Marker;

impl DotGraph {
    pub fn render_to_axes(&self, axes: &mut Axes) {
        // Render subgraph backgrounds first
        for subgraph in &self.subgraphs {
            self.render_subgraph_background(axes, subgraph);
        }

        // Render edges (so they appear behind nodes but above subgraph backgrounds)
        for edge in &self.edges {
            if let (Some(from_node), Some(to_node)) = (
                self.nodes.iter().find(|n| n.id == edge.from),
                self.nodes.iter().find(|n| n.id == edge.to),
            ) {
                // Calculate edge endpoints at node boundaries instead of centers
                let (start_x, start_y) = self.calculate_edge_start_point(from_node, to_node);
                let (end_x, end_y) = self.calculate_edge_end_point(from_node, to_node);

                // Generate curved path for better visual appearance
                let (x_line, y_line) = self.generate_curved_edge(start_x, start_y, end_x, end_y);
                axes.plot(x_line, y_line);

                if let Some(last_plot) = axes.plots.last_mut() {
                    last_plot.color = edge.color.clone();
                    last_plot.line_width = match edge.style {
                        EdgeStyle::Solid => 2.0,
                        EdgeStyle::Dashed => 2.0,
                        EdgeStyle::Dotted => 1.5,
                    };
                }

                // Add arrow for directed edges
                if edge.directed {
                    self.add_arrow(axes, from_node, to_node);
                }
            }
        }

        // Render nodes individually to support different colors and shapes
        for node in &self.nodes {
            let x_coords = vec![node.x];
            let y_coords = vec![node.y];

            axes.scatter(x_coords, y_coords);
            if let Some(last_plot) = axes.plots.last_mut() {
                last_plot.marker = match node.shape {
                    NodeShape::Circle => Marker::Circle,
                    NodeShape::Rectangle => Marker::Square,
                    NodeShape::Diamond => Marker::Diamond,
                    NodeShape::Ellipse => Marker::Ellipse, // Use proper ellipse shape
                    NodeShape::Mdiamond => Marker::Mdiamond,
                    NodeShape::Msquare => Marker::Msquare,
                };
                last_plot.marker_size = match node.shape {
                    NodeShape::Mdiamond | NodeShape::Msquare => 50.0, // Much larger to match graphviz size
                    _ => 15.0,
                };
                last_plot.color = node.color.clone();
                // Add node label if available
                if let Some(ref label) = node.label {
                    last_plot.label = Some(label.clone());
                }
            }
        }

        // Render subgraph borders and labels on top
        for subgraph in &self.subgraphs {
            self.render_subgraph_border(axes, subgraph);
        }
    }

    fn render_subgraph_background(&self, axes: &mut Axes, subgraph: &Subgraph) {
        if subgraph.nodes.is_empty() {
            return;
        }

        // Find bounding box of subgraph nodes
        let subgraph_nodes: Vec<&Node> = self
            .nodes
            .iter()
            .filter(|n| subgraph.nodes.contains(&n.id))
            .collect();

        if subgraph_nodes.is_empty() {
            return;
        }

        let min_x = subgraph_nodes
            .iter()
            .map(|n| n.x)
            .fold(f64::INFINITY, f64::min)
            - 0.05;
        let max_x = subgraph_nodes
            .iter()
            .map(|n| n.x)
            .fold(f64::NEG_INFINITY, f64::max)
            + 0.05;
        let min_y = subgraph_nodes
            .iter()
            .map(|n| n.y)
            .fold(f64::INFINITY, f64::min)
            - 0.05;
        let max_y = subgraph_nodes
            .iter()
            .map(|n| n.y)
            .fold(f64::NEG_INFINITY, f64::max)
            + 0.05;

        // Draw filled rectangle for subgraph background
        if let Some(ref style) = subgraph.style {
            if style == "filled" {
                let fill_color = if let Some(ref color) = subgraph.fill_color {
                    color.as_str()
                } else {
                    "lightgrey"
                };

                // Use the same coordinate system as the border rendering
                let border_x = vec![min_x, max_x, max_x, min_x, min_x];
                let border_y = vec![min_y, min_y, max_y, max_y, min_y];

                // Create filled background manually since Plot doesn't support fill
                // First add a temporary plot to get the coordinate transformation
                axes.plot(border_x.as_slice(), border_y.as_slice());

                // Remove the temporary plot and create a filled polygon instead
                if let Some(_) = axes.plots.pop() {
                    // Get coordinate ranges from existing plots (if any) or use global range
                    let ((x_min, x_max), (y_min, y_max)) = if axes.plots.is_empty() {
                        // Use global coordinate range when no other plots exist
                        let all_nodes_x: Vec<f64> = self.nodes.iter().map(|n| n.x).collect();
                        let all_nodes_y: Vec<f64> = self.nodes.iter().map(|n| n.y).collect();
                        let x_range = crate::utils::calculate_range(&all_nodes_x);
                        let y_range = crate::utils::calculate_range(&all_nodes_y);
                        (x_range, y_range)
                    } else {
                        // Use the range from existing plots
                        let mut all_x: Vec<f64> = Vec::new();
                        let mut all_y: Vec<f64> = Vec::new();
                        for plot in &axes.plots {
                            all_x.extend(&plot.x_data);
                            all_y.extend(&plot.y_data);
                        }
                        let x_range = crate::utils::calculate_range(&all_x);
                        let y_range = crate::utils::calculate_range(&all_y);
                        (x_range, y_range)
                    };

                    // Convert coordinates using the same transformation as plots
                    let margin = 60.0;
                    let plot_width = 680.0;
                    let plot_height = 480.0;

                    let mut svg_points = Vec::new();
                    for i in 0..border_x.len() {
                        let svg_x =
                            crate::utils::map_range(border_x[i], x_min, x_max, 0.0, plot_width);
                        let svg_y =
                            crate::utils::map_range(border_y[i], y_min, y_max, plot_height, 0.0); // Flip Y axis
                        svg_points.push(format!("{},{}", svg_x, svg_y));
                    }

                    let points_str = svg_points.join(" ");
                    let polygon_svg = format!(
                        "<g transform=\"translate({},{})\"><polygon fill=\"{}\" fill-opacity=\"0.3\" stroke=\"none\" points=\"{}\"/></g>",
                        margin, margin, fill_color, points_str
                    );

                    axes.add_svg_element(polygon_svg);
                }
            }
        }
    }

    fn render_subgraph_border(&self, axes: &mut Axes, subgraph: &Subgraph) {
        if subgraph.nodes.is_empty() {
            return;
        }

        // Find bounding box of subgraph nodes
        let subgraph_nodes: Vec<&Node> = self
            .nodes
            .iter()
            .filter(|n| subgraph.nodes.contains(&n.id))
            .collect();

        if subgraph_nodes.is_empty() {
            return;
        }

        let min_x = subgraph_nodes
            .iter()
            .map(|n| n.x)
            .fold(f64::INFINITY, f64::min)
            - 0.05;
        let max_x = subgraph_nodes
            .iter()
            .map(|n| n.x)
            .fold(f64::NEG_INFINITY, f64::max)
            + 0.05;
        let min_y = subgraph_nodes
            .iter()
            .map(|n| n.y)
            .fold(f64::INFINITY, f64::min)
            - 0.05;
        let max_y = subgraph_nodes
            .iter()
            .map(|n| n.y)
            .fold(f64::NEG_INFINITY, f64::max)
            + 0.05;

        // Draw border
        let border_color = if let Some(ref color) = subgraph.color {
            match color.as_str() {
                "lightgrey" => Color::GRAY,
                "blue" => Color::BLUE,
                _ => Color::BLACK,
            }
        } else {
            Color::BLACK
        };

        let border_x = vec![min_x, max_x, max_x, min_x, min_x];
        let border_y = vec![min_y, min_y, max_y, max_y, min_y];
        axes.plot(border_x, border_y);

        if let Some(last_plot) = axes.plots.last_mut() {
            last_plot.color = border_color;
            last_plot.line_width = 2.0;
        }
    }

    fn add_arrow(&self, axes: &mut Axes, from: &Node, to: &Node) {
        // Get data ranges from all nodes for coordinate transformation
        let x_coords: Vec<f64> = self.nodes.iter().map(|n| n.x).collect();
        let y_coords: Vec<f64> = self.nodes.iter().map(|n| n.y).collect();
        let (x_min, x_max) = crate::utils::calculate_range(&x_coords);
        let (y_min, y_max) = crate::utils::calculate_range(&y_coords);

        let margin = 60.0;
        let plot_width = 680.0;
        let plot_height = 480.0;

        // Convert node positions to SVG coordinates first
        let from_svg_x = crate::utils::map_range(from.x, x_min, x_max, 0.0, plot_width);
        let from_svg_y = crate::utils::map_range(from.y, y_min, y_max, plot_height, 0.0);
        let to_svg_x = crate::utils::map_range(to.x, x_min, x_max, 0.0, plot_width);
        let to_svg_y = crate::utils::map_range(to.y, y_min, y_max, plot_height, 0.0);

        // Calculate arrow direction in SVG coordinate space
        let dx = to_svg_x - from_svg_x;
        let dy = to_svg_y - from_svg_y;
        let length = (dx * dx + dy * dy).sqrt();

        if length > 0.0 {
            // Arrow size in SVG pixels (fixed size regardless of coordinate transformation)
            let arrow_length = 8.0; // Length in SVG pixels
            let arrow_width = 5.0; // Half-width in SVG pixels

            // Normalize direction vector
            let unit_x = dx / length;
            let unit_y = dy / length;

            // Calculate node radius in SVG coordinates
            let node_radius_logical = self.get_node_radius(&to.shape);
            let node_radius_svg = {
                // Convert radius from logical to SVG coordinates
                let radius_x = crate::utils::map_range(
                    node_radius_logical,
                    0.0,
                    x_max - x_min,
                    0.0,
                    plot_width,
                );
                let radius_y = crate::utils::map_range(
                    node_radius_logical,
                    0.0,
                    y_max - y_min,
                    0.0,
                    plot_height,
                );
                (radius_x + radius_y) / 2.0 // Average for circular approximation
            };

            // Arrow tip position at the edge of the target node
            let tip_x = to_svg_x - node_radius_svg * unit_x;
            let tip_y = to_svg_y - node_radius_svg * unit_y;

            // Calculate perpendicular vector for arrow base
            let perp_x = -unit_y;
            let perp_y = unit_x;

            // Calculate arrow triangle vertices in SVG coordinates
            let base_x = tip_x - arrow_length * unit_x;
            let base_y = tip_y - arrow_length * unit_y;

            let left_x = base_x + arrow_width * perp_x;
            let left_y = base_y + arrow_width * perp_y;

            let right_x = base_x - arrow_width * perp_x;
            let right_y = base_y - arrow_width * perp_y;

            let points = format!(
                "{},{} {},{} {},{} {},{}",
                tip_x, tip_y, left_x, left_y, right_x, right_y, tip_x, tip_y
            );

            let polygon_svg = format!(
                "<g transform=\"translate({},{})\"><polygon fill=\"black\" stroke=\"black\" points=\"{}\"/></g>",
                margin, margin, points
            );

            axes.add_svg_element(polygon_svg);
        }
    }

    // Calculate the start point of an edge at the boundary of the from_node
    fn calculate_edge_start_point(&self, from_node: &Node, to_node: &Node) -> (f64, f64) {
        let dx = to_node.x - from_node.x;
        let dy = to_node.y - from_node.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance == 0.0 {
            return (from_node.x, from_node.y);
        }

        let radius = self.get_node_radius(&from_node.shape);
        let offset_x = (dx / distance) * radius;
        let offset_y = (dy / distance) * radius;

        (from_node.x + offset_x, from_node.y + offset_y)
    }

    // Calculate the end point of an edge at the boundary of the to_node
    fn calculate_edge_end_point(&self, from_node: &Node, to_node: &Node) -> (f64, f64) {
        let dx = from_node.x - to_node.x;
        let dy = from_node.y - to_node.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance == 0.0 {
            return (to_node.x, to_node.y);
        }

        let radius = self.get_node_radius(&to_node.shape);
        let offset_x = (dx / distance) * radius;
        let offset_y = (dy / distance) * radius;

        (to_node.x + offset_x, to_node.y + offset_y)
    }

    // Get the effective radius of a node based on its shape
    fn get_node_radius(&self, shape: &NodeShape) -> f64 {
        match shape {
            NodeShape::Circle => 0.05, // Increased radius for better edge connection
            NodeShape::Rectangle | NodeShape::Msquare => 0.06, // Square edge
            NodeShape::Diamond | NodeShape::Mdiamond => 0.07, // Diamond edge
            NodeShape::Ellipse => 0.08, // Larger radius to match ellipse size
        }
    }

    // Generate a curved edge path using multiple points to simulate bezier curves
    fn generate_curved_edge(
        &self,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        let dx = end_x - start_x;
        let dy = end_y - start_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // For short edges, use straight lines
        if distance < 0.1 {
            return (vec![start_x, end_x], vec![start_y, end_y]);
        }

        // Calculate control points for a smooth curve
        let mid_x = (start_x + end_x) / 2.0;
        let mid_y = (start_y + end_y) / 2.0;

        // Add some curvature perpendicular to the line direction
        let perpendicular_x = -dy / distance;
        let perpendicular_y = dx / distance;
        let curve_strength = distance * 0.2; // Adjust curve strength based on distance

        let control_x = mid_x + perpendicular_x * curve_strength;
        let control_y = mid_y + perpendicular_y * curve_strength;

        // Generate points along the curve
        let num_points = 10;
        let mut x_points = Vec::new();
        let mut y_points = Vec::new();

        for i in 0..=num_points {
            let t = i as f64 / num_points as f64;
            let t2 = t * t;
            let t3 = 1.0 - t;
            let t4 = t3 * t3;

            // Quadratic bezier curve: P = (1-t)²P₀ + 2(1-t)tP₁ + t²P₂
            let x = t4 * start_x + 2.0 * t3 * t * control_x + t2 * end_x;
            let y = t4 * start_y + 2.0 * t3 * t * control_y + t2 * end_y;

            x_points.push(x);
            y_points.push(y);
        }

        (x_points, y_points)
    }
}
