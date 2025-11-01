pub fn nondimensional_hot_gas_temperature_increase_forced_ventilation_fpa(
    q: Vec<f64>,
    m: f64,
    t_a: f64,
    h_k: f64,
    a_t: f64,
    c_p: f64,
) -> Vec<f64> {
    q.iter()
        .map(|&q| 0.63 * ((q) / (m * c_p * t_a)).powf(0.72) * ((h_k * a_t) / (m * c_p)).powf(-0.36))
        .collect()
}

pub fn nondimensional_hot_gas_temperature_increase_forced_ventilation_fpa_equation(
    delta_t_g_over_t_a: String,
    q: String,
    m: String,
    t_a: String,
    h_k: String,
    a_t: String,
    c_p: String,
) -> String {
    format!(
        "{} = 0.63 \\cdot \\left( \\frac{{{}}}{{{} \\cdot {} \\ {}}} \\right)^{{0.72}} \\cdot \\left( \\frac{{{} \\cdot {}}}{{{} \\cdot {}}} \\right)^{{-0.36}}",
        delta_t_g_over_t_a, q, m, c_p, t_a, h_k, a_t, m, c_p
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nondimensional_hot_gas_temperature_increase_forced_ventilation_fpa() {
        let t_a = 293.0;
        let q = vec![150.0, 300.0, 450.0];
        let m = 2.5;
        let h_k = 0.035;
        let a_t = 100.0;
        let c_p = 1.0;

        let expected_results = vec![0.1781798833, 0.2934947027, 0.3929940486];

        let results = nondimensional_hot_gas_temperature_increase_forced_ventilation_fpa(
            q, m, t_a, h_k, a_t, c_p,
        );

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
