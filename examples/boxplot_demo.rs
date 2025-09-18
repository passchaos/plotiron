use plotiron::*;

fn main() {
    // Example 1: Simple box plot with normal distribution
    let data1 = vec![
        1.2, 2.3, 3.1, 4.5, 5.2, 6.1, 7.3, 8.2, 9.1, 10.5,
        11.2, 12.8, 13.5, 14.1, 15.3, 16.7, 17.2, 18.9, 19.1, 20.3,
        21.5, 22.1, 23.8, 24.2, 25.6, 26.3, 27.1, 28.5, 29.2, 30.1
    ];
    
    let mut fig1 = figure();
    fig1.add_subplot()
        .boxplot(&data1)
        .set_title("Simple Box Plot - Normal Distribution")
        .set_ylabel("Values");
    
    std::fs::write("output/boxplot_simple.svg", fig1.to_svg()).expect("Failed to write box plot to file");
    println!("Simple box plot saved to boxplot_simple.svg");
    
    // Example 2: Box plot with outliers
    let data2 = vec![
        10.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0,
        21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
        // Add some outliers
        5.0, 45.0, 50.0
    ];
    
    let mut fig2 = figure();
    fig2.add_subplot()
        .boxplot(&data2)
        .set_title("Box Plot with Outliers")
        .set_ylabel("Test Scores");
    
    std::fs::write("output/boxplot_outliers.svg", fig2.to_svg()).expect("Failed to write box plot to file");
    println!("Box plot with outliers saved to boxplot_outliers.svg");
    
    // Example 3: Box plot with skewed data
    let data3 = vec![
        1.0, 1.5, 2.0, 2.2, 2.5, 3.0, 3.2, 3.5, 4.0, 4.5,
        5.0, 6.0, 7.0, 8.0, 10.0, 12.0, 15.0, 18.0, 22.0, 28.0,
        35.0, 45.0, 60.0, 80.0, 100.0
    ];
    
    let mut fig3 = figure();
    fig3.add_subplot()
        .boxplot(&data3)
        .set_title("Box Plot - Skewed Distribution")
        .set_ylabel("Response Time (ms)");
    
    std::fs::write("output/boxplot_skewed.svg", fig3.to_svg()).expect("Failed to write box plot to file");
    println!("Skewed box plot saved to boxplot_skewed.svg");
    
    println!("Box plot demo completed!");
}