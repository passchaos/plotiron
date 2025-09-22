//! PlotIron - A Rust plotting library inspired by matplotlib
//!
//! This library provides a simple and intuitive API for creating 2D plots
//! similar to matplotlib in Python.

pub mod axes;
pub mod colors;
// pub mod dot;
pub mod figure;
pub mod markers;
pub mod plot;
pub mod prelude;
pub mod utils;
pub mod viewer;

pub use axes::Axes;
pub use colors::Color;
pub use figure::Figure;
pub use markers::Marker;
pub use plot::{Plot, PlotType};

/// Trait for types that can be converted into Vec<f64>
pub trait IntoVec<T> {
    fn into_vec(self) -> Vec<T>;
}

impl<T, Tp> IntoVec<T> for Tp
where
    Tp: Into<Vec<T>>,
{
    fn into_vec(self) -> Vec<T> {
        self.into()
    }
}

/// Create a new figure with default settings
pub fn figure() -> Figure {
    Figure::new()
}

/// Create a new figure with specified width and height
pub fn figure_with_size(width: f64, height: f64) -> Figure {
    Figure::with_size(width, height)
}
