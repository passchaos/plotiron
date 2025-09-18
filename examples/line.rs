use plotiron::*;

fn main() {
    let x: Vec<_> = (-100..100).map(|i| (i as f64) * 0.1).collect();
    let y: Vec<_> = x.iter().map(|a| a.sin()).collect();

    let mut fig = figure();
    fig.add_subplot().plot(&x, &y);

    std::fs::write("output/line.svg", fig.to_svg()).unwrap();

    fig.show();
}
