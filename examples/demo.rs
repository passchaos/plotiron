use plotiron::*;

fn main() {
    // Create some sample data
    let x: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
    let y1: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
    let y2: Vec<f64> = x.iter().map(|&x| x.cos()).collect();
    let y3: Vec<f64> = x.iter().map(|&x| (x * 0.5).sin() * 0.5).collect();

    // Example 1: Simple line plot
    println!("Creating simple line plot...");
    let mut fig1 = figure();
    fig1.add_subplot()
        .plot(&x, &y1)
        .set_title("Sine Function")
        .set_xlabel("x")
        .set_ylabel("sin(x)");
    
    let svg_content = fig1.to_svg();
    println!("Sine plot SVG content:\n{}", svg_content);

    // Example 2: Multi-line plot
    println!("Creating multi-line plot...");
    let mut fig2 = figure_with_size(900.0, 600.0);
    fig2.add_subplot()
        .plot(&x, &y1)
        .plot(&x, &y2)
        .plot(&x, &y3)
        .set_title("Trigonometric Functions Comparison")
        .set_xlabel("x")
        .set_ylabel("y")
        .legend(true);
    
    let svg_content = fig2.to_svg();
    println!("Multi-line plot SVG content:\n{}", svg_content);

    // Example 3: Scatter plot
    println!("Creating scatter plot...");
    let scatter_x: Vec<f64> = (0..50).map(|_| rand::random::<f64>() * 10.0).collect();
    let scatter_y: Vec<f64> = scatter_x.iter().map(|&x| x * 2.0 + rand::random::<f64>() * 5.0).collect();
    
    let mut fig3 = figure();
    fig3.add_subplot()
        .scatter(&scatter_x, &scatter_y)
        .set_title("Random Scatter Plot")
        .set_xlabel("X Value")
        .set_ylabel("Y Value");
    
    let svg_content = fig3.to_svg();
    println!("Scatter plot SVG content:\n{}", svg_content);

    // Example 4: Bar chart
    println!("Creating bar chart...");
    let categories: Vec<f64> = (1..=5).map(|i| i as f64).collect();
    let values = vec![23.0, 45.0, 56.0, 78.0, 32.0];
    
    let mut fig4 = figure();
    fig4.add_subplot()
        .bar(&categories, &values)
        .set_title("Sales Data")
        .set_xlabel("Month")
        .set_ylabel("Sales Amount");
    
    let svg_content = fig4.to_svg();
    println!("Bar chart SVG content:\n{}", svg_content);
    
    // Example 5: Graph from DOT markup
    println!("Creating graph from DOT markup...");
    let dot_content = r#"
        digraph G {
            A -> B;
            B -> C;
            C -> D;
            D -> A;
            A -> C;
        }
    "#;
    
    match Figure::from_dot(dot_content) {
        Ok(fig5) => {
            let svg_content = fig5.to_svg();
            println!("DOT graph SVG content:\n{}", svg_content);
        }
        Err(e) => {
            println!("Error parsing DOT content: {}", e);
        }
    }
    
    // Example 6: Chart with disabled axes
    println!("Creating chart with disabled axes...");
    let mut fig6 = figure();
    fig6.add_subplot()
        .plot(&x, &y1)
        .set_title("Chart without X and Y axes")
        .show_x_axis(false)
        .show_y_axis(false);
    
    let svg_content = fig6.to_svg();
    println!("Chart without axes SVG content:\n{}", svg_content);
    
    println!("All charts SVG content generated successfully!");
    
    // Example 7: Combined chart with sine/cosine and DOT graph
    println!("Creating combined chart with sine/cosine and DOT graph...");
    let mut fig7 = figure();
    
    // First subplot: sine and cosine functions
    fig7.add_subplot()
        .plot(&x, &y1)
        .plot(&x, &y2)
        .set_title("Sine and Cosine Functions")
        .set_xlabel("X")
        .set_ylabel("Y");
    
    // Second subplot: DOT graph without axes
    let dot_content = "digraph G {\n    A -> B;\n    B -> C;\n    C -> D;\n    A -> D;\n}";
    println!("DOT content: {}", dot_content);
    match fig7.add_dot_subplot(dot_content) {
        Ok(dot_axes) => {
            println!("DOT subplot added successfully with {} plots", dot_axes.plots.len());
            dot_axes.set_title("DOT Graph (No Axes)")
                .show_x_axis(false)
                .show_y_axis(false);
        }
        Err(e) => println!("Error creating DOT graph: {}", e),
    }
    
    println!("Figure has {} subplots total", fig7.subplots.len());
    
    let combined_svg = fig7.to_svg();
    println!("Combined chart SVG content:\n{}", combined_svg);
    
    // Save combined chart to file
    std::fs::write("output/combined_chart.svg", &combined_svg).expect("Failed to write combined chart to file");
    println!("Combined chart saved to combined_chart.svg");
    
    // Example 8: Chart without background grid
    println!("Creating chart without background grid...");
    let mut fig8 = figure();
    
    fig8.add_subplot()
        .plot(&x, &y1)
        .plot(&x, &y2)
        .set_title("Sine and Cosine Functions (No Grid)")
        .set_xlabel("X")
        .set_ylabel("Y")
        .grid(false);  // Disable background grid
    
    let no_grid_svg = fig8.to_svg();
    println!("No grid chart SVG content:\n{}", no_grid_svg);
    
    // Save no grid chart to file
    std::fs::write("output/no_grid_chart.svg", &no_grid_svg).expect("Failed to write no grid chart to file");
    println!("No grid chart saved to no_grid_chart.svg");
    
    // Example 9: Advanced DOT graph layouts
    println!("Creating advanced DOT graphs with different layouts...");
    
    // Complex DOT graph for testing with subgraphs and new shapes
    let complex_dot = r#"digraph G { 
 
       subgraph cluster_0 { 
         style=filled; 
         color=lightgrey; 
         node [style=filled,color=white]; 
         a0 -> a1 -> a2 -> a3; 
         label = "process #1"; 
       } 
     
       subgraph cluster_1 { 
         node [style=filled]; 
         b0 -> b1 -> b2 -> b3; 
         label = "process #2"; 
         color=blue 
       } 
       start -> a0; 
       start -> b0; 
       a1 -> b3; 
       b2 -> a3; 
       a3 -> a0; 
       a3 -> end; 
       b3 -> end; 
     
       start [shape=Mdiamond]; 
       end [shape=Msquare]; 
     }"#;
    
    // Hierarchical layout
    let mut fig9 = figure();
    match fig9.add_dot_subplot_with_layout(complex_dot, plotiron::dot_renderer::LayoutAlgorithm::Hierarchical) {
        Ok(axes) => {
            axes.set_title("Hierarchical Layout")
                .show_x_axis(false)
                .show_y_axis(false)
                .grid(false);
        }
        Err(e) => println!("Error creating hierarchical DOT graph: {}", e),
    }
    
    // Force-directed layout
    match fig9.add_dot_subplot_with_layout(complex_dot, plotiron::dot_renderer::LayoutAlgorithm::ForceDirected) {
        Ok(axes) => {
            axes.set_title("Force-Directed Layout")
                .show_x_axis(false)
                .show_y_axis(false)
                .grid(false);
        }
        Err(e) => println!("Error creating force-directed DOT graph: {}", e),
    }
    
    // Circular layout
    match fig9.add_dot_subplot_with_layout(complex_dot, plotiron::dot_renderer::LayoutAlgorithm::Circular) {
        Ok(axes) => {
            axes.set_title("Circular Layout")
                .show_x_axis(false)
                .show_y_axis(false)
                .grid(false);
        }
        Err(e) => println!("Error creating circular DOT graph: {}", e),
    }
    
    // Grid layout
    match fig9.add_dot_subplot_with_layout(complex_dot, plotiron::dot_renderer::LayoutAlgorithm::Grid) {
        Ok(axes) => {
            axes.set_title("Grid Layout")
                .show_x_axis(false)
                .show_y_axis(false)
                .grid(false);
        }
        Err(e) => println!("Error creating grid DOT graph: {}", e),
    }
    
    let advanced_dot_svg = fig9.to_svg();
    std::fs::write("output/advanced_dot_layouts.svg", &advanced_dot_svg).expect("Failed to write advanced DOT layouts to file");
    println!("Advanced DOT layouts saved to advanced_dot_layouts.svg");
    
    println!("PlotIron - Rust plotting library demo completed");
    println!("Generated SVG content for all chart types including advanced DOT graphs with multiple layouts, axis control, combined charts, and grid control");
}