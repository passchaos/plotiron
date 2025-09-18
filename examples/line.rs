use plotiron::*;

fn main() {
    let x: Vec<_> = (-100..100).map(|i| (i as f64) * 0.2).collect();
    let y: Vec<_> = x.iter().map(|a| a.sin()).collect();
    let z: Vec<_> = x.iter().map(|a| a.cos()).collect();

    let mut fig = figure_with_size(1000.0, 800.0);
    fig.add_subplot().plot(&x, &y).plot(&x, &z);

    std::fs::write("output/line.svg", fig.to_svg()).unwrap();

    fig.show();
}
