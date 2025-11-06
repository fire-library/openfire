pub fn heat_transfer_coefficient_shorttimes_or_thickwalls(k: f64, rho: f64, c: f64, t: f64) -> f64 {
    (k * rho * c / t).powf(0.5)
}

pub fn heat_transfer_coefficient_shorttimes_or_thickwalls_equation(
    h_k: String,
    k: String,
    rho: String,
    c: String,
    t: String,
) -> String {
    format!(
        "{} = \\left( \\frac{{ {} \\cdot {} \\cdot {} }}{{ {} }} \\right)^{{1/2}}",
        h_k, k, rho, c, t
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_transfer_coefficient_shortimes_or_thickwalls() {
        let k = 0.002;
        let rho = 2400.0;
        let c = 1.17;
        let t = 1800.0;

        let result = heat_transfer_coefficient_shorttimes_or_thickwalls(k, rho, c, t);
        let expected_result = 0.05585696018;

        assert!(
            (result - expected_result).abs() < 1.0e-6,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
