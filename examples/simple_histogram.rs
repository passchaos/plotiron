//! Simple histogram example for PlotIron

use plotiron::*;

fn main() {
    println!("PlotIron - Simple Histogram Example");
    
    // Create simple test data
    let data = vec![
        1.0, 1.2, 1.5, 1.8, 2.0, 2.1, 2.3, 2.5, 2.7, 2.9,
        3.0, 3.1, 3.2, 3.4, 3.6, 3.8, 4.0, 4.2, 4.5, 4.8,
        5.0, 5.2, 5.5, 5.8, 6.0, 6.1, 6.3, 6.5, 6.7, 6.9,
        7.0, 7.2, 7.5, 7.8, 8.0, 8.2, 8.5, 8.8, 9.0, 9.5
    ];
    
    // Create figure with histogram
    let mut fig = figure();
    fig.add_subplot()
        .histogram(&data, 10)
        .set_title("Simple Histogram Example")
        .set_xlabel("Value")
        .set_ylabel("Count")
        .grid(true);
    
    // Save to file
    let svg_content = fig.to_svg();
    std::fs::create_dir_all("output").ok();
    match std::fs::write("output/simple_histogram.svg", svg_content) {
        Ok(_) => println!("Simple histogram saved to output/simple_histogram.svg"),
        Err(e) => eprintln!("Error saving simple histogram: {}", e),
    }
    
    println!("Simple histogram example completed!");
}