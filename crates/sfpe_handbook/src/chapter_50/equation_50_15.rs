pub fn height_limit(f_r: f64, delta_p_max: f64, delta_p_min: f64, t_0: f64, t_s: f64) -> f64 {
    0.000289 * ((f_r * (delta_p_max - delta_p_min)) / (1/(t_0 + 273) - 1/(t_s + 273)).abs())
}

#[cfg(not(coverage))]
pub fn height_limit_equation(
    h_m: String,
    f_r: String,
    delta_p_max: String,
    delta_p_min: String,
    t_0: String,
    t_s: String,
) -> String {
    format!(
        "{} = 0.000289 \\times \\frac{{ {} \\times ( {} - {} ) }}{{ \\left| \\frac{{1}}{{ {} + 273}} - \\frac{{1}}{{ {} + 273}} \\right| }}",
        h_m, f_r, delta_p_max, delta_p_min, t_0, t_s
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_limit() {
        let result = height_limit(1.0, 2.0, 3.0, 4.0, 5.0);
        let expected = 7.0;
        assert!((result - expected).abs() < 1e-6);
    }
}