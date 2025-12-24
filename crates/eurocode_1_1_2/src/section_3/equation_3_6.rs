pub fn hydrocarbon_temp_time_curve(t: f64) -> f64 {
    1080.0
        * (1.0
            - 0.325 * std::f64::consts::E.powf(-0.167 * t)
            - 0.675 * std::f64::consts::E.powf(-2.5 * t))
        + 20.0
}

#[cfg(not(coverage))]
pub fn hydrocarbon_temp_time_curve_equation(theta_g: String, t: String) -> String {
    format!(
        "{} = 1080 \\cdot \\left( 1 - 0.325 \\cdot e^{{-0.167 \\cdot {}}} - 0.675 \\cdot e^{{-2.5 \\cdot {}}} \\right) + 20",
        theta_g, t, t
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydrocarbon_temp_time_curve() {
        let result = hydrocarbon_temp_time_curve(10.0);
        let expected = 1033.92527995068;
        assert!((result - expected).abs() < 1e-6);
    }
}
