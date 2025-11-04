pub fn placeholder_equation_2_13(rho_g: Vec<f64>) -> Vec<f64> {
    rho_g.iter().map(|&rho_g_val| 0.076 / rho_g_val).collect()
}

pub fn placeholder_equation_2_13_latex(
    result: String,
    rho_g: String,
) -> String {
    format!("{} = \\frac{{0.076}}{{{}}} ", result, rho_g,)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_placeholder_equation_2_13() {
        let rho_g = vec![0.75, 0.5, 0.25];

        let result = placeholder_equation_2_13(rho_g);

        let expected_result = vec![0.1013333333, 0.1520000000, 0.3040000000];

        assert_eq!(
            result.len(),
            expected_result.len(),
            "Should return same number of results as input densities"
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