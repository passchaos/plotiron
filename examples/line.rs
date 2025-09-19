use std::path::PathBuf;

use plotiron::prelude::*;
use vectra::Array;

fn main() {
    let x: Vec<_> = (-100..100).map(|i| (i as f64) * 0.1).collect();

    let x = Array::from(x);
    let y = x.sin();
    let z = x.cos();

    let mut fig = figure();
    fig.add_subplot().plot(&x, y).plot(x, z);

    let workspace_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    std::fs::write(workspace_dir.join("output/line.svg"), fig.to_svg()).unwrap();

    fig.show();
}
