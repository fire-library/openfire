pub fn hot_gas_temperature_increase(
    k: f64,
    rho: f64,
    c: f64,
    t: f64,
    m: f64,
    c_p: f64,
    q: f64,
) -> f64 {
    let k_1 = 2.0 * 0.4 * (k * rho * c).powf(0.5) / (m * c_p);

    let k_2 = q / (m * c_p);
    let left = 2.0 * k_2 / k_1.powf(2.0);
    let parentheses = k_1 * t.powf(0.5) - 1.0 + (-k_1 * t.powf(0.5)).exp();
    return left * parentheses;
}

pub fn hot_gas_temperature_increase_equation(
    delta_t_g: String,
    k: String,
    rho: String,
    c: String,
    t: String,
    m: String,
    c_p: String,
    q: String,
) -> String {
    format!(
        "{} = \\frac{{2 \\cdot \\frac{{{}}}{{{}\\cdot{}}}}}{{\\left(\\frac{{2 \\cdot 0.4 \\cdot \\sqrt{{{}\\cdot{}\\cdot{}}}}}{{{}\\cdot{}}}\\right)^2}} \\cdot \\left(\\frac{{2 \\cdot 0.4 \\cdot \\sqrt{{{}\\cdot{}\\cdot{}}}}}{{{}\\cdot{}}} \\cdot \\sqrt{{{}}} - 1 + e^{{-\\frac{{2 \\cdot 0.4 \\cdot \\sqrt{{{}\\cdot{}\\cdot{}}}}}{{{}\\cdot{}}} \\cdot \\sqrt{{{}}}}}\\right)",
        delta_t_g, q, m, c_p, k, rho, c, m, c_p, k, rho, c, m, c_p, t, k, rho, c, m, c_p, t
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_gas_temperature_increase() {
        let k = 0.002;
        let rho = 2400.0;
        let c = 1.17;
        let m = 100.0;
        let c_p = 1.0;
        let q = 500.0;
        let t = 60.0;

        let result = hot_gas_temperature_increase(k, rho, c, t, m, c_p, q);
        let expected_result = 285.8385048;

        assert!(
            (result - expected_result).abs() < 1e-6,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
