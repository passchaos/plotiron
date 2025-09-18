//! Histogram demo for PlotIron

use plotiron::*;

fn main() {
    println!("PlotIron - Histogram demo");
    
    // Generate sample data - simple normal-like distribution
    let data1: Vec<f64> = (0..1000).map(|i| {
        let x = (i as f64 - 500.0) / 100.0;
        x + (rand::random::<f64>() - 0.5) * 2.0
    }).collect();
    
    // Generate another dataset with different characteristics
    let data2: Vec<f64> = (0..800).map(|i| {
        let x = (i as f64 - 200.0) / 80.0;
        x * 1.5 + (rand::random::<f64>() - 0.5) * 3.0
    }).collect();
    
    // Create figure and add histograms
    let mut fig = figure();
    
    // Single histogram
    fig.add_subplot()
        .histogram(&data1, 30)
        .set_title("Sample Data Histogram")
        .set_xlabel("Value")
        .set_ylabel("Frequency")
        .grid(true);
    
    // Save the figure
    let svg_content = fig.to_svg();
    std::fs::create_dir_all("output").ok();
    match std::fs::write("output/histogram_demo.svg", svg_content) {
        Ok(_) => println!("Histogram demo saved to output/histogram_demo.svg"),
        Err(e) => eprintln!("Error saving histogram demo: {}", e),
    }
    
    // Create another figure with overlapping histograms
    let mut fig2 = figure_with_size(900.0, 600.0);
    fig2.add_subplot()
        .histogram(&data1, 25)
        .histogram(&data2, 25)
        .set_title("Overlapping Histograms")
        .set_xlabel("Value")
        .set_ylabel("Frequency")
        .grid(true)
        .legend(true);
    
    let svg_content2 = fig2.to_svg();
    match std::fs::write("output/histogram_overlapping.svg", svg_content2) {
        Ok(_) => println!("Overlapping histograms saved to output/histogram_overlapping.svg"),
        Err(e) => eprintln!("Error saving overlapping histograms: {}", e),
    }
    
    println!("Histogram demo completed!");
}