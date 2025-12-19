pub fn psi(m_e: f64, a_v: f64, a_t: f64) -> f64 {
    m_e / (a_v * a_t).powf(0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psi_zero() {
        let result = psi(0.0, 0.0, 0.0);
        assert!(result.is_nan());
    }

    #[test]
    fn test_psi_positive() {
        let result = psi(1.0, 1.0, 1.0);

        assert!((result - 1.0).abs() < f64::EPSILON);
    }
}
