//! Test example for the prelude module
//!
//! This example demonstrates how to use the prelude module to import
//! all commonly used types and functions with a single import statement.

use plotiron::prelude::*;

fn main() {
    println!("Testing prelude module...");

    // Test data
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    // Create a figure using the prelude imports
    let mut fig = figure();

    // Add a plot using different input types to test IntoVec trait
    fig.add_subplot().plot(x, y);

    // Test with owned vectors
    let x2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y2 = vec![1.0, 3.0, 5.0, 7.0, 9.0];

    fig.add_subplot().scatter(x2, y2);

    // Test with arrays
    let x3 = [1.0, 2.0, 3.0, 4.0, 5.0];
    let y3 = [0.5, 1.5, 2.5, 3.5, 4.5];

    fig.add_subplot().plot(&x3, &y3);

    // Save the figure
    std::fs::create_dir_all("output").unwrap();
    std::fs::write("output/test_prelude.svg", fig.to_svg()).unwrap();

    println!("Prelude test completed successfully!");
    println!("Output saved to: output/test_prelude.svg");
}
