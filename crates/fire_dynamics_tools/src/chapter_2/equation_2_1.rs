pub fn hot_gas_temperature_natural_ventilation(q: f64, a_v: f64, h_v: f64, a_t: f64, h_k: f64) -> f64 {
    return 6.85 * (q.powf(2.0)/ ((a_v * h_v.powf(0.5)) + (a_t * h_k))).powf(1.0/3.0);
}

pub fn hot_gas_temperature_natural_ventilation_equation(delta_t_g: String, q: String, a_v:String, h_v: String, a_t: String, h_k: String) -> String {
    format!("{} = 6.85 \\cdot \\left( \\frac{{{}}}{{ {} \\cdot {} + {} \\cdot {} }} \\right)^{{1/3}}", delta_t_g, q, a_v, h_v, a_t, h_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_gas_temperature_natural_ventilation() {
        // Test with typical values for fire dynamics
        let q = 500.0; // kW (heat release rate)
        let a_v = 20.0; // m² (area of vertical opening)
        let h_v = 2.75; // m (height of vertical opening)
        let a_t = 30.0; // m² (area of horizontal opening)
        let h_k = 1.5; // m (height parameter for horizontal opening)

        let result = hot_gas_temperature_natural_ventilation(q, a_v, h_v, a_t, h_k);
        
        // Manual calculation:
        // denominator = (20.0 * 2.75^0.5) + (30.0 * 1.5) = 20.0 * 1.658 + 45.0 = 33.16 + 45.0 = 78.16
        // numerator = 500^2 = 250,000
        // fraction = 250,000 / 78.16 = 3,199.23
        // result = 6.85 * (3,199.23)^(1/3) = 6.85 * 14.75 ≈ 101.0
        let expected_result = 101.0;
        
        assert!(
            (result - expected_result).abs() < 1.0,
            "Result should be approximately {} K, but got {} K",
            expected_result, result
        );
    }

    #[test]
    fn test_hot_gas_temperature_with_different_values() {
        // Test with smaller heat release rate
        let q = 100.0; // kW
        let a_v = 10.0; // m²
        let h_v = 2.0; // m
        let a_t = 15.0; // m²
        let h_k = 1.0; // m

        let result = hot_gas_temperature_natural_ventilation(q, a_v, h_v, a_t, h_k);
        
        // With smaller Q, temperature should be lower
        assert!(result > 0.0, "Temperature should be positive");
        assert!(result < 200.0, "Temperature should be reasonable for this heat release rate");
    }

    #[test]
    fn test_hot_gas_temperature_equation_formatting() {
        // Test the LaTeX equation formatting
        let delta_t_g = "\\Delta T_g".to_string();
        let q = "Q".to_string();
        let a_v = "A_v".to_string();
        let h_v = "H_v".to_string();
        let a_t = "A_t".to_string();
        let h_k = "H_k".to_string();
        
        let equation = hot_gas_temperature_natural_ventilation_equation(delta_t_g, q, a_v, h_v, a_t, h_k);
        let expected = "\\Delta T_g = 6.85 \\cdot \\left( \\frac{Q}{ A_v \\cdot H_v + A_t \\cdot H_k } \\right)^{1/3}";
        
        assert_eq!(equation, expected, "LaTeX equation should match expected format");
    }
}
