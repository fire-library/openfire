pub fn calculate_nondime_hrr(q_dot: f64, rho_a: f64, c_p: f64, t_a: f64, g: f64, h_e: f64) -> f64 {
    q_dot / (rho_a * c_p * t_a * g.powf(0.5) * h_e.powf(5.0 / 2.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero() {
        let result = calculate_nondime_hrr(0.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive() {
        let result = calculate_nondime_hrr(1000.0, 1.2, 1.0, 293.0, 9.8, 3.0);

        assert!((result - 0.058282068762112685).abs() < f64::EPSILON);
    }
}
