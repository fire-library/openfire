pub fn hot_gas_temperature_natural_ventilation_mqh(q: Vec<f64>, a_v: f64, h_v: f64, a_t: f64, h_k: f64) -> Vec<f64> {
    q.iter().map(|&q| {
        6.85 * (q.powf(2.0)/ ((a_v * h_v.powf(0.5)) * (a_t * h_k))).powf(1.0/3.0)
    }).collect()
}

pub fn hot_gas_temperature_natural_ventilation_mqh_equation(delta_t_g: String, q: String, a_v:String, h_v: String, a_t: String, h_k: String) -> String {
    format!("{} = 6.85 \\cdot \\left( \\frac{{{}^{{2}}}}{{({} \\cdot {}^{{0.5}}) \\cdot ({} \\cdot {})}} \\right)^{{1/3}}", delta_t_g, q, a_v, h_v, a_t, h_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_gas_temperature_natural_ventilation_mqh() {
        let q = vec![500.0, 1000.0, 1500.0];
        let a_v = 2.5;
        let h_v = 2.0;
        let a_t = 75.0;
        let h_k = 0.035;
        let expected_results = vec![205.3410616, 325.9586172, 427.1266204];

        let results = hot_gas_temperature_natural_ventilation_mqh(q, a_v, h_v, a_t, h_k);

        assert_eq!(results.len(), 3, "Should return same number of results as input times");
        for (i, (result, expected)) in results.iter().zip(expected_results.iter()).enumerate() {
            assert!(
                (result - expected).abs() < 1e-6,
                "Result at index {} should be approximately {}, but got {}",
                i, expected, result
            );
        }
        
    }

}
