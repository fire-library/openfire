pub fn q_fo(a_t: f64, a_v: f64, h_v: f64) -> f64 {
    return 7.8 * a_t + 378.0 * a_v * h_v.powf(0.5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero() {
        let result = q_fo(0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive() {
        let result = q_fo(1.0, 2.0, 9.0);

        assert!((result - 2275.8).abs() < f64::EPSILON);
    }
}
