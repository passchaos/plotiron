use plotiron::*;

fn main() {
    println!("PlotIron - Violin Plot Demo");
    
    // Example 1: Normal distribution data
    let normal_data = vec![
        12.5, 14.2, 15.8, 16.1, 17.3, 18.9, 19.2, 20.1, 21.5, 22.8,
        23.1, 24.7, 25.3, 26.9, 27.2, 28.5, 29.1, 30.8, 31.2, 32.5,
        33.1, 34.2, 35.5, 36.8, 37.1, 38.4, 39.7, 40.2, 41.5, 42.8,
        43.2, 44.1, 45.3, 46.7, 47.9, 48.2, 49.5, 50.1, 51.8, 52.3
    ];
    
    let mut fig1 = figure();
    fig1.add_subplot()
        .violin(&normal_data)
        .set_title("Normal Distribution - Violin Plot")
        .set_ylabel("Values")
        .grid(true);
    
    std::fs::create_dir_all("output").ok();
    std::fs::write("output/violin_normal.svg", fig1.to_svg()).expect("Failed to write violin plot to file");
    println!("Normal distribution violin plot saved to violin_normal.svg");
    
    // Example 2: Bimodal distribution data
    let mut bimodal_data = Vec::new();
    // First mode around 20
    for i in 0..30 {
        bimodal_data.push(20.0 + (i as f64 - 15.0) * 0.5 + (rand::random::<f64>() - 0.5) * 3.0);
    }
    // Second mode around 40
    for i in 0..30 {
        bimodal_data.push(40.0 + (i as f64 - 15.0) * 0.5 + (rand::random::<f64>() - 0.5) * 3.0);
    }
    
    let mut fig2 = figure();
    fig2.add_subplot()
        .violin(&bimodal_data)
        .set_title("Bimodal Distribution - Violin Plot")
        .set_ylabel("Values")
        .grid(true);
    
    std::fs::write("output/violin_bimodal.svg", fig2.to_svg()).expect("Failed to write bimodal violin plot to file");
    println!("Bimodal distribution violin plot saved to violin_bimodal.svg");
    
    // Example 3: Skewed distribution data
    let skewed_data = vec![
        1.0, 1.5, 2.0, 2.2, 2.5, 3.0, 3.2, 3.5, 4.0, 4.5,
        5.0, 6.0, 7.0, 8.0, 10.0, 12.0, 15.0, 18.0, 22.0, 28.0,
        35.0, 45.0, 60.0, 80.0, 100.0, 120.0, 150.0, 180.0, 220.0, 280.0
    ];
    
    let mut fig3 = figure();
    fig3.add_subplot()
        .violin(&skewed_data)
        .set_title("Right-Skewed Distribution - Violin Plot")
        .set_ylabel("Values")
        .grid(true);
    
    std::fs::write("output/violin_skewed.svg", fig3.to_svg()).expect("Failed to write skewed violin plot to file");
    println!("Skewed distribution violin plot saved to violin_skewed.svg");
    
    println!("Violin plot demo completed!");
}