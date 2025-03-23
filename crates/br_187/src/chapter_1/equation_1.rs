pub mod openfire_runner;

pub fn calculate_ventilation_factor(a_s: f64, a: f64, h: f64) -> f64 {
    return a_s / (a * h.sqrt());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero() {
        let result = calculate_ventilation_factor(0.0, 1.0, 1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive() {
        let result = calculate_ventilation_factor(37.215, 1.785, 2.1);

        let custom_epsilon = 0.001;
        assert!((result - 14.386997).abs() < custom_epsilon);
    }
}
