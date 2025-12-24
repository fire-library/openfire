pub fn time_to_ignition(rho: f64, c: f64, tau: f64, temp_ig: f64, temp_0: f64, q_r: f64) -> f64 {
    rho * c * tau * (temp_ig - temp_0) / q_r
}

#[cfg(not(coverage))]
pub fn time_to_ignition_equation(
    t_ig: String,
    rho: String,
    c: String,
    tau: String,
    temp_ig: String,
    temp_o: String,
    q_r: String,
) -> String {
    format!(
        "{} = {} \\cdot {} \\cdot {} \\cdot \\left( {} - {} \\right) \\cdot \\dfrac{{1}}{{{}}}",
        t_ig, rho, c, tau, temp_ig, temp_o, q_r
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_to_ignition() {
        let result = time_to_ignition(1190.0, 1420.0, 0.001, 573.0, 298.0, 20000.0);
        assert_eq!(result, 23.23475);
    }
}
