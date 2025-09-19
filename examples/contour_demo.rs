use plotiron::prelude::*;

fn main() {
    // Create sample data for contour plot
    let x: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
    let y: Vec<f64> = (0..15).map(|i| i as f64 * 0.4).collect();

    // Create a 2D function: z = sin(x) * cos(y)
    let mut z: Vec<Vec<f64>> = Vec::new();
    for &yi in &y {
        let mut row = Vec::new();
        for &xi in &x {
            let zi = (xi * 0.5).sin() * (yi * 0.3).cos() + 0.2 * (xi * yi * 0.1).sin();
            row.push(zi);
        }
        z.push(row);
    }

    // Create contour plot
    let contour_plot = Plot::contour(x.as_slice(), y.as_slice(), &z)
        .label("Contour Plot")
        .alpha(0.8);

    // Create figure and add the plot
    let mut fig = Figure::new();
    let axes = fig.add_subplot();
    axes.set_title("Contour Plot Demo")
        .set_xlabel("X values")
        .set_ylabel("Y values");

    axes.plots.push(contour_plot);

    // Save as SVG
    let svg_content = fig.to_svg();
    std::fs::create_dir_all("output").unwrap_or_default();
    match std::fs::write("output/contour_demo.svg", svg_content) {
        Ok(_) => println!("Contour plot saved as output/contour_demo.svg"),
        Err(e) => eprintln!("Error saving contour plot: {}", e),
    }

    // Create a second example with different function
    let mut z2: Vec<Vec<f64>> = Vec::new();
    for &yi in &y {
        let mut row = Vec::new();
        for &xi in &x {
            let zi = ((xi - 5.0).powi(2) + (yi - 3.0).powi(2)).sqrt().exp() * -0.1;
            row.push(zi);
        }
        z2.push(row);
    }

    let contour_plot2 = Plot::contour(x, y, &z2)
        .label("Gaussian-like Function")
        .alpha(0.9);

    let mut fig2 = Figure::new();
    let axes2 = fig2.add_subplot();
    axes2
        .set_title("Contour Plot - Gaussian Function")
        .set_xlabel("X values")
        .set_ylabel("Y values");

    axes2.plots.push(contour_plot2);

    let svg_content2 = fig2.to_svg();
    match std::fs::write("output/contour_gaussian.svg", svg_content2) {
        Ok(_) => println!("Gaussian contour plot saved as output/contour_gaussian.svg"),
        Err(e) => eprintln!("Error saving gaussian contour plot: {}", e),
    }

    println!("Contour plot demo completed!");
}
