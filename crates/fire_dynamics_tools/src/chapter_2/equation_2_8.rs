pub fn hot_gas_temperature_increase_forced_ventilation_deal_and_beyler(
    q: Vec<f64>,
    m: f64,
    c_p: f64,
    h_k: f64,
    a_t: f64,
) -> Vec<f64> {
    q.iter().map(|&q| q / ((m * c_p) + (h_k * a_t))).collect()
}

pub fn hot_gas_temperature_increase_forced_ventilation_deal_and_beyler_latex(
    result: String,
    q: String,
    m: String,
    c_p: String,
    h_k: String,
    a_t: String,
) -> String {
    format!(
        "{} = \\frac{{{}}}{{({} \\cdot {} + {} \\cdot {})}}",
        result, q, m, c_p, h_k, a_t
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_gas_temperature_increase_forced_ventilation_deal_and_beyler() {
        let q = vec![150.0, 300.0, 450.0];
        let m = 2.5;
        let c_p = 1.0;
        let h_k = 0.035;
        let a_t = 100.0;
        let expected_results = vec![25.0, 50.0, 75.0];

        let results =
            hot_gas_temperature_increase_forced_ventilation_deal_and_beyler(q, m, c_p, h_k, a_t);

        assert_eq!(
            results.len(),
            3,
            "Should return same number of results as input times"
        );
        for (i, (result, expected)) in results.iter().zip(expected_results.iter()).enumerate() {
            assert!(
                (result - expected).abs() < 1e-6,
                "Result at index {} should be approximately {}, but got {}",
                i,
                expected,
                result
            );
        }
    }
}
