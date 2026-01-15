pub fn visibility(k: f64, delta_m: f64, c_i: f64) -> f64 {
    k / (2.303 * delta_m * c_i)
}

#[cfg(not(coverage))]
pub fn visibility_equation(
    s_i: String,
    k: String,
    delta_m: String,
    c_i: String,
) -> String {
    format!(
        "{} = \\frac{{{}}}{{2.303 \\times {} \\times {}}}",
        s_i, k, delta_m, c_i, 
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility() {
        let result = visibility(8.0, 0.22, 1.0);
        let expected = 15.78968144;
        assert!((result - expected).abs() < 1e-6);
    }
}
