/// Calculates the result of equation 5-1.
///
/// This is a mock implementation of equation 5-1 with a fraction-based formula.
///
/// # Arguments
///
/// * `a` - First parameter (units: dimensionless)
/// * `b` - Second parameter (units: dimensionless)
/// * `c` - Third parameter (units: dimensionless)
/// * `d` - Fourth parameter (units: dimensionless)
///
/// # Returns
///
/// * `f64` - The calculated result (units: dimensionless)
///
/// # Example
///
/// ```
/// use fire_dynamics_tools::chapter_5::equation_5_1::calculate_equation_5_1;
///
/// let result = calculate_equation_5_1(10.0, 5.0, 2.0, 3.0);
/// assert!((result - 3.333333333333333).abs() < 1e-10);
/// ```
pub fn calculate_equation_5_1(a: f64, b: f64, c: f64, d: f64) -> f64 {
    (a * b) / (c * d)
}

/// Returns the LaTeX representation of equation 5-1.
///
/// # Arguments
///
/// * `result` - Symbol for the result variable
/// * `a` - Symbol for the first parameter
/// * `b` - Symbol for the second parameter
/// * `c` - Symbol for the third parameter
/// * `d` - Symbol for the fourth parameter
///
/// # Returns
///
/// * `String` - LaTeX equation string
pub fn calculate_equation_5_1_equation(
    result: String,
    a: String,
    b: String,
    c: String,
    d: String,
) -> String {
    format!(
        "{} = \\frac{{{} \\cdot {}}}{{{} \\cdot {}}}",
        result, a, b, c, d
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_equation_5_1_basic() {
        let result = calculate_equation_5_1(10.0, 5.0, 2.0, 3.0);
        let expected = (10.0 * 5.0) / (2.0 * 3.0);
        assert!((result - expected).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_equation_5_1_unity() {
        let result = calculate_equation_5_1(1.0, 1.0, 1.0, 1.0);
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_equation_5_1_zeros() {
        let result = calculate_equation_5_1(0.0, 5.0, 2.0, 3.0);
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_equation_5_1_fractional() {
        let result = calculate_equation_5_1(1.5, 2.5, 3.5, 4.5);
        let expected = (1.5 * 2.5) / (3.5 * 4.5);
        assert!((result - expected).abs() < 1e-10);
    }

    #[test]
    fn test_equation_latex_format() {
        let latex = calculate_equation_5_1_equation(
            "R".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        );
        let expected = "R = \\frac{a \\cdot b}{c \\cdot d}";
        assert_eq!(latex, expected);
    }
}