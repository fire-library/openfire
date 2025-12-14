pub fn standard_temp_time_curve(t: f64) -> f64 {
    20.0 + 345.0 * (8.0 * t + 1.0).log10()
}

pub fn standard_temp_time_curve_equation(delta_g: String, t: String) -> String {
    format!(
        "{} = 20 + 345 \\cdot \\log_{{10}}(8 \\cdot {} + 1)",
        delta_g, t,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_temp_time_curve() {
        let result = standard_temp_time_curve(10.0);
        let expected = 678.42733151313;
        assert!((result - expected).abs() < 1e-6);
    }
}
