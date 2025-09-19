use plotiron::prelude::*;

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
        .plot(x.as_slice(), y1.clone())
        .set_title("Sine Function")
        .set_xlabel("x")
        .set_ylabel("sin(x)");

    let svg_content = fig1.to_svg();
    println!("Sine plot SVG content:\n{}", svg_content);

    // Example 2: Multi-line plot
    println!("Creating multi-line plot...");
    let mut fig2 = figure_with_size(900.0, 600.0);
    fig2.add_subplot()
        .plot(x.as_slice(), y1.as_slice())
        .plot(x.as_slice(), y2.clone())
        .plot(x.as_slice(), y3)
        .set_title("Trigonometric Functions Comparison")
        .set_xlabel("x")
        .set_ylabel("y")
        .legend(true);

    let svg_content = fig2.to_svg();
    println!("Multi-line plot SVG content:\n{}", svg_content);

    // Example 3: Scatter plot
    println!("Creating scatter plot...");
    let scatter_x: Vec<f64> = (0..50).map(|_| rand::random::<f64>() * 10.0).collect();
    let scatter_y: Vec<f64> = scatter_x
        .iter()
        .map(|&x| x * 2.0 + rand::random::<f64>() * 5.0)
        .collect();

    let mut fig3 = figure();
    fig3.add_subplot()
        .scatter(scatter_x, scatter_y)
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
        .bar(categories, values)
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
        .plot(x.clone(), y1.clone())
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
        .plot(x.as_slice(), y1.clone())
        .plot(x.as_slice(), y2.clone())
        .set_title("Sine and Cosine Functions")
        .set_xlabel("X")
        .set_ylabel("Y");

    // Second subplot: DOT graph without axes
    let dot_content = "digraph G {\n    A -> B;\n    B -> C;\n    C -> D;\n    A -> D;\n}";
    println!("DOT content: {}", dot_content);
    match fig7.add_dot_subplot(dot_content) {
        Ok(dot_axes) => {
            println!(
                "DOT subplot added successfully with {} plots",
                dot_axes.plots.len()
            );
            dot_axes
                .set_title("DOT Graph (No Axes)")
                .show_x_axis(false)
                .show_y_axis(false);
        }
        Err(e) => println!("Error creating DOT graph: {}", e),
    }

    println!("Figure has {} subplots total", fig7.subplots.len());

    let combined_svg = fig7.to_svg();
    println!("Combined chart SVG content:\n{}", combined_svg);

    // Save combined chart to file
    std::fs::write("output/combined_chart.svg", &combined_svg)
        .expect("Failed to write combined chart to file");
    println!("Combined chart saved to combined_chart.svg");

    // Example 8: Chart without background grid
    println!("Creating chart without background grid...");
    let mut fig8 = figure();

    fig8.add_subplot()
        .plot(x.as_slice(), y1)
        .plot(x.as_slice(), y2)
        .set_title("Sine and Cosine Functions (No Grid)")
        .set_xlabel("X")
        .set_ylabel("Y")
        .grid(false); // Disable background grid

    let no_grid_svg = fig8.to_svg();
    println!("No grid chart SVG content:\n{}", no_grid_svg);

    // Save no grid chart to file
    std::fs::write("output/no_grid_chart.svg", &no_grid_svg)
        .expect("Failed to write no grid chart to file");
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
    match fig9
        .add_dot_subplot_with_layout(complex_dot, plotiron::dot::LayoutAlgorithm::Hierarchical)
    {
        Ok(axes) => {
            axes.set_title("Hierarchical Layout")
                .show_x_axis(false)
                .show_y_axis(false)
                .grid(false);
        }
        Err(e) => println!("Error creating hierarchical DOT graph: {}", e),
    }

    // Force-directed layout
    match fig9
        .add_dot_subplot_with_layout(complex_dot, plotiron::dot::LayoutAlgorithm::ForceDirected)
    {
        Ok(axes) => {
            axes.set_title("Force-Directed Layout")
                .show_x_axis(false)
                .show_y_axis(false)
                .grid(false);
        }
        Err(e) => println!("Error creating force-directed DOT graph: {}", e),
    }

    // Circular layout
    match fig9.add_dot_subplot_with_layout(complex_dot, plotiron::dot::LayoutAlgorithm::Circular) {
        Ok(axes) => {
            axes.set_title("Circular Layout")
                .show_x_axis(false)
                .show_y_axis(false)
                .grid(false);
        }
        Err(e) => println!("Error creating circular DOT graph: {}", e),
    }

    // Grid layout
    match fig9.add_dot_subplot_with_layout(complex_dot, plotiron::dot::LayoutAlgorithm::Grid) {
        Ok(axes) => {
            axes.set_title("Grid Layout")
                .show_x_axis(false)
                .show_y_axis(false)
                .grid(false);
        }
        Err(e) => println!("Error creating grid DOT graph: {}", e),
    }

    let advanced_dot_svg = fig9.to_svg();
    std::fs::write("output/advanced_dot_layouts.svg", &advanced_dot_svg)
        .expect("Failed to write advanced DOT layouts to file");
    println!("Advanced DOT layouts saved to advanced_dot_layouts.svg");

    // Example 10: Histogram
    println!("Creating histogram...");
    let histogram_data: Vec<f64> = (0..1000)
        .map(|i| {
            let x = (i as f64 - 500.0) / 100.0;
            x + (rand::random::<f64>() - 0.5) * 2.0
        })
        .collect();

    let mut fig10 = figure();
    fig10
        .add_subplot()
        .histogram(&histogram_data, 20)
        .set_title("Data Distribution Histogram")
        .set_xlabel("Value")
        .set_ylabel("Frequency")
        .grid(true);

    let histogram_svg = fig10.to_svg();
    std::fs::write("output/histogram_demo_main.svg", &histogram_svg)
        .expect("Failed to write histogram to file");
    println!("Histogram saved to histogram_demo_main.svg");

    // Example 11: Pie Chart
    println!("Creating pie chart...");
    let pie_values = vec![35.0, 25.0, 20.0, 12.0, 8.0];
    let pie_labels = vec![
        "Development".to_string(),
        "Marketing".to_string(),
        "Sales".to_string(),
        "Support".to_string(),
        "Other".to_string(),
    ];

    let mut fig11 = figure();
    fig11
        .add_subplot()
        .pie(&pie_values, Some(&pie_labels))
        .set_title("Department Budget Allocation");

    let pie_svg = fig11.to_svg();
    std::fs::write("output/pie_chart_demo_main.svg", &pie_svg)
        .expect("Failed to write pie chart to file");
    println!("Pie chart saved to pie_chart_demo_main.svg");

    // Example 12: Box plot
    println!("Creating box plot...");
    let box_data = vec![
        12.5, 14.2, 15.8, 16.1, 17.3, 18.9, 19.2, 20.1, 21.5, 22.8, 23.1, 24.7, 25.3, 26.9, 27.2,
        28.5, 29.1, 30.8, 31.2, 32.5, // Add some outliers
        8.0, 38.5, 42.0,
    ];

    let mut fig12 = figure();
    fig12
        .add_subplot()
        .boxplot(&box_data)
        .set_title("Performance Distribution with Outliers")
        .set_ylabel("Score");

    let box_svg = fig12.to_svg();
    std::fs::write("output/boxplot_demo_main.svg", &box_svg)
        .expect("Failed to write box plot to file");
    println!("Box plot saved to boxplot_demo_main.svg");

    // Example 13: Heatmap
    println!("Creating heatmap...");
    let correlation_matrix = vec![
        vec![1.0, 0.8, 0.3, -0.1],
        vec![0.8, 1.0, 0.5, 0.2],
        vec![0.3, 0.5, 1.0, 0.7],
        vec![-0.1, 0.2, 0.7, 1.0],
    ];

    let mut fig13 = figure();
    fig13
        .add_subplot()
        .heatmap(&correlation_matrix)
        .set_title("Correlation Matrix Heatmap")
        .set_xlabel("Variables")
        .set_ylabel("Variables")
        .grid(false);

    let heatmap_svg = fig13.to_svg();
    std::fs::write("output/heatmap_demo_main.svg", &heatmap_svg)
        .expect("Failed to write heatmap to file");
    println!("Heatmap saved to heatmap_demo_main.svg");

    // Example 14: Violin Plot
    println!("Creating violin plot...");
    let violin_data = vec![
        12.5, 14.2, 15.8, 16.1, 17.3, 18.9, 19.2, 20.1, 21.5, 22.8, 23.1, 24.7, 25.3, 26.9, 27.2,
        28.5, 29.1, 30.8, 31.2, 32.5, 33.1, 34.2, 35.5, 36.8, 37.1, 38.4, 39.7, 40.2, 41.5, 42.8,
        // Add some variation
        15.5, 18.2, 22.1, 25.8, 29.3, 33.7, 37.4, 41.1, 44.6, 48.2,
    ];

    let mut fig14 = figure();
    fig14
        .add_subplot()
        .violin(&violin_data)
        .set_title("Data Distribution - Violin Plot")
        .set_ylabel("Values")
        .grid(true);

    let violin_svg = fig14.to_svg();
    std::fs::write("output/violin_demo_main.svg", &violin_svg)
        .expect("Failed to write violin plot to file");
    println!("Violin plot saved to violin_demo_main.svg");

    // Example 15: Contour Plot
    println!("Creating contour plot...");
    let x_contour: Vec<f64> = (0..15).map(|i| i as f64 * 0.6).collect();
    let y_contour: Vec<f64> = (0..12).map(|i| i as f64 * 0.5).collect();

    // Create a 2D function: z = sin(x) * cos(y) + noise
    let mut z_contour: Vec<Vec<f64>> = Vec::new();
    for &yi in &y_contour {
        let mut row = Vec::new();
        for &xi in &x_contour {
            let zi = (xi * 0.4).sin() * (yi * 0.3).cos() + 0.1 * (xi * yi * 0.05).sin();
            row.push(zi);
        }
        z_contour.push(row);
    }

    let mut fig15 = figure();
    fig15
        .add_subplot()
        .contour(x_contour, y_contour, &z_contour)
        .set_title("Mathematical Function - Contour Plot")
        .set_xlabel("X values")
        .set_ylabel("Y values")
        .grid(true);

    let contour_svg = fig15.to_svg();
    std::fs::write("output/contour_demo_main.svg", &contour_svg)
        .expect("Failed to write contour plot to file");
    println!("Contour plot saved to contour_demo_main.svg");

    println!("PlotIron - Rust plotting library demo completed");
    println!(
        "Generated SVG content for all chart types including histograms, pie charts, box plots, heatmaps, violin plots, contour plots, advanced DOT graphs with multiple layouts, axis control, combined charts, and grid control"
    );
}
