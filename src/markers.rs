//! Marker definitions for scatter plots and line plots

use std::fmt;

/// Marker styles for plots
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Marker {
    /// Circle marker (default)
    Circle,
    /// Square marker
    Square,
    /// Triangle up marker
    TriangleUp,
    /// Triangle down marker
    TriangleDown,
    /// Diamond marker
    Diamond,
    /// Plus marker
    Plus,
    /// Cross marker
    Cross,
    /// Star marker
    Star,
    /// Modified diamond (graphviz Mdiamond)
    Mdiamond,
    /// Modified square (graphviz Msquare)
    Msquare,
    /// Ellipse marker
    Ellipse,
    /// No marker
    None,
}

impl Marker {
    /// Get the SVG element for the marker
    pub fn to_svg_element(&self, x: f64, y: f64, size: f64, color: &str) -> String {
        let half_size = size / 2.0;
        match self {
            Marker::Circle => {
                format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />", x, y, half_size, color)
            },
            Marker::Square => {
                format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />", 
                       x - half_size, y - half_size, size, size, color)
            },
            Marker::TriangleUp => {
                let h = half_size * 0.866; // sqrt(3)/2
                let points = format!("{},{} {},{} {},{}", 
                                    x, y - h, x - half_size, y + h, x + half_size, y + h);
                format!("<polygon points=\"{}\" fill=\"{}\" />", points, color)
            },
            Marker::TriangleDown => {
                let h = half_size * 0.866;
                let points = format!("{},{} {},{} {},{}", 
                                    x, y + h, x - half_size, y - h, x + half_size, y - h);
                format!("<polygon points=\"{}\" fill=\"{}\" />", points, color)
            },
            Marker::Diamond => {
                let points = format!("{},{} {},{} {},{} {},{}", 
                                    x, y - half_size, x + half_size, y, 
                                    x, y + half_size, x - half_size, y);
                format!("<polygon points=\"{}\" fill=\"{}\" />", points, color)
            },
            Marker::Plus => {
                let thin = half_size * 0.2;
                format!("<g fill=\"{}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" /><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" /></g>",
                       color,
                       x - thin, y - half_size, thin * 2.0, size,
                       x - half_size, y - thin, size, thin * 2.0)
            },
            Marker::Cross => {
                let thin = half_size * 0.2;
                let offset = half_size * 0.707;
                format!("<g fill=\"{}\" transform=\"translate({},{}) rotate(45)\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" /><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" /></g>",
                       color, x, y,
                       -thin, -offset, thin * 2.0, offset * 2.0,
                       -offset, -thin, offset * 2.0, thin * 2.0)
            },
            Marker::Star => {
                let mut points = Vec::new();
                for i in 0..10 {
                    let angle = (i as f64) * std::f64::consts::PI / 5.0;
                    let radius = if i % 2 == 0 { half_size } else { half_size * 0.4 };
                    let px = x + radius * angle.cos();
                    let py = y + radius * angle.sin();
                    points.push(format!("{},{}", px, py));
                }
                format!("<polygon points=\"{}\" fill=\"{}\" />", points.join(" "), color)
            },
            Marker::Mdiamond => {
                // Modified diamond shape like graphviz Mdiamond with rectangular aspect ratio
                let w = half_size * 2.6; // Much wider to match graphviz aspect ratio (~2.17:1)
                let h = half_size * 1.2; // Keep height similar
                // Main diamond outline
                let points = format!("{},{} {},{} {},{} {},{} {},{}", 
                                    x, y - h, x + w, y, x, y + h, x - w, y, x, y - h);
                // Internal polylines matching graphviz style
                let polylines = format!(
                    "<polyline fill=\"none\" stroke=\"black\" points=\"{},{} {},{}\" />\
                     <polyline fill=\"none\" stroke=\"black\" points=\"{},{} {},{}\" />\
                     <polyline fill=\"none\" stroke=\"black\" points=\"{},{} {},{}\" />\
                     <polyline fill=\"none\" stroke=\"black\" points=\"{},{} {},{}\" />",
                    x - w * 0.6, y - h * 0.5, x - w * 0.6, y,
                    x - w * 0.2, y - h * 0.8, x + w * 0.2, y - h * 0.8,
                    x + w * 0.6, y, x + w * 0.6, y + h * 0.5,
                    x + w * 0.2, y + h * 0.8, x - w * 0.2, y + h * 0.8
                );
                format!("<g><polygon points=\"{}\" fill=\"none\" stroke=\"black\"/>{}</g>", points, polylines)
            },
            Marker::Msquare => {
                // Modified square shape like graphviz Msquare (octagon)
                let s = half_size;
                let cut = s * 0.3;
                let points = format!("{},{} {},{} {},{} {},{} {},{} {},{} {},{} {},{}",
                                    x - s + cut, y - s,
                                    x + s - cut, y - s,
                                    x + s, y - s + cut,
                                    x + s, y + s - cut,
                                    x + s - cut, y + s,
                                    x - s + cut, y + s,
                                    x - s, y + s - cut,
                                    x - s, y - s + cut);
                let polylines = format!(
                    "<polyline fill=\"none\" stroke=\"black\" points=\"{},{} {},{}\"/>\
                     <polyline fill=\"none\" stroke=\"black\" points=\"{},{} {},{}\"/>\
                     <polyline fill=\"none\" stroke=\"black\" points=\"{},{} {},{}\"/>\
                     <polyline fill=\"none\" stroke=\"black\" points=\"{},{} {},{}\"/>",
                    x - s + cut * 2.0, y - s, x - s, y - s + cut * 2.0,
                    x - s, y - cut, x - s + cut, y,
                    x + s - cut, y, x + s, y - cut,
                    x + s, y + s - cut * 2.0, x + s - cut * 2.0, y + s
                );
                format!("<g><polygon points=\"{}\" fill=\"none\" stroke=\"black\"/>{}</g>", points, polylines)
            },
            Marker::Ellipse => {
                // Ellipse size matching graphviz standards (rx=27, ry=18 when half_size=7.5)
                let rx = half_size * 3.6; // Horizontal radius to match graphviz rx=27
                let ry = half_size * 2.4; // Vertical radius to match graphviz ry=18
                format!("<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"{}\" />", x, y, rx, ry, color)
            },
            Marker::None => String::new(),
        }
    }

    /// Check if marker should be rendered
    pub fn is_visible(&self) -> bool {
        !matches!(self, Marker::None)
    }
}

impl Default for Marker {
    fn default() -> Self {
        Marker::Circle
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Marker::Circle => "circle",
            Marker::Square => "square",
            Marker::TriangleUp => "triangle_up",
            Marker::TriangleDown => "triangle_down",
            Marker::Diamond => "diamond",
            Marker::Plus => "plus",
            Marker::Cross => "cross",
            Marker::Star => "star",
            Marker::Mdiamond => "mdiamond",
            Marker::Msquare => "msquare",
            Marker::Ellipse => "ellipse",
            Marker::None => "none",
        };
        write!(f, "{}", name)
    }
}

impl From<&str> for Marker {
    fn from(s: &str) -> Self {
        match s {
            "o" | "circle" => Marker::Circle,
            "s" | "square" => Marker::Square,
            "^" | "triangle_up" => Marker::TriangleUp,
            "v" | "triangle_down" => Marker::TriangleDown,
            "D" | "diamond" => Marker::Diamond,
            "+" | "plus" => Marker::Plus,
            "x" | "cross" => Marker::Cross,
            "*" | "star" => Marker::Star,
            "mdiamond" => Marker::Mdiamond,
            "ellipse" => Marker::Ellipse,
            "msquare" => Marker::Msquare,
            "" | "none" => Marker::None,
            _ => Marker::Circle,
        }
    }
}