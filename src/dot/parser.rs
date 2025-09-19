//! DOT file parsing functionality

use crate::colors::Color;
use super::types::*;
use std::collections::HashMap;

impl DotGraph {
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
                        let node = Node {
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
                        let node = Node {
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
}