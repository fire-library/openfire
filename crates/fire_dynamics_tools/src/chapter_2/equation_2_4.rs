pub fn thermal_penetration_time(rho: f64, c_p: f64, k: f64, delta: f64) -> f64 {
    ((c_p * rho) / (k)) * (delta / 2.0).powf(2.0)
}

#[cfg(not(coverage))]
pub fn thermal_penetration_time_equation(
    t_p: String,
    rho: String,
    c_p: String,
    k: String,
    delta: String,
) -> String {
    format!(
        "{} = \\frac{{{} \\cdot {}}}{{{}}} \\cdot \\left( \\frac{{{}}}{2} \\right)^2}}",
        t_p, c_p, rho, k, delta
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_penetration_time() {
        let rho = 2400.0;
        let c_p = 1.17;
        let k = 0.002;
        let delta = 0.25;

        let result = thermal_penetration_time(rho, c_p, k, delta);
        let expected_result = 21937.5;

        assert!(
            (result - expected_result).abs() < 1.0e-6,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
