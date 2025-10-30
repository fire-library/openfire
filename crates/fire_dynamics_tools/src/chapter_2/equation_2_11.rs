pub fn k_constant_smoke_layer_height(rho_g: f64, rho_a: f64, g: f64, c_p: f64, t_a: f64) -> f64 {
    let left = 0.21 / rho_g;
    let right_top = rho_a.powf(2.0) * g;
    let right_bottom = c_p * t_a;
    return left * (right_top / right_bottom).powf(1.0 / 3.0);
}

pub fn height_smoke_layer_interface_2_11_equation(
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
        // Test with typical values for fire dynamics
        let rho_g = 0.3; // kg/m³ (hot gas density)
        let rho_a = 1.2; // kg/m³ (ambient air density)
        let g = 9.81; // m/s² (gravitational acceleration)
        let c_p = 1005.0; // J/(kg·K) (specific heat capacity of air)
        let t_a = 293.15; // K (ambient temperature = 20°C)

        let result = k_constant_smoke_layer_height(rho_g, rho_a, g, c_p, t_a);

        // Expected calculation:
        // left = 0.21 / 0.3 = 0.7
        // right_top = 1.2² * 9.81 = 14.1264
        // right_bottom = 1005.0 * 293.15 = 294,615.75
        // right = (14.1264 / 294,615.75)^(1/3) = (4.7927e-5)^(1/3) ≈ 0.03631
        // k = 0.7 * 0.03631 ≈ 0.02542
        let expected_result = 0.02542;

        assert!(
            (result - expected_result).abs() < 1e-4,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
