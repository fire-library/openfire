pub fn k_constant_smoke_layer_height(rho_g: f64) -> f64 {
    0.076 / rho_g
}

pub fn k_constant_smoke_layer_height_equation(k: String, rho_g: String) -> String {
    format!("{} = \\frac{{0.076}}{{{}}} ", k, rho_g,)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_constant_smoke_layer_height() {
        let rho_g = 0.5;

        let result = k_constant_smoke_layer_height(rho_g);

        let expected_result = 0.1520000000;

        assert!(
            (result - expected_result).abs() < 1e-4,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
