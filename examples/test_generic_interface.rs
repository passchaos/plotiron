use plotiron::{Figure, IntoVec, Plot};

fn main() {
    // Test different input types for the generic interface
    let mut fig = Figure::new();
    let axes = fig.add_subplot();

    // Test with Vec<f64>
    let x_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_vec = vec![1.0, 4.0, 9.0, 16.0, 25.0];
    axes.plot(x_vec.clone(), y_vec.clone());

    // Test with &[f64]
    let x_slice = &[1.5, 2.5, 3.5, 4.5, 5.5];
    let y_slice = &[2.25, 6.25, 12.25, 20.25, 30.25];
    axes.scatter(x_slice, y_slice);

    // Test with &Vec<f64>
    axes.bar(x_vec.clone(), y_vec.clone());

    // Test with array references
    let x_array = [2.0, 3.0, 4.0, 5.0, 6.0];
    let y_array = [4.0, 9.0, 16.0, 25.0, 36.0];
    axes.plot(&x_array, &y_array);

    // Test Plot constructors directly with different input types
    let _line_plot = Plot::line(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0]); // Vec<f64>
    let _scatter_plot = Plot::scatter(&[0.0, 1.0, 2.0], &[0.0, 2.0, 8.0]); // &[f64]
    let _bar_plot = Plot::bar(x_vec, y_vec); // &Vec<f64>

    // Test with fixed-size arrays
    let x_fixed = [1.0, 2.0, 3.0];
    let y_fixed = [1.0, 4.0, 9.0];
    let _array_plot = Plot::line(&x_fixed, &y_fixed); // &[f64; 3]

    // Test with owned arrays
    let _owned_array_plot = Plot::scatter([1.5, 2.5, 3.5], [2.25, 6.25, 12.25]); // [f64; 3]

    axes.set_title("Generic Interface Test")
        .set_xlabel("X values")
        .set_ylabel("Y values")
        .legend(true);

    // Save the plot
    let svg_content = fig.to_svg();
    std::fs::write("output/test_generic_interface.svg", svg_content)
        .expect("Failed to write SVG file");

    println!("IntoVec trait interface test completed successfully!");
    println!("Successfully tested: Vec<f64>, &[f64], &Vec<f64>, &[f64; N], [f64; N]");
    println!("Output saved to: output/test_generic_interface.svg");
}
