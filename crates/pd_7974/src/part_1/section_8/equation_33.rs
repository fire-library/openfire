pub fn q_max_vc(a_v: f64, h_v: f64) -> f64 {
    1500.0 * a_v * h_v.powf(0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero() {
        let result = q_max_vc(0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive() {
        let result = q_max_vc(3.0, 9.0);

        assert!((result - 13500.0).abs() < f64::EPSILON);
    }
}
