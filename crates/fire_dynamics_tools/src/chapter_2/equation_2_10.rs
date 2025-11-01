pub fn height_smoke_layer_interface_natural_ventilation_yamana_tanaka(
    k: Vec<f64>,
    q: Vec<f64>,
    t: Vec<f64>,
    a_c: f64,
    h_c: f64,
) -> Vec<f64> {
    k.iter()
        .zip(q.iter())
        .zip(t.iter())
        .map(|((&k_val, &q_val), &t_val)| {
            let top_left = 2.0 * k_val * q_val.powf(1.0 / 3.0) * t_val;
            let bottom_left = 3.0 * a_c;
            (top_left / bottom_left + 1.0 / h_c.powf(2.0 / 3.0)).powf(-3.0 / 2.0)
        })
        .collect()
}

pub fn height_smoke_layer_interface_natural_ventilation_yamana_tanaka_equation(
    z: String,
    k: String,
    q: String,
    t: String,
    a_c: String,
    h_c: String,
) -> String {
    format!(
        "{} = \\left( \\frac{{2 \\cdot {} \\cdot {}^{{1/3}} \\cdot {} }}{{3 \\cdot {} }} + \\frac{{1}}{{ {}^{{2/3}} }} \\right)^{{-3/2}}",
        z, k, q, t, a_c, h_c
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_smoke_layer_interface_natural_ventilation_yamana_tanaka() {
        let k = vec![0.15, 0.12, 0.1];
        let q = vec![500.0, 1000.0, 1500.0];
        let a_c = 250.0;
        let h_c = 4.5;
        let time_values = vec![60.0, 90.0, 120.0];
        let expected_results = vec![2.403177584, 1.886933556, 1.592853252];

        let results = height_smoke_layer_interface_natural_ventilation_yamana_tanaka(
            k,
            q,
            time_values,
            a_c,
            h_c,
        );

        assert_eq!(
            results.len(),
            3,
            "Should return same number of results as input times"
        );

        for (i, (actual, expected)) in results.iter().zip(expected_results.iter()).enumerate() {
            assert!(
                (actual - expected).abs() < 1e-8,
                "Result at index {} should be approximately {}, but got {}",
                i,
                expected,
                actual
            );
        }
    }
}
