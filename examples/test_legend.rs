//! Legend functionality test for PlotIron

use plotiron::prelude::*;
use vectra::Array;

fn main() {
    use unicode_display_width::width;

    fn main() {
        assert_eq!(width("🔥🗡🍩👩🏻‍🚀⏰💃🏼🔦👍🏻"), 15);
        assert_eq!(width("🦀"), 2);
        assert_eq!(width("👨‍👩‍👧‍👧"), 2);
        assert_eq!(width("sane text"), 9);
        assert_eq!(width("Ẓ̌á̲l͔̝̞̄̑͌g̖̘̘̔̔͢͞͝o̪̔T̢̙̫̈̍͞e̬͈͕͌̏͑x̺̍ṭ̓̓ͅ"), 9);
        assert_eq!(width("슬라바 우크라이나"), 17);
    }

    println!("PlotIron - Legend functionality test");

    // Create test data
    let x: Vec<_> = (-100..100).map(|i| (i as f64) * 0.1).collect();
    let x = Array::from(x);

    // Calculate different functions
    let y_sin = x.sin();
    let y_cos = x.cos();
    let y_tan = x.tan();

    // Create figure with multiple plots and labels
    let mut fig = figure();
    let subplot = fig.add_subplot();

    // Add plots with labels for legend
    subplot.plot(&x, &y_sin);
    if let Some(plot) = subplot.plots.last_mut() {
        plot.label = Some("sin(x)".to_string());
        plot.color = Color::BLUE;
    }

    subplot.plot(&x, &y_cos);
    if let Some(plot) = subplot.plots.last_mut() {
        plot.label = Some("cos(x)".to_string());
        plot.color = Color::RED;
    }

    // Limit tan values to avoid extreme values
    let y_tan_limited: Vec<f64> = y_tan
        .iter()
        .map(|&val| if val.abs() > 5.0 { f64::NAN } else { val })
        .collect();

    subplot.plot(&x, y_tan_limited);
    if let Some(plot) = subplot.plots.last_mut() {
        plot.label = Some("tan(x)".to_string());
        plot.color = Color::GREEN;
    }

    // Configure the subplot
    subplot
        .set_title("Trigonometric Functions with Legend")
        .set_xlabel("x")
        .set_ylabel("y")
        .grid(true)
        .legend(true); // Enable legend

    // Save the figure
    let svg_content = fig.to_svg();
    std::fs::create_dir_all("output").ok();
    match std::fs::write("output/test_legend.svg", svg_content) {
        Ok(_) => println!("Legend test saved to output/test_legend.svg"),
        Err(e) => eprintln!("Error saving legend test: {}", e),
    }

    // Test 2: Scatter plot with legend
    let mut fig2 = figure();
    let subplot2 = fig2.add_subplot();

    // Generate some sample data
    let x1: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
    let y1: Vec<f64> = x1
        .iter()
        .map(|&x| x + (rand::random::<f64>() - 0.5) * 2.0)
        .collect();

    let x2: Vec<f64> = (0..50).map(|i| i as f64 * 0.2 + 1.0).collect();
    let y2: Vec<f64> = x2
        .iter()
        .map(|&x| x * 0.5 + (rand::random::<f64>() - 0.5) * 1.5)
        .collect();

    // Add scatter plots with labels
    subplot2.scatter(x1, y1);
    if let Some(plot) = subplot2.plots.last_mut() {
        plot.label = Some("Dataset A".to_string());
        plot.color = Color::PURPLE;
    }

    subplot2.scatter(x2, y2);
    if let Some(plot) = subplot2.plots.last_mut() {
        plot.label = Some("Dataset B".to_string());
        plot.color = Color::ORANGE;
    }

    // Configure the subplot
    subplot2
        .set_title("Scatter Plot with Legend")
        .set_xlabel("X values")
        .set_ylabel("Y values")
        .grid(true)
        .legend(true); // Enable legend

    // Save the second figure
    let svg_content2 = fig2.to_svg();
    match std::fs::write("output/test_legend_scatter.svg", svg_content2) {
        Ok(_) => println!("Scatter legend test saved to output/test_legend_scatter.svg"),
        Err(e) => eprintln!("Error saving scatter legend test: {}", e),
    }

    println!("Legend functionality tests completed!");
}