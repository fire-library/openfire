pub fn calculate(omega: f64) -> f64 {
    let numerator = 1.0 - (-0.1 * omega).exp();
    let denominator = omega.sqrt();

    return 6000.0 * (numerator / denominator);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero() {
        let result = calculate(0.0);
        assert!(result.is_nan());
    }

    #[test]
    fn test_calculate_positive() {
        let result = calculate(1.0);

        assert!((result - 570.9754917842429).abs() < f64::EPSILON);
    }
}
