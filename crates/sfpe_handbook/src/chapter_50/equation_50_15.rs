pub fn height_limit(f_r: f64, delta_p_max: f64, delta_p_min: f64, t_0: f64, t_s: f64) -> f64 {
    let numerator = f_r * (delta_p_max - delta_p_min);
    let denominator = (1.0 / (t_0 + 273.0) - 1.0 / (t_s + 273.0)).abs();
    0.000289 * (numerator / denominator)
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
        "{} = 0.000289 \\times \\dfrac{{ {} \\times ( {} - {} ) }}{{ \\left| \\dfrac{{1}}{{ {} + 273}} - \\dfrac{{1}}{{ {} + 273}} \\right| }}",
        h_m, f_r, delta_p_max, delta_p_min, t_0, t_s
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_limit() {
        let result = height_limit(2.0, 75.0, 25.0, 0.0, 25.0);
        let expected = 94.0452240;
        assert!((result - expected).abs() < 1e-6);
    }
}
