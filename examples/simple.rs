use std::f64::consts::PI;

use plotiron::prelude::*;
use vectra::prelude::*;

fn main() {
    let x: Vec<_> = (-100..100).map(|i| (i as f64) * 0.1).collect();

    let x = Array::from(x);
    let y = x.sin();
    let z = x.cos();

    let mut fig = figure();
    // fig.add_subplot()
    //     .add_plot(Plot::line(&x, y).label("sin(x)"))
    //     .add_plot(Plot::line(x, z).label("cos(x)"))
    //     .legend(true);

    // let scatter_x = Array::arange(0.0, 10.0, 0.1);
    let scatter_x = Array::arange(-5.0, 5.0, 0.01);

    fn density(miu: f64, delta: f64, x: f64) -> f64 {
        (1.0 / (2.0 * PI * delta.powi(2)).sqrt()) * (-0.5 * ((x - miu) / delta).powi(2)).exp()
    }

    let axes = fig.add_subplot();
    for (miu, delta) in [(0.0, 1.0), (0.0, 0.5), (0.0, 2.0), (-2.0, 1.0)] {
        let scatter_y = scatter_x.map(|&x| density(miu, delta, x));
        axes.add_plot(
            Plot::scatter(&scatter_x, scatter_y).label(&format!("miu= {miu}, delta= {delta}")),
        );
    }
    axes.legend(true);

    // let workspace_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    // std::fs::write(workspace_dir.join("output/line.svg"), fig.to_svg()).unwrap();

    fig.show();
}
