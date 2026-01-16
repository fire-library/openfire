pub fn factor(a_sb: f64, a_bo: f64, t_b: f64, t_s: f64) -> f64 {
    let numerator = a_sb.powf(2.0) * (t_b + 273.0);
    let denominator = a_bo.powf(2.0) * (t_s + 273.0);
    1.0 + numerator / denominator
}

#[cfg(not(coverage))]
pub fn factor_equation(
    f_r: String,
    a_sb: String,
    t_b: String,
    a_bo: String,
    t_s: String,
) -> String {
    format!(
        "{} = 1 + \\frac{{ {}^2 \\times ( {} + 273 ) }}{{ {}^2 \\times ( {} + 273 ) }}",
        f_r, a_sb, t_b, a_bo, t_s
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        let result = factor(0.005, 0.005, 15.0, 5.0);
        let expected = 2.035971223;
        assert!((result - expected).abs() < 1e-6);
    }
}