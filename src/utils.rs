//! Utility functions for the plotting library

use num_traits::Float;

/// Calculate the range of values in a slice
pub fn calculate_range<T: Float + Copy>(data: &[T]) -> (T, T) {
    if data.is_empty() {
        return (T::zero(), T::one());
    }
    
    let mut min_val = data[0];
    let mut max_val = data[0];
    
    for &value in data.iter() {
        if value < min_val {
            min_val = value;
        }
        if value > max_val {
            max_val = value;
        }
    }
    
    // Add some padding if min == max
    if min_val == max_val {
        let padding = if min_val == T::zero() { T::one() } else { min_val * T::from(0.1).unwrap() };
        min_val = min_val - padding;
        max_val = max_val + padding;
    }
    
    (min_val, max_val)
}

/// Generate nice tick values for an axis
pub fn generate_ticks(min: f64, max: f64, target_count: usize) -> Vec<f64> {
    if min >= max || target_count == 0 {
        return vec![min, max];
    }
    
    let range = max - min;
    let raw_step = range / (target_count - 1) as f64;
    
    // Find a "nice" step size
    let magnitude = 10.0_f64.powf(raw_step.log10().floor());
    let normalized_step = raw_step / magnitude;
    
    let nice_step = if normalized_step <= 1.0 {
        1.0
    } else if normalized_step <= 2.0 {
        2.0
    } else if normalized_step <= 5.0 {
        5.0
    } else {
        10.0
    } * magnitude;
    
    // Generate ticks
    let start = (min / nice_step).floor() * nice_step;
    let mut ticks = Vec::new();
    let mut current = start;
    
    while current <= max + nice_step * 0.001 {
        if current >= min - nice_step * 0.001 {
            ticks.push(current);
        }
        current += nice_step;
    }
    
    if ticks.is_empty() {
        vec![min, max]
    } else {
        ticks
    }
}

/// Linear interpolation between two values
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Map a value from one range to another
pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    if from_max == from_min {
        return to_min;
    }
    let t = (value - from_min) / (from_max - from_min);
    lerp(to_min, to_max, t)
}

/// Format a number for display on axes
pub fn format_number(value: f64) -> String {
    if value.abs() < 1e-10 {
        "0".to_string()
    } else if value.abs() >= 1e6 || value.abs() < 1e-3 {
        format!("{:.2e}", value)
    } else if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.3}", value).trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_range() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (min, max) = calculate_range(&data);
        assert_eq!(min, 1.0);
        assert_eq!(max, 5.0);
    }

    #[test]
    fn test_generate_ticks() {
        let ticks = generate_ticks(0.0, 10.0, 6);
        assert!(!ticks.is_empty());
        assert!(ticks[0] <= 0.0);
        assert!(ticks[ticks.len() - 1] >= 10.0);
    }

    #[test]
    fn test_map_range() {
        assert_eq!(map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(map_range(0.0, 0.0, 10.0, 0.0, 100.0), 0.0);
        assert_eq!(map_range(10.0, 0.0, 10.0, 0.0, 100.0), 100.0);
    }
}