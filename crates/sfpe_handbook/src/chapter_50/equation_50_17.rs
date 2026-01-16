pub fn stairwell_temperature(t_0: f64, eta: f64, t_b: f64) -> f64 {
    t_0 + eta * (t_b - t_0)
}

#[cfg(not(coverage))]
pub fn stairwell_temperature_equation(t_0: String, eta: String, t_b: String) -> String {
    format!("{} = {} + {} \\times ( {} - {} )", t_0, t_0, eta, t_b, t_0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stairwell_temperature() {
        let result = stairwell_temperature(-10.0, 0.15, 15.0);
        let expected = -6.25;
        assert!((result - expected).abs() < 1e-6);
    }
}
