//! PlotIron - A Rust plotting library inspired by matplotlib
//!
//! This library provides a simple and intuitive API for creating 2D plots
//! similar to matplotlib in Python.

pub mod figure;
pub mod axes;
pub mod plot;
pub mod colors;
pub mod markers;
pub mod utils;
pub mod dot;

pub use figure::Figure;
pub use axes::Axes;
pub use plot::{Plot, PlotType};
pub use colors::Color;
pub use markers::Marker;

/// Create a new figure with default settings
pub fn figure() -> Figure {
    Figure::new()
}

/// Create a new figure with specified width and height
pub fn figure_with_size(width: f64, height: f64) -> Figure {
    Figure::with_size(width, height)
}

/// Quick plot function for simple line plots
pub fn plot(x: &[f64], y: &[f64]) -> Figure {
    let mut fig = Figure::new();
    fig.add_subplot().plot(x, y);
    fig
}

/// Quick scatter plot function
pub fn scatter(x: &[f64], y: &[f64]) -> Figure {
    let mut fig = Figure::new();
    fig.add_subplot().scatter(x, y);
    fig
}