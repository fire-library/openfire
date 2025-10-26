pub fn hot_gas_temperature_natural_ventilation_mqh(
    q: Vec<f64>,
    a_v: Vec<f64>,
    h_v: Vec<f64>,
    a_t: f64,
    h_k: f64,
) -> Vec<f64> {
    let area_av_times_hv: f64 = a_v
        .iter()
        .zip(h_v.iter())
        .map(|(av, hv)| av * hv.powf(0.5))
        .sum();

    q.iter()
        .map(|&q| 6.85 * (q.powf(2.0) / ((area_av_times_hv) * (a_t * h_k))).powf(1.0 / 3.0))
        .collect()
}

pub fn hot_gas_temperature_natural_ventilation_mqh_equation(
    delta_t_g: String,
    q: String,
    a_v: String,
    h_v: String,
    a_t: String,
    h_k: String,
) -> String {
    format!(
        "{} = 6.85 \\cdot \\left( \\frac{{{}^{{2}}}}{{\\left(\\sum_{{i}} {} \\cdot \\sqrt{{{}}}\\right) \\cdot ({} \\cdot {})}} \\right)^{{1/3}}",
        delta_t_g, q, a_v, h_v, a_t, h_k
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_gas_temperature_natural_ventilation_mqh() {
        let q = vec![500.0, 1000.0, 1500.0];
        let a_v = vec![2.5, 1.5];
        let h_v = vec![2.0, 1.0];
        let a_t = 75.0;
        let h_k = 0.035;
        let expected_results = vec![182.5067636, 289.7114284, 379.6293664];

        let results = hot_gas_temperature_natural_ventilation_mqh(q, a_v, h_v, a_t, h_k);

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
