pub fn k_constant_smoke_layer_height_yamana_tanaka(
    rho_g: Vec<f64>,
    rho_a: f64,
    g: f64,
    c_p: f64,
    t_a: f64,
) -> Vec<f64> {
    let right_top = rho_a.powf(2.0) * g;
    let right_bottom = c_p * t_a;
    let right_side = (right_top / right_bottom).powf(1.0 / 3.0);

    rho_g
        .iter()
        .map(|&rho_g_val| (0.21 / rho_g_val) * right_side)
        .collect()
}

pub fn k_constant_smoke_layer_height_yamana_tanaka_equation(
    k: String,
    rho_g: String,
    rho_a: String,
    g: String,
    c_p: String,
    t_a: String,
) -> String {
    format!(
        "{} = \\frac{{0.21}}{{{}}} \\cdot \\left( \\frac{{{}^{{2}} \\cdot {} }}{{ {} \\cdot {} }} \\right)^{{1/3}}",
        k, rho_g, rho_a, g, c_p, t_a
    )
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_k_constant_smoke_layer_height() {
        let rho_g = vec![0.75, 0.5, 0.25];
        let rho_a = 1.2;
        let g = 9.81;
        let c_p = 1.0;
        let t_a = 293.15;

        let result = k_constant_smoke_layer_height_yamana_tanaka(rho_g, rho_a, g, c_p, t_a);

        let expected_result = vec![0.1018916429, 0.1528374644, 0.3056749288];

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
