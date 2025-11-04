pub fn wall_flame_height_correlation(q: Vec<f64>) -> Vec<f64> {
    q.iter()
        .map(|&q_val| 0.034 * q_val.powf(2.0 / 3.0))
        .collect()
}

pub fn wall_flame_height_correlation_equation(h_f: String, q: String) -> String {
    format!("{} = 0.034 \\cdot {}^{{\\frac{{2}}{{3}}}}", h_f, q)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_wall_flame_height_correlation() {
        let input_values = vec![350.0, 700.0, 1050.0];

        let result = wall_flame_height_correlation(input_values);

        let expected_result = vec![1.688590260245, 2.680469955456, 3.512409284098];

        assert_eq!(
            result.len(),
            expected_result.len(),
            "Should return same number of results as input values"
        );

        for (i, (actual, expected)) in result.iter().zip(expected_result.iter()).enumerate() {
            assert!(
                (actual - expected).abs() < 1e-4,
                "Result at index {} should be approximately {}, but got {}",
                i,
                expected,
                actual
            );
        }
    }
}
