pub fn external_temp_time_curve(t: f64) -> f64 {
    660.0
        * (1.0
            - 0.687 * std::f64::consts::E.powf(-0.32 * t)
            - 0.313 * std::f64::consts::E.powf(-3.8 * t))
        + 20.0
}

pub fn external_temp_time_curve_equation(theta_g: String, t: String) -> String {
    format!(
        "{} = 660 \\cdot \\left( 1 - 0.687 \\cdot e^{{-0.32 \\cdot {}}} - 0.313 \\cdot e^{{-3.8 \\cdot {}}} \\right) + 20",
        theta_g, t, t
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_temp_time_curve() {
        let result = external_temp_time_curve(10.0);
        let expected = 661.51760147213;
        assert!((result - expected).abs() < 1e-6);
    }
}
