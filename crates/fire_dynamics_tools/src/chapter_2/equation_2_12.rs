pub fn k_constant_smoke_layer_height_yamana_tanaka_post_substitution(rho_g: Vec<f64>) -> Vec<f64> {
    rho_g.iter().map(|&rho_g_val| 0.076 / rho_g_val).collect()
}

pub fn k_constant_smoke_layer_height_yamana_tanaka_post_substitution_equation(
    k: String,
    rho_g: String,
) -> String {
    format!("{} = \\frac{{0.076}}{{{}}} ", k, rho_g,)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_k_constant_smoke_layer_height_yamana_tanaka_post_substitution() {
        let rho_g = vec![0.75, 0.5, 0.25];

        let result = k_constant_smoke_layer_height_yamana_tanaka_post_substitution(rho_g);

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
