//! DOT graph data types and structures

use crate::colors::Color;


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

    pub fn set_layout(&mut self, layout: LayoutAlgorithm) {
        self.layout = layout;
    }
}