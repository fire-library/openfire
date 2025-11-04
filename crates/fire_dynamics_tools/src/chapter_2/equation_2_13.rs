pub fn density_hot_gas_layer(t_g: Vec<f64>) -> Vec<f64> {
    t_g.iter().map(|&t_g_val| 353.0 / t_g_val).collect()
}

pub fn density_hot_gas_layer_equation(rho_g: String, t_g: String) -> String {
    format!("{} = \\frac{{353.0}}{{{}}} ", rho_g, t_g,)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_density_hot_gas_layer() {
        let t_g = vec![400.0, 500.0, 600.0];

        let result = density_hot_gas_layer(t_g);

        let expected_result = vec![0.8825, 0.706, 0.5883333333333334];

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
