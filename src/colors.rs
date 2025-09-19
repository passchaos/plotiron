//! Color definitions and utilities

use std::fmt;

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64, // Alpha channel (0.0 to 1.0)
}

impl Color {
    /// Create a new color with RGB values
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    /// Create a new color with RGBA values
    pub fn rgba(r: u8, g: u8, b: u8, a: f64) -> Self {
        Color { r, g, b, a: a.clamp(0.0, 1.0) }
    }

    /// Create color from hex string (e.g., "#FF0000" for red)
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Hex color must be 6 characters long");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;

        Ok(Color::rgb(r, g, b))
    }

    /// Convert to SVG color string
    pub fn to_svg_string(&self) -> String {
        if self.a < 1.0 {
            format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
        } else {
            format!("rgb({},{},{})", self.r, self.g, self.b)
        }
    }

    // Predefined colors (matplotlib-like)
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 1.0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 1.0 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0, g: 128, b: 0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 1.0 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0, g: 255, b: 255, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 1.0 };
    pub const ORANGE: Color = Color { r: 255, g: 165, b: 0, a: 1.0 };
    pub const PURPLE: Color = Color { r: 128, g: 0, b: 128, a: 1.0 };
    pub const GRAY: Color = Color { r: 128, g: 128, b: 128, a: 1.0 };
    pub const LIGHTGRAY: Color = Color { r: 211, g: 211, b: 211, a: 1.0 };
    pub const DARKGRAY: Color = Color { r: 64, g: 64, b: 64, a: 1.0 };
    
    // Matplotlib-style colors for better aesthetics
    pub const GRID_COLOR: Color = Color { r: 230, g: 230, b: 230, a: 1.0 }; // Very light gray for grid
    pub const AXIS_COLOR: Color = Color { r: 77, g: 77, b: 77, a: 1.0 };   // Dark gray for axes
    pub const TEXT_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 1.0 };     // Pure black for text
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color(r={}, g={}, b={}, a={})", self.r, self.g, self.b, self.a)
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "black" | "k" => Color::BLACK,
            "white" | "w" => Color::WHITE,
            "red" | "r" => Color::RED,
            "green" | "g" => Color::GREEN,
            "blue" | "b" => Color::BLUE,
            "yellow" | "y" => Color::YELLOW,
            "cyan" | "c" => Color::CYAN,
            "magenta" | "m" => Color::MAGENTA,
            "orange" => Color::ORANGE,
            "purple" => Color::PURPLE,
            "gray" | "grey" => Color::GRAY,
            _ => {
                if s.starts_with('#') {
                    Color::from_hex(s).unwrap_or(Color::BLACK)
                } else {
                    Color::BLACK
                }
            }
        }
    }
}

/// Default color cycle for plots (matplotlib-like)
pub const DEFAULT_COLOR_CYCLE: [Color; 10] = [
    Color { r: 31, g: 119, b: 180, a: 1.0 },   // blue
    Color { r: 255, g: 127, b: 14, a: 1.0 },   // orange
    Color { r: 44, g: 160, b: 44, a: 1.0 },    // green
    Color { r: 214, g: 39, b: 40, a: 1.0 },    // red
    Color { r: 148, g: 103, b: 189, a: 1.0 },  // purple
    Color { r: 140, g: 86, b: 75, a: 1.0 },    // brown
    Color { r: 227, g: 119, b: 194, a: 1.0 },  // pink
    Color { r: 127, g: 127, b: 127, a: 1.0 },  // gray
    Color { r: 188, g: 189, b: 34, a: 1.0 },   // olive
    Color { r: 23, g: 190, b: 207, a: 1.0 },   // cyan
];

/// Get color from the default color cycle
pub fn get_cycle_color(index: usize) -> Color {
    DEFAULT_COLOR_CYCLE[index % DEFAULT_COLOR_CYCLE.len()]
}