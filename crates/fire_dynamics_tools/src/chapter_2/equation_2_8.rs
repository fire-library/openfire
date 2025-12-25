pub fn hot_gas_temperature_increase(q: f64, m: f64, c_p: f64, h_k: f64, a_t: f64) -> f64 {
    q / ((m * c_p) + (h_k * a_t))
}

#[cfg(not(coverage))]
pub fn hot_gas_temperature_increase_equation(
    result: String,
    q: String,
    m: String,
    c_p: String,
    h_k: String,
    a_t: String,
) -> String {
    format!(
        "{} = \\frac{{{}}}{{({} \\cdot {} + {} \\cdot {})}}",
        result, q, m, c_p, h_k, a_t
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_gas_temperature_increase() {
        let q = 300.0;
        let m = 2.5;
        let c_p = 1.0;
        let h_k = 0.035;
        let a_t = 100.0;
        let expected_result = 50.0;

        let result = hot_gas_temperature_increase(q, m, c_p, h_k, a_t);

        assert!((result - expected_result).abs() < 1e-6,);
    }
}
