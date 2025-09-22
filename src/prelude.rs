//! Prelude module for plotiron
//!
//! This module re-exports the most commonly used types and traits from plotiron,
//! making it convenient to import everything needed with a single `use` statement:
//!
//! ```rust
//! use plotiron::prelude::*;
//! ```

pub use crate::{IntoVec, axes::Axes, colors::Color, figure::Figure, markers::Marker, plot::Plot};

// Re-export DOT module for graph visualization
// pub use crate::dot;

// Re-export commonly used functions
pub use crate::{figure, figure_with_size};
