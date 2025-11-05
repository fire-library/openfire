pub fn q_fo(h_k: f64, a_t: f64, a_v: f64, h_v: f64) -> f64 {
    610.0 * (h_k * a_t * a_v * h_v.powf(0.5)).powf(0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero() {
        let result = q_fo(0.0, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive() {
        let result = q_fo(0.01, 10.0, 0.5, 1.0);
        assert!((result - 136.40014662748717).abs() < f64::EPSILON);
    }
}
