pub fn height_smoke_layer_interface(k: f64, q: f64, t: Vec<f64>, a_c: f64, h_c: f64) -> Vec<f64> {
    return t.iter().map(|&time| {
        (((2.0 * k * q.powf(1.0/3.0) * time) / (3.0 * a_c)) + (1.0 / h_c.powf(2.0/3.0))).powf(-3.0/2.0)
    }).collect();
}

pub fn height_smoke_layer_interface_equation(z: String, k: String, q: String, t: String, a_c: String, h_c: String) -> String {
    return format!("{} = \\left( \\frac{{2 \\cdot {} \\cdot {}^{{1/3}} \\cdot {} }}{{3 \\cdot {} }} + \\frac{{1}}{{ {}^{{2/3}} }} \\right)^{{-3/2}}", z, k, q, t, a_c, h_c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_smoke_layer_interface() {
        let time_values = vec![100.0, 200.0, 300.0];
        let results = height_smoke_layer_interface(1.0, 100.0, time_values, 0.5, 3.0);
        
        assert_eq!(results.len(), 3, "Should return same number of results as input times");
        for result in results {
            assert!(result > 0.0, "All results should be positive");
        }
    }

    #[test]
    fn test_height_smoke_layer_interface_single_value() {
        let time_values = vec![300.0];
        let results = height_smoke_layer_interface(1.0, 100.0, time_values, 0.5, 3.0);
        
        assert_eq!(results.len(), 1, "Should return one result for one time value");
        assert!(results[0] > 0.0, "Result should be positive");
    }

    #[test]
    fn test_height_smoke_layer_interface_empty_vector() {
        let time_values = vec![];
        let results = height_smoke_layer_interface(1.0, 100.0, time_values, 0.5, 3.0);
        
        assert_eq!(results.len(), 0, "Should return empty vector for empty input");
    }
}
