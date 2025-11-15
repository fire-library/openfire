pub fn time_to_ignition_thermally_thin(
    rho: f64,
    c: f64,
    tau: f64,
    temp_ig: f64,
    temp_o: f64,
    q_r: f64,
) -> f64 {
    rho * c * tau * (temp_ig - temp_o) / q_r
}

pub fn time_to_ignition_thermally_thin_equation(
    t_ig: String,
    rho: String,
    c: String,
    tau: String,
    temp_ig: String,
    temp_o: String,
    q_r: String,
) -> String {
    format!(
        "{} = {} \\cdot {} \\cdot {} \\cdot {} \\cdot \\left( {} - {} \\right) \\cdot \\dfrac{{1}}{{{}}}",
        t_ig, rho, c, tau, temp_ig, temp_o, q_r
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_to_ignition_thermally_thick() {
        let result = time_to_ignition_thermally_thick(0.19, 1190.0, 1420.0, 300.0, 25.0, 20000.0);
        assert_eq!(result, 47.67428456490953);
    }
}