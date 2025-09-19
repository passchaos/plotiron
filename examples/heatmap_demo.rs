//! Heatmap demo for PlotIron

use plotiron::prelude::*;

fn main() {
    println!("PlotIron - Heatmap demo");

    // Example 1: Simple correlation matrix
    let correlation_data = vec![
        vec![1.0, 0.8, 0.3, -0.2],
        vec![0.8, 1.0, 0.5, 0.1],
        vec![0.3, 0.5, 1.0, 0.7],
        vec![-0.2, 0.1, 0.7, 1.0],
    ];

    let mut fig1 = figure();
    fig1.add_subplot()
        .heatmap(&correlation_data)
        .set_title("Correlation Matrix")
        .set_xlabel("Variables")
        .set_ylabel("Variables");

    std::fs::create_dir_all("output").ok();
    std::fs::write("output/heatmap_correlation.svg", fig1.to_svg())
        .expect("Failed to write correlation heatmap to file");
    println!("Correlation heatmap saved to heatmap_correlation.svg");

    // Example 2: Temperature data (simulated)
    let temp_data: Vec<Vec<f64>> = (0..8)
        .map(|i| {
            (0..12)
                .map(|j| {
                    // Simulate temperature data with seasonal variation
                    let base_temp = 15.0;
                    let seasonal = 10.0 * ((j as f64 * std::f64::consts::PI / 6.0).sin());
                    let daily = 5.0 * ((i as f64 * std::f64::consts::PI / 4.0).cos());
                    let noise = (i * j) as f64 % 3.0 - 1.0;
                    base_temp + seasonal + daily + noise
                })
                .collect()
        })
        .collect();

    let mut fig2 = figure_with_size(900.0, 600.0);
    fig2.add_subplot()
        .heatmap(&temp_data)
        .set_title("Temperature Heatmap (8 locations × 12 months)")
        .set_xlabel("Months")
        .set_ylabel("Locations");

    std::fs::write("output/heatmap_temperature.svg", fig2.to_svg())
        .expect("Failed to write temperature heatmap to file");
    println!("Temperature heatmap saved to heatmap_temperature.svg");

    // Example 3: Small intensity matrix
    let intensity_data = vec![
        vec![0.1, 0.3, 0.8, 0.9, 0.7],
        vec![0.2, 0.5, 0.9, 1.0, 0.8],
        vec![0.4, 0.7, 1.0, 0.9, 0.6],
        vec![0.3, 0.6, 0.8, 0.7, 0.4],
        vec![0.1, 0.2, 0.4, 0.3, 0.2],
    ];

    let mut fig3 = figure();
    fig3.add_subplot()
        .heatmap(&intensity_data)
        .set_title("Intensity Matrix")
        .set_xlabel("X Coordinate")
        .set_ylabel("Y Coordinate")
        .grid(false); // Disable grid for cleaner heatmap

    std::fs::write("output/heatmap_intensity.svg", fig3.to_svg())
        .expect("Failed to write intensity heatmap to file");
    println!("Intensity heatmap saved to heatmap_intensity.svg");

    println!("Heatmap demo completed!");
}
