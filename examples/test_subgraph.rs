use plotiron::dot_renderer::DotGraph;
use plotiron::figure::Figure;
use std::fs;

fn main() {
    println!("Testing subgraph parsing...");
    
    // Read the test_subgraph.dot file
    let dot_content = fs::read_to_string("output/test_subgraph.dot")
        .expect("Failed to read test_subgraph.dot");
    
    println!("DOT content:");
    println!("{}", dot_content);
    
    // Parse the DOT content
    let mut graph = DotGraph::parse_dot(&dot_content)
        .expect("Failed to parse DOT content");
    
    println!("\nParsed graph:");
    println!("Nodes: {}", graph.nodes.len());
    for node in &graph.nodes {
        println!("  - {} at ({:.2}, {:.2})", node.id, node.x, node.y);
    }
    
    println!("\nEdges: {}", graph.edges.len());
    for edge in &graph.edges {
        println!("  - {} -> {}", edge.from, edge.to);
    }
    
    println!("\nSubgraphs: {}", graph.subgraphs.len());
    for subgraph in &graph.subgraphs {
        println!("  - {} ({:?}): {:?}", subgraph.id, subgraph.label, subgraph.nodes);
        println!("    Style: {:?}, Color: {:?}", subgraph.style, subgraph.color);
    }
    
    // Apply layout
    graph.apply_layout();
    
    println!("\nAfter layout:");
    for node in &graph.nodes {
        println!("  - {} at ({:.2}, {:.2})", node.id, node.x, node.y);
    }
    
    // Create a figure and render
    let mut figure = Figure::new();
    let mut axes = figure.add_subplot();
    
    // 关闭坐标轴和背景网格
    axes.show_x_axis(false)
        .show_y_axis(false)
        .grid(false);
    
    graph.render_to_axes(&mut axes);
    
    // Save the result
    let svg_content = figure.to_svg();
    fs::write("output/test_subgraph_parsed.svg", svg_content)
        .expect("Failed to save SVG");
    
    println!("\nSubgraph test completed. Check output/test_subgraph_parsed.svg");
}