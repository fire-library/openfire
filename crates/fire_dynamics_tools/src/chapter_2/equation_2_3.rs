pub fn heat_transfer_coefficient_longtimes_or_thinwalls(k: f64, delta: f64) -> f64 {
    return k / delta;
}

pub fn heat_transfer_coefficient_longtimes_or_thinwalls_equation(
    h_k: String,
    k: String,
    delta: String,
) -> String {
    format!("{} = \\frac{{{}}}{{{}}}", h_k, k, delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_equation_2_3() {
        let k = 0.002;
        let delta = 0.25;

        let result = heat_transfer_coefficient_longtimes_or_thinwalls(k, delta);
        let expected_result = 0.008;

        assert!(
            (result - expected_result).abs() < 1.0e-6,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
