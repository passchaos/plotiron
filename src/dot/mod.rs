//! DOT graph rendering module with advanced layout algorithms
//! Provides Graphviz-like functionality for rendering DOT graphs

pub mod layout;
pub mod parser;
pub mod renderer;
pub mod types;

pub use types::*;
// pub use parser::*;  // Commented out unused import
// pub use layout::*;  // Commented out unused import
// pub use renderer::*;  // Commented out unused import

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_GRAPH: &str = r#""#;

    #[test]
    fn test_simple_graph() {
        let x: Vec<_> = (-100..100).map(|i| (i as f64) * 0.1).collect();

        let x = Array::from(x);
        let y = x.sin();

        let mut fig = figure();
        fig.add_subplot().plot(&x, y);

        let workspace_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
        std::fs::write(workspace_dir.join("output/line.svg"), fig.to_svg()).unwrap();
    }
}
