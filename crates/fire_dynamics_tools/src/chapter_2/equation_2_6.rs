pub fn hot_gas_temperature_natural_ventilation_beyler_closed_compartment(k: f64, rho: f64, c: f64, t: f64, m: f64, c_p: f64, q: Vec<f64>) -> Vec<f64> {
    let k_1 = 2.0 * 0.4 * (k * rho * c).powf(0.5) / (m * c_p);
    
    q.iter().map(|&q_val| {
        let k_2 = q_val / (m * c_p);
        let left = 2.0 * k_2 / k_1.powf(2.0);
        let parentheses = k_1 * t.powf(0.5) - 1.0 + (-k_1 * t.powf(0.5)).exp();
        left * parentheses
    }).collect()
}

pub fn hot_gas_temperature_natural_ventilation_beyler_closed_compartment_equation(delta_t_g: String, k: String, rho: String, c: String, t: String, m: String, c_p: String, q: String) -> String {
    format!("{} = \\frac{{2 \\cdot \\frac{{{}}}{{{}\\cdot{}}}}}{{\\left(\\frac{{2 \\cdot 0.4 \\cdot \\sqrt{{{}\\cdot{}\\cdot{}}}}}{{{}\\cdot{}}}\\right)^2}} \\cdot \\left(\\frac{{2 \\cdot 0.4 \\cdot \\sqrt{{{}\\cdot{}\\cdot{}}}}}{{{}\\cdot{}}} \\cdot \\sqrt{{{}}} - 1 + e^{{-\\frac{{2 \\cdot 0.4 \\cdot \\sqrt{{{}\\cdot{}\\cdot{}}}}}{{{}\\cdot{}}} \\cdot \\sqrt{{{}}}}}\\right)", 
        delta_t_g, q, m, c_p, k, rho, c, m, c_p, k, rho, c, m, c_p, t, k, rho, c, m, c_p, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_equation_2_6() {
        let k = 0.002;
        let rho = 2400.0;
        let c = 1.17;
        let m = 100.0;
        let c_p = 1.0;
        let q = vec![250.0, 500.0, 750.0];
        let t = 60.0;

        let result = hot_gas_temperature_natural_ventilation_beyler_closed_compartment(k, rho, c, t, m, c_p, q);
        let expected_result = vec![142.9192524, 285.8385048, 428.7577571];
        
        assert_eq!(result.len(), expected_result.len(), "Should return same number of results as input");
        
        for (i, (actual, expected)) in result.iter().zip(expected_result.iter()).enumerate() {
            assert!(
                (actual - expected).abs() < 1e-6,
                "Result at index {} should be approximately {}, but got {}",
                i, expected, actual
            );
        }
    }
}