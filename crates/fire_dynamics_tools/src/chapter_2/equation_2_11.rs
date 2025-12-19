pub fn k_constant_smoke_layer_height(rho_g: f64, rho_a: f64, g: f64, c_p: f64, t_a: f64) -> f64 {
    let right_top = rho_a.powf(2.0) * g;
    let right_bottom = c_p * t_a;
    let right_side = (right_top / right_bottom).powf(1.0 / 3.0);

    (0.21 / rho_g) * right_side
}

pub fn k_constant_smoke_layer_height_equation(
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
    use super::*;

    #[test]
    fn test_k_constant_smoke_layer_height() {
        let rho_g = 0.5;
        let rho_a = 1.2;
        let g = 9.81;
        let c_p = 1.0;
        let t_a = 293.15;

        let result = k_constant_smoke_layer_height(rho_g, rho_a, g, c_p, t_a);

        let expected_result = 0.1528374644;

        assert!(
            (result - expected_result).abs() < 1e-4,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
