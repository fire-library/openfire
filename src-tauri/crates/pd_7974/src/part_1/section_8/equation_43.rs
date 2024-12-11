pub fn calculate(t_g_max: f64, psi: f64) -> f64 {
    t_g_max * (1.0 - (-0.05 * psi).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero() {
        let result = calculate(0.0, 0.0);
        assert!((result - 0.095) < f64::EPSILON);
    }

    #[test]
    fn test_calculate_positive() {
        let result = calculate(1.0, 1.0);

        println!("{}", result);
        assert!((result - 0.0487707973914495) < f64::EPSILON);
    }
}
