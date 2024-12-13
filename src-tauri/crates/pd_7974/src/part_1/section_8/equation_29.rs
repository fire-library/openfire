pub fn q_fo(h_k: f64, a_t: f64, a_v: f64, h_v: f64) -> f64 {
    return 610.0 * (h_k * a_t * a_v * h_v.powf(0.5)).powf(0.5);
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
        let result = q_fo(1.0, 2.0, 3.0, 9.0);

        println!("{}", result);
        assert!((result - 2588.010819142764).abs() < f64::EPSILON);
    }
}
