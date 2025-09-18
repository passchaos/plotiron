//! Layout algorithms for DOT graphs

use super::types::*;
use std::collections::{HashMap, HashSet, VecDeque};

impl DotGraph {
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
        // Improved hierarchical layout with cycle handling
        
        // Step 1: Build adjacency lists
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        
        // Initialize
        for node in &self.nodes {
            adj_list.insert(node.id.clone(), Vec::new());
            in_degree.insert(node.id.clone(), 0);
        }
        
        // Build graph
        for edge in &self.edges {
            adj_list.entry(edge.from.clone()).or_insert_with(Vec::new).push(edge.to.clone());
            *in_degree.entry(edge.to.clone()).or_insert(0) += 1;
        }
        
        // Step 2: Modified topological sort to handle cycles
        let mut layers: Vec<Vec<String>> = Vec::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut current_in_degree = in_degree.clone();
        let mut processed: HashSet<String> = HashSet::new();
        
        // Find all nodes with in-degree 0
        for (node_id, degree) in &current_in_degree {
            if *degree == 0 {
                queue.push_back(node_id.clone());
            }
        }
        
        while !queue.is_empty() {
            let mut current_layer = Vec::new();
            let layer_size = queue.len();
            
            for _ in 0..layer_size {
                if let Some(node_id) = queue.pop_front() {
                    current_layer.push(node_id.clone());
                    processed.insert(node_id.clone());
                    
                    // Process all neighbors
                    if let Some(neighbors) = adj_list.get(&node_id) {
                        for neighbor in neighbors {
                            if let Some(degree) = current_in_degree.get_mut(neighbor) {
                                *degree -= 1;
                                if *degree == 0 {
                                    queue.push_back(neighbor.clone());
                                }
                            }
                        }
                    }
                }
            }
            
            if !current_layer.is_empty() {
                layers.push(current_layer);
            }
        }
        
        // Step 3: Handle remaining nodes (those in cycles) with better layer assignment
        let all_nodes: HashSet<String> = self.nodes.iter().map(|n| n.id.clone()).collect();
        let remaining_nodes: Vec<String> = all_nodes.difference(&processed).cloned().collect();
        
        if !remaining_nodes.is_empty() {
            // Try to assign remaining nodes to appropriate layers based on their connections
            for node_id in remaining_nodes {
                let mut best_layer = layers.len(); // Default to new layer
                
                // Find the best layer based on incoming edges from already processed nodes
                for (layer_idx, layer) in layers.iter().enumerate() {
                    for processed_node in layer {
                        // Check if there's an edge from processed_node to this remaining node
                        for edge in &self.edges {
                            if edge.from == *processed_node && edge.to == node_id {
                                best_layer = (layer_idx + 1).min(layers.len());
                                break;
                            }
                        }
                    }
                }
                
                // Add to the determined layer
                if best_layer >= layers.len() {
                    layers.push(vec![node_id]);
                } else {
                    layers[best_layer].push(node_id);
                }
            }
        }
        
        // Step 4: Position nodes based on layers
        self.position_nodes_by_layers(&layers);
    }
    
    fn position_nodes_by_layers(&mut self, layers: &[Vec<String>]) {
        let layer_count = layers.len();
        if layer_count == 0 {
            return;
        }
        
        // First, determine the fixed x positions for each subgraph
        let mut subgraph_x_positions: HashMap<String, f64> = HashMap::new();
        let unique_subgraphs: HashSet<String> = self.nodes.iter()
            .map(|n| self.get_node_subgraph(&n.id))
            .collect();
        
        // Separate real subgraphs from independent nodes
        let real_subgraphs: Vec<String> = unique_subgraphs.iter()
            .filter(|&s| s != "_none")
            .cloned()
            .collect();
        
        // Position real subgraphs
        for (i, subgraph_id) in real_subgraphs.iter().enumerate() {
            let x_pos = if real_subgraphs.len() == 1 {
                0.5
            } else {
                0.25 + (i as f64 / (real_subgraphs.len() - 1) as f64) * 0.5
            };
            subgraph_x_positions.insert(subgraph_id.clone(), x_pos);
        }
        
        // Independent nodes (not in any subgraph) get center position
        subgraph_x_positions.insert("_none".to_string(), 0.5);
        
        for (layer_index, layer) in layers.iter().enumerate() {
            // Calculate y position (top to bottom)
            let y = if layer_count == 1 {
                0.5
            } else {
                0.9 - (layer_index as f64 / (layer_count - 1) as f64) * 0.8
            };
            
            // Group nodes by subgraph
            let mut subgraph_groups: HashMap<String, Vec<String>> = HashMap::new();
            
            for node_id in layer {
                let subgraph = self.get_node_subgraph(node_id);
                subgraph_groups.entry(subgraph).or_insert_with(Vec::new).push(node_id.clone());
            }
            
            // Position nodes within each subgraph column
            for (subgraph_id, group_nodes) in subgraph_groups {
                let base_x = subgraph_x_positions.get(&subgraph_id).copied().unwrap_or(0.5);
                let group_count = group_nodes.len();
                
                for (node_index, node_id) in group_nodes.iter().enumerate() {
                    let x = if group_count == 1 {
                        base_x
                    } else {
                        // Spread nodes slightly around the base position
                        let offset = (node_index as f64 / (group_count - 1) as f64 - 0.5) * 0.15;
                        (base_x + offset).max(0.05).min(0.95)
                    };
                    
                    if let Some(node) = self.nodes.iter_mut().find(|n| n.id == *node_id) {
                        node.x = x;
                        node.y = y;
                    }
                }
            }
        }
    }
    
    fn get_node_subgraph(&self, node_id: &str) -> String {
        for subgraph in &self.subgraphs {
            if subgraph.nodes.contains(&node_id.to_string()) {
                return subgraph.id.clone();
            }
        }
        "_none".to_string()
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
}