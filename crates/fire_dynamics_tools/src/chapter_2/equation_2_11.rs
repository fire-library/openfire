pub fn height_smoke_layer_interface_2_11(k: f64, q: f64, t: Vec<f64>, a_c: f64, h_c: f64) -> Vec<f64> {
    t.iter().map(|&time| {
        (((2.0 * k * q.powf(1.0/3.0) * time) / (3.0 * a_c)) + (1.0 / h_c.powf(2.0/3.0))).powf(-3.0/2.0)
    }).collect()
}

pub fn height_smoke_layer_interface_2_11_equation(z: String, k: String, q: String, t: String, a_c: String, h_c: String) -> String {
    format!("{} = \\left( \\frac{{2 \\cdot {} \\cdot {}^{{1/3}} \\cdot {} }}{{3 \\cdot {} }} + \\frac{{1}}{{ {}^{{2/3}} }} \\right)^{{-3/2}}", z, k, q, t, a_c, h_c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_smoke_layer_interface_2_11() {
        // Test values: k = 0.025, q = 500, a_c = 20, h_c = 2.75, t = [25, 50, 75]
        let k = 0.025;
        let q = 500.0;
        let a_c = 20.0;
        let h_c = 2.75;
        let time_values = vec![25.0, 50.0, 75.0];
        let expected_results = vec![1.803939505, 1.298521222, 0.991770071];
        
        let results = height_smoke_layer_interface_2_11(k, q, time_values, a_c, h_c);
        
        assert_eq!(results.len(), 3, "Should return same number of results as input times");
        
        for (i, (actual, expected)) in results.iter().zip(expected_results.iter()).enumerate() {
            assert!(
                (actual - expected).abs() < 1e-8,
                "Result at index {} should be approximately {}, but got {}",
                i, expected, actual
            );
        }
    }

    #[test]
    fn test_height_smoke_layer_interface_2_11_single_value() {
        // Test with single time value t = 25
        let k = 0.025;
        let q = 500.0;
        let a_c = 20.0;
        let h_c = 2.75;
        let time_values = vec![25.0];
        let expected_result = 1.803939505;
        
        let results = height_smoke_layer_interface_2_11(k, q, time_values, a_c, h_c);
        
        assert_eq!(results.len(), 1, "Should return one result for one time value");
        assert!(
            (results[0] - expected_result).abs() < 1e-8,
            "Result should be approximately {}, but got {}",
            expected_result, results[0]
        );
    }

    #[test]
    fn test_height_smoke_layer_interface_2_11_empty_vector() {
        let k = 0.025;
        let q = 500.0;
        let a_c = 20.0;
        let h_c = 2.75;
        let time_values = vec![];
        
        let results = height_smoke_layer_interface_2_11(k, q, time_values, a_c, h_c);
        
        assert_eq!(results.len(), 0, "Should return empty vector for empty input");
    }
}