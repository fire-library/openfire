pub fn density_hot_gas_layer(t_g: f64) -> f64 {
    353.0 / t_g
}

pub fn density_hot_gas_layer_equation(rho_g: String, t_g: String) -> String {
    format!("{} = \\frac{{353.0}}{{{}}} ", rho_g, t_g,)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_hot_gas_layer() {
        let t_g = 500.0;

        let result = density_hot_gas_layer(t_g);

        let expected_result = 0.706;

        assert!(
            (result - expected_result).abs() < 1e-4,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
