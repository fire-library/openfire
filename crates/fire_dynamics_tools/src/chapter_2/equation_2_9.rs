pub fn convective_heat_transfer_coefficient(k: f64, rho: f64, c: f64, t: f64, delta: f64) -> f64 {
    let left = (k * rho * c / t).powf(0.5);
    let right = k / delta;
    0.4 * left.max(right)
}

pub fn convective_heat_transfer_coefficient_equation(
    h_k: String,
    k: String,
    rho: String,
    c: String,
    t: String,
    delta: String,
) -> String {
    format!(
        "{} = 0.4 \\cdot \\max \\left( \\left( \\frac{{ {} \\cdot {} \\cdot {} }}{{ {} }} \\right)^{{1/2}}, \\frac{{{}}}{{{}}} \\right)",
        h_k, k, rho, c, t, k, delta
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convective_heat_transfer_coefficient() {
        let k = 0.002;
        let rho = 2400.0;
        let c = 1.17;
        let t = 180.0;
        let delta = 0.2;
        let expected_result = 0.07065408693062278;

        let result = convective_heat_transfer_coefficient(k, rho, c, t, delta);

        assert!(
            (result - expected_result).abs() < 1e-6,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
