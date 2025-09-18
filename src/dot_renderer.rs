//! DOT graph rendering module with advanced layout algorithms
//! Provides Graphviz-like functionality for rendering DOT graphs

use crate::axes::Axes;
use crate::colors::Color;
use crate::markers::Marker;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub label: Option<String>,
    pub shape: NodeShape,
    pub color: Color,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub enum NodeShape {
    Circle,
    Rectangle,
    Diamond,
    Ellipse,
    Mdiamond,  // Modified diamond shape
    Msquare,   // Modified square shape
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub color: Color,
    pub style: EdgeStyle,
    pub directed: bool,
}

#[derive(Debug, Clone)]
pub enum EdgeStyle {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Debug, Clone)]
pub struct Subgraph {
    pub id: String,
    pub label: Option<String>,
    pub nodes: Vec<String>,
    pub style: Option<String>,
    pub color: Option<String>,
    pub fill_color: Option<String>,
}

#[derive(Debug)]
pub struct DotGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub subgraphs: Vec<Subgraph>,
    pub directed: bool,
    pub layout: LayoutAlgorithm,
}

#[derive(Debug, Clone)]
pub enum LayoutAlgorithm {
    Circular,
    Hierarchical,
    ForceDirected,
    Grid,
}

impl DotGraph {
    pub fn new(directed: bool) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            subgraphs: Vec::new(),
            directed,
            layout: LayoutAlgorithm::Hierarchical,
        }
    }

    pub fn parse_dot(dot_content: &str) -> Result<Self, String> {
        let mut graph = DotGraph::new(true);
        let lines: Vec<&str> = dot_content.lines().collect();
        let mut node_map: HashMap<String, Node> = HashMap::new();
        let mut current_subgraph: Option<Subgraph> = None;
        let mut brace_depth = 0;
        
        // Determine if graph is directed
        for line in &lines {
            let line = line.trim();
            if line.starts_with("digraph") {
                graph.directed = true;
                break;
            } else if line.starts_with("graph") {
                graph.directed = false;
                break;
            }
        }
        
        for line in lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            
            // Handle braces for subgraph tracking
            let old_brace_depth = brace_depth;
            if line.contains('{') {
                brace_depth += line.matches('{').count();
            }
            if line.contains('}') {
                brace_depth -= line.matches('}').count();
                // End of subgraph when we go from depth 2 to 1 (subgraph to main graph)
                if old_brace_depth == 2 && brace_depth == 1 && current_subgraph.is_some() {
                    if let Some(subgraph) = current_subgraph.take() {
                        graph.subgraphs.push(subgraph);
                    }
                }
            }
            
            if line.starts_with("digraph") || line.starts_with("graph") || 
               line == "{" || line == "}" {
                continue;
            }
            
            // Parse subgraph definition
            if line.starts_with("subgraph") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let subgraph_id = parts[1].to_string();

                    current_subgraph = Some(Subgraph {
                        id: subgraph_id,
                        label: None,
                        nodes: Vec::new(),
                        style: None,
                        color: None,
                        fill_color: None,
                    });
                }
                continue;
            }
            
            // Parse subgraph attributes
            if current_subgraph.is_some() {
                if line.starts_with("label") {
                    if let Some(ref mut subgraph) = current_subgraph {
                        let label = line.split('=').nth(1)
                            .unwrap_or("")
                            .trim()
                            .trim_matches('"')
                            .trim_end_matches(';')
                            .to_string();
                        subgraph.label = Some(label);
                    }
                    continue;
                }
                if line.starts_with("style") {
                    if let Some(ref mut subgraph) = current_subgraph {
                        let style = line.split('=').nth(1)
                            .unwrap_or("")
                            .trim()
                            .trim_end_matches(';')
                            .to_string();
                        subgraph.style = Some(style);
                    }
                    continue;
                }
                if line.starts_with("color") {
                    if let Some(ref mut subgraph) = current_subgraph {
                        let color = line.split('=').nth(1)
                            .unwrap_or("")
                            .trim()
                            .trim_end_matches(';')
                            .to_string();
                        subgraph.color = Some(color.clone());
                        // Always set fill_color when color is specified, will be used if style=filled
                        subgraph.fill_color = Some(color);
                    }
                    continue;
                }
                if line.starts_with("node") && line.contains('[') {
                    // Parse node default attributes within subgraph
                    let attrs = Self::parse_attributes(line);
                    if let Some(ref mut subgraph) = current_subgraph {
                        // Store node attributes for this subgraph
                        // Only set fill_color from node attributes if subgraph doesn't already have a color
                        if attrs.get("style").map_or(false, |s| s == "filled") && subgraph.color.is_none() {
                            if let Some(color) = attrs.get("color") {
                                subgraph.fill_color = Some(color.clone());
                            } else {
                                // If no color specified but style is filled, use lightgrey as default
                                subgraph.fill_color = Some("lightgrey".to_string());
                            }
                        }
                    }
                    continue;
                }
            }
            
            if line.contains("->") || line.contains("--") {
                // Parse edge or node sequence
                let separator = if line.contains("->") { "->" } else { "--" };
                let directed = separator == "->";
                
                let parts: Vec<&str> = line.split(separator).collect();
                if parts.len() == 2 {
                    let from = Self::clean_node_name(parts[0]);
                    let to_part = parts[1].trim_end_matches(';');
                    let to = Self::clean_node_name(to_part);
                    
                    // Ensure nodes exist
                    if !node_map.contains_key(&from) {
                        let mut node = Node {
                            id: from.clone(),
                            label: Some(from.clone()),
                            shape: NodeShape::Ellipse,
                            color: Self::get_node_color(&from),
                            x: 0.0,
                            y: 0.0,
                        };
                        node_map.insert(from.clone(), node);
                    }
                    
                    if !node_map.contains_key(&to) {
                        let node = Node {
                            id: to.clone(),
                            label: Some(to.clone()),
                            shape: NodeShape::Ellipse,
                            color: Self::get_node_color(&to),
                            x: 0.0,
                            y: 0.0,
                        };
                        node_map.insert(to.clone(), node);
                    }
                    
                    let edge = Edge {
                        from,
                        to,
                        label: None,
                        color: Color::BLACK,
                        style: EdgeStyle::Solid,
                        directed,
                    };
                    graph.edges.push(edge);
                } else if parts.len() > 2 {
                    // Handle node sequences like "a0 -> a1 -> a2 -> a3;"
                    let nodes: Vec<&str> = parts.iter().map(|s| s.trim().trim_end_matches(';')).collect();

                    
                    for (i, node_name) in nodes.iter().enumerate() {
                        let clean_name = Self::clean_node_name(node_name);
                        
                        // Add node to current subgraph if we're in one
                        if let Some(ref mut subgraph) = current_subgraph {
                            if !subgraph.nodes.contains(&clean_name) {
                                subgraph.nodes.push(clean_name.clone());
                            }
                        }
                        
                        // Ensure node exists
                         if !node_map.contains_key(&clean_name) {
                             let mut node = Node {
                            id: clean_name.clone(),
                            label: Some(clean_name.clone()),
                            shape: NodeShape::Ellipse,
                            color: Self::get_node_color(&clean_name),
                            x: 0.0,
                            y: 0.0,
                        };
                            
                            // Apply subgraph styling
                            if let Some(ref subgraph) = current_subgraph {
                                if let Some(ref fill_color) = subgraph.fill_color {
                                    node.color = Self::parse_color(fill_color);
                                }
                            }
                            
                            node_map.insert(clean_name.clone(), node);
                        }
                        
                        // Create edge to next node
                        if i < nodes.len() - 1 {
                            let next_node = Self::clean_node_name(nodes[i + 1]);
                            let edge = Edge {
                                from: clean_name,
                                to: next_node,
                                label: None,
                                color: Color::BLACK,
                                style: EdgeStyle::Solid,
                                directed,
                            };
                            graph.edges.push(edge);
                        }
                    }
                }
            } else if line.contains('[') && line.contains(']') {
                // Parse node with attributes
                let node_name = Self::clean_node_name(line.split('[').next().unwrap());
                let attrs = Self::parse_attributes(line);
                
                let mut node = Node {
                    id: node_name.clone(),
                    label: Some(node_name.clone()),
                    shape: NodeShape::Ellipse,
                    color: Self::get_node_color(&node_name),
                    x: 0.0,
                    y: 0.0,
                };
                
                // Apply attributes
                if let Some(label) = attrs.get("label") {
                    node.label = Some(label.clone());
                }
                if let Some(shape) = attrs.get("shape") {
                    node.shape = match shape.as_str() {
                        "box" | "rectangle" => NodeShape::Rectangle,
                        "diamond" => NodeShape::Diamond,
                        "ellipse" => NodeShape::Ellipse,
                        "Mdiamond" => NodeShape::Mdiamond,
                        "Msquare" => NodeShape::Msquare,
                        "circle" => NodeShape::Circle,
                        _ => NodeShape::Ellipse, // Default to ellipse like graphviz
                    };
                }
                if let Some(color) = attrs.get("color") {
                    node.color = Self::parse_color(color);
                }
                
                // Add node to current subgraph if we're in one
                if let Some(ref mut subgraph) = current_subgraph {
                    subgraph.nodes.push(node_name.clone());
                    // Apply subgraph node styling
                    if let Some(ref fill_color) = subgraph.fill_color {
                        node.color = Self::parse_color(fill_color);
                    }
                }
                
                node_map.insert(node_name, node);
            } else if line.ends_with(';') && !line.trim().starts_with("label") && !line.trim().starts_with("style") && !line.trim().starts_with("color") {
                // Simple node definition or node sequence

                if line.contains("->") {
                    // Handle node sequences like "a0 -> a1 -> a2 -> a3;"
                    let sequence = line.trim_end_matches(';');
                    let nodes: Vec<&str> = sequence.split("->").map(|s| s.trim()).collect();
                    println!("Processing node sequence: {:?}, current_subgraph: {:?}", nodes, current_subgraph.as_ref().map(|sg| &sg.id));
                    
                    for (i, node_name) in nodes.iter().enumerate() {
                        let clean_name = Self::clean_node_name(node_name);
                        if !clean_name.is_empty() && !node_map.contains_key(&clean_name) {
                            let mut node = Node {
                            id: clean_name.clone(),
                            label: Some(clean_name.clone()),
                            shape: NodeShape::Ellipse,
                            color: Self::get_node_color(&clean_name),
                            x: 0.0,
                            y: 0.0,
                        };
                            
                            // Add to current subgraph and apply styling
                            if let Some(ref mut subgraph) = current_subgraph {
                                subgraph.nodes.push(clean_name.clone());
                                if let Some(ref fill_color) = subgraph.fill_color {
                                    node.color = Self::parse_color(fill_color);
                                }
                            }
                            
                            node_map.insert(clean_name.clone(), node);
                        }
                        
                        // Create edges between consecutive nodes
                        if i > 0 {
                            let from = Self::clean_node_name(nodes[i-1]);
                            let to = Self::clean_node_name(node_name);
                            let edge = Edge {
                                from,
                                to,
                                label: None,
                                color: Color::BLACK,
                                style: EdgeStyle::Solid,
                                directed: true,
                            };
                            graph.edges.push(edge);
                        }
                    }
                } else {
                    // Simple single node
                    let node_name = Self::clean_node_name(line.trim_end_matches(';'));
                    if !node_name.is_empty() && !node_map.contains_key(&node_name) {
                        let mut node = Node {
                            id: node_name.clone(),
                            label: Some(node_name.clone()),
                            shape: NodeShape::Ellipse,
                            color: Self::get_node_color(&node_name),
                            x: 0.0,
                            y: 0.0,
                        };
                        
                        // Add to current subgraph if we're in one
                        if let Some(ref mut subgraph) = current_subgraph {
                            subgraph.nodes.push(node_name.clone());
                        }
                        
                        node_map.insert(node_name, node);
                    }
                }
            }
        }
        
        // Add any remaining subgraph
        if let Some(subgraph) = current_subgraph {
            graph.subgraphs.push(subgraph);
        }
        
        graph.nodes = node_map.into_values().collect();
        
        if graph.nodes.is_empty() {
            return Err("No nodes found in DOT content".to_string());
        }
        
        Ok(graph)
    }
    
    fn clean_node_name(name: &str) -> String {
        name.trim().trim_matches('"').to_string()
    }
    
    fn get_node_color(_node_id: &str) -> Color {
        // Default node color is black
        Color::BLACK
    }
    
    fn parse_color(color_str: &str) -> Color {
        match color_str.to_lowercase().as_str() {
            "red" => Color::RED,
            "blue" => Color::BLUE,
            "green" => Color::GREEN,
            "white" => Color::WHITE,
            "black" => Color::BLACK,
            "lightgrey" | "lightgray" => Color::GRAY,
            _ => Color::BLACK,
        }
    }
    
    fn parse_attributes(line: &str) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        if let Some(start) = line.find('[') {
            if let Some(end) = line.find(']') {
                let attr_str = &line[start+1..end];
                for pair in attr_str.split(',') {
                    let parts: Vec<&str> = pair.split('=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim().to_string();
                        let value = parts[1].trim().trim_matches('"').to_string();
                        attrs.insert(key, value);
                    }
                }
            }
        }
        attrs
    }
    
    pub fn apply_layout(&mut self) {
        match self.layout {
            LayoutAlgorithm::Circular => self.apply_circular_layout(),
            LayoutAlgorithm::Hierarchical => self.apply_hierarchical_layout(),
            LayoutAlgorithm::ForceDirected => self.apply_force_directed_layout(),
            LayoutAlgorithm::Grid => self.apply_grid_layout(),
        }
    }
    
    fn apply_circular_layout(&mut self) {
        let node_count = self.nodes.len();
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node_count == 1 {
                node.x = 0.5;
                node.y = 0.5;
            } else {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / node_count as f64;
                node.x = 0.5 + 0.35 * angle.cos();
                node.y = 0.5 + 0.35 * angle.sin();
            }
        }
    }
    
    fn apply_hierarchical_layout(&mut self) {
        // Enhanced hierarchical layout with subgraph awareness
        
        // First, handle special nodes (start/end)
        let mut special_nodes = Vec::new();
        let mut subgraph_nodes = HashMap::new();
        let mut unassigned_nodes = Vec::new();
        
        // Categorize nodes
        for node in &self.nodes {
            let mut assigned = false;
            
            // Check if node belongs to any subgraph
            for subgraph in &self.subgraphs {
                if subgraph.nodes.contains(&node.id) {
                    subgraph_nodes.entry(subgraph.id.clone())
                        .or_insert_with(Vec::new)
                        .push(node.id.clone());
                    assigned = true;
                    break;
                }
            }
            
            if !assigned {
                // Check for special nodes (start/end)
                if node.id == "start" || node.id == "end" {
                    special_nodes.push(node.id.clone());
                } else {
                    unassigned_nodes.push(node.id.clone());
                }
            }
        }
        
        // Position special nodes
        if let Some(start_node) = self.nodes.iter_mut().find(|n| n.id == "start") {
            start_node.x = 0.5;
            start_node.y = 0.95; // Top (will be flipped in SVG coordinate system)
        }
        if let Some(end_node) = self.nodes.iter_mut().find(|n| n.id == "end") {
            end_node.x = 0.5;
            end_node.y = 0.05; // Bottom (will be flipped in SVG coordinate system)
        }
        
        // Position subgraphs side by side
        let subgraph_count = self.subgraphs.len();
        if subgraph_count > 0 {
            for (sg_idx, subgraph) in self.subgraphs.iter().enumerate() {
                let sg_center_x = if subgraph_count == 1 {
                    0.5
                } else {
                    0.15 + 0.7 * sg_idx as f64 / (subgraph_count - 1) as f64
                };
                
                // Position nodes within subgraph vertically
                if let Some(sg_nodes) = subgraph_nodes.get(&subgraph.id) {
                    let node_count = sg_nodes.len();
                    for (node_idx, node_id) in sg_nodes.iter().enumerate() {
                        if let Some(node) = self.nodes.iter_mut().find(|n| &n.id == node_id) {
                            node.x = sg_center_x;
                            
                            // Arrange nodes vertically within subgraph
                            if node_count == 1 {
                                node.y = 0.5;
                            } else {
                                // Leave space for start/end nodes
                                node.y = 0.2 + 0.6 * node_idx as f64 / (node_count - 1) as f64;
                            }
                        }
                    }
                }
            }
        }
        
        // Position any unassigned nodes
        if !unassigned_nodes.is_empty() {
            let unassigned_count = unassigned_nodes.len();
            for (idx, node_id) in unassigned_nodes.iter().enumerate() {
                if let Some(node) = self.nodes.iter_mut().find(|n| &n.id == node_id) {
                    node.x = if unassigned_count == 1 {
                        0.5
                    } else {
                        0.1 + 0.8 * idx as f64 / (unassigned_count - 1) as f64
                    };
                    node.y = 0.5;
                }
            }
        }
    }
    
    fn apply_force_directed_layout(&mut self) {
        // Simplified force-directed layout
        // For now, use a spring-based approach similar to circular but with edge attraction
        let node_count = self.nodes.len();
        
        // Start with circular layout
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node_count == 1 {
                node.x = 0.5;
                node.y = 0.5;
            } else {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / node_count as f64;
                node.x = 0.5 + 0.25 * angle.cos();
                node.y = 0.5 + 0.25 * angle.sin();
            }
        }
        
        // Adjust positions based on connectivity
        for _ in 0..10 {
            let mut adjustments: Vec<(f64, f64)> = vec![(0.0, 0.0); node_count];
            
            // Pull connected nodes closer
            for edge in &self.edges {
                if let (Some(from_idx), Some(to_idx)) = (
                    self.nodes.iter().position(|n| n.id == edge.from),
                    self.nodes.iter().position(|n| n.id == edge.to)
                ) {
                    let from_x = self.nodes[from_idx].x;
                    let from_y = self.nodes[from_idx].y;
                    let to_x = self.nodes[to_idx].x;
                    let to_y = self.nodes[to_idx].y;
                    
                    let dx = to_x - from_x;
                    let dy = to_y - from_y;
                    let distance = (dx*dx + dy*dy).sqrt();
                    
                    if distance > 0.0 {
                        let pull_strength = 0.02;
                        adjustments[from_idx].0 += dx * pull_strength;
                        adjustments[from_idx].1 += dy * pull_strength;
                        adjustments[to_idx].0 -= dx * pull_strength;
                        adjustments[to_idx].1 -= dy * pull_strength;
                    }
                }
            }
            
            // Apply adjustments
            for (i, node) in self.nodes.iter_mut().enumerate() {
                node.x += adjustments[i].0;
                node.y += adjustments[i].1;
                
                // Keep nodes within bounds
                node.x = node.x.max(0.1).min(0.9);
                node.y = node.y.max(0.1).min(0.9);
            }
        }
    }
    
    fn apply_grid_layout(&mut self) {
        let node_count = self.nodes.len();
        let cols = (node_count as f64).sqrt().ceil() as usize;
        let rows = (node_count + cols - 1) / cols;
        
        for (i, node) in self.nodes.iter_mut().enumerate() {
            let col = i % cols;
            let row = i / cols;
            
            node.x = if cols == 1 { 0.5 } else { 0.1 + 0.8 * col as f64 / (cols - 1) as f64 };
            node.y = if rows == 1 { 0.5 } else { 0.1 + 0.8 * row as f64 / (rows - 1) as f64 };
        }
    }
    
    pub fn render_to_axes(&self, axes: &mut Axes) {
        // Render subgraph backgrounds first
        for subgraph in &self.subgraphs {
            self.render_subgraph_background(axes, subgraph);
        }
        
        // Render edges (so they appear behind nodes but above subgraph backgrounds)
        for edge in &self.edges {
            if let (Some(from_node), Some(to_node)) = (
                self.nodes.iter().find(|n| n.id == edge.from),
                self.nodes.iter().find(|n| n.id == edge.to)
            ) {
                // Calculate edge endpoints at node boundaries instead of centers
                let (start_x, start_y) = self.calculate_edge_start_point(from_node, to_node);
                let (end_x, end_y) = self.calculate_edge_end_point(from_node, to_node);
                
                // Generate curved path for better visual appearance
                let (x_line, y_line) = self.generate_curved_edge(start_x, start_y, end_x, end_y);
                axes.plot(&x_line, &y_line);
                
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
            
            axes.scatter(&x_coords, &y_coords);
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
        let subgraph_nodes: Vec<&Node> = self.nodes.iter()
            .filter(|n| subgraph.nodes.contains(&n.id))
            .collect();
            
        if subgraph_nodes.is_empty() {
            return;
        }
        
        let min_x = subgraph_nodes.iter().map(|n| n.x).fold(f64::INFINITY, f64::min) - 0.05;
        let max_x = subgraph_nodes.iter().map(|n| n.x).fold(f64::NEG_INFINITY, f64::max) + 0.05;
        let min_y = subgraph_nodes.iter().map(|n| n.y).fold(f64::INFINITY, f64::min) - 0.05;
        let max_y = subgraph_nodes.iter().map(|n| n.y).fold(f64::NEG_INFINITY, f64::max) + 0.05;
        
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
                axes.plot(&border_x, &border_y);
                
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
                        let svg_x = crate::utils::map_range(border_x[i], x_min, x_max, 0.0, plot_width);
                        let svg_y = crate::utils::map_range(border_y[i], y_min, y_max, plot_height, 0.0); // Flip Y axis
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
        let subgraph_nodes: Vec<&Node> = self.nodes.iter()
            .filter(|n| subgraph.nodes.contains(&n.id))
            .collect();
            
        if subgraph_nodes.is_empty() {
            return;
        }
        
        let min_x = subgraph_nodes.iter().map(|n| n.x).fold(f64::INFINITY, f64::min) - 0.05;
        let max_x = subgraph_nodes.iter().map(|n| n.x).fold(f64::NEG_INFINITY, f64::max) + 0.05;
        let min_y = subgraph_nodes.iter().map(|n| n.y).fold(f64::INFINITY, f64::min) - 0.05;
        let max_y = subgraph_nodes.iter().map(|n| n.y).fold(f64::NEG_INFINITY, f64::max) + 0.05;
        
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
        axes.plot(&border_x, &border_y);
        
        if let Some(last_plot) = axes.plots.last_mut() {
            last_plot.color = border_color;
            last_plot.line_width = 2.0;
        }
    }
    
    fn add_arrow(&self, axes: &mut Axes, from: &Node, to: &Node) {
        // Calculate arrow position and direction
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let length = (dx*dx + dy*dy).sqrt();
        
        if length > 0.0 {
            let arrow_length = 0.035; // Increased to match graphviz arrow size
            let arrow_width = 0.025; // Increased width to match graphviz arrow size
            
            // Normalize direction vector
            let unit_x = dx / length;
            let unit_y = dy / length;
            
            // Get the radius of the target node
            let node_radius = self.get_node_radius(&to.shape);
            
            // Arrow tip position at the edge of the target node
            let tip_x = to.x - node_radius * unit_x;
            let tip_y = to.y - node_radius * unit_y;
            
            // Calculate perpendicular vector for arrow base
            let perp_x = -unit_y;
            let perp_y = unit_x;
            
            // Calculate arrow triangle vertices
            let base_x = tip_x - arrow_length * unit_x;
            let base_y = tip_y - arrow_length * unit_y;
            
            let left_x = base_x + arrow_width * perp_x;
            let left_y = base_y + arrow_width * perp_y;
            
            let right_x = base_x - arrow_width * perp_x;
            let right_y = base_y - arrow_width * perp_y;
            
            // Use the same coordinate transformation as the plot lines
            // Get data ranges from all nodes
            let x_coords: Vec<f64> = self.nodes.iter().map(|n| n.x).collect();
            let y_coords: Vec<f64> = self.nodes.iter().map(|n| n.y).collect();
            let (x_min, x_max) = crate::utils::calculate_range(&x_coords);
            let (y_min, y_max) = crate::utils::calculate_range(&y_coords);
            
            let margin = 60.0;
            let plot_width = 680.0;
            let plot_height = 480.0;
            
            // Convert arrow coordinates using the same map_range function as plot lines
            let svg_tip_x = crate::utils::map_range(tip_x, x_min, x_max, 0.0, plot_width);
            let svg_tip_y = crate::utils::map_range(tip_y, y_min, y_max, plot_height, 0.0); // Flip Y axis
            let svg_left_x = crate::utils::map_range(left_x, x_min, x_max, 0.0, plot_width);
            let svg_left_y = crate::utils::map_range(left_y, y_min, y_max, plot_height, 0.0);
            let svg_right_x = crate::utils::map_range(right_x, x_min, x_max, 0.0, plot_width);
            let svg_right_y = crate::utils::map_range(right_y, y_min, y_max, plot_height, 0.0);
            
            let points = format!("{},{} {},{} {},{} {},{}",
                               svg_tip_x, svg_tip_y,
                               svg_left_x, svg_left_y,
                               svg_right_x, svg_right_y,
                               svg_tip_x, svg_tip_y);
            
            let polygon_svg = format!(
                "<g transform=\"translate({},{})\"><polygon fill=\"black\" stroke=\"black\" points=\"{}\"/></g>",
                margin, margin, points
            );
            
            axes.add_svg_element(polygon_svg);
        }
    }
    
    pub fn set_layout(&mut self, layout: LayoutAlgorithm) {
        self.layout = layout;
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
    fn generate_curved_edge(&self, start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> (Vec<f64>, Vec<f64>) {
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