use std::path::PathBuf;

use plotiron::*;

fn main() {
    let x: Vec<_> = (-100..100).map(|i| (i as f64) * 0.5).collect();
    let y: Vec<_> = x.iter().map(|a| a.sin()).collect();
    let z: Vec<_> = x.iter().map(|a| a.cos()).collect();

    let mut fig = figure();
    fig.add_subplot().plot(&x, &y).plot(&x, &z);

    let workspace_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    std::fs::write(workspace_dir.join("output/line.svg"), fig.to_svg()).unwrap();

    fig.show();
}
