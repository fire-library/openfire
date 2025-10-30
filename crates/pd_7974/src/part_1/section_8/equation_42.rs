pub fn omega(a_t: f64, a_v: f64, h_v: f64) -> f64 {
    a_t / (a_v * h_v.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omega_zero() {
        let result = omega(0.0, 0.0, 0.0);
        assert!(result.is_nan());
    }

    #[test]
    fn test_omega_positive() {
        let result = omega(1.0, 1.0, 1.0);

        assert!((result - 1.0).abs() < f64::EPSILON);
    }
}
