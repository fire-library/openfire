pub fn visibility(k: f64, l: f64, lambda: f64) -> f64 {
    -k * l / ((1.0 - lambda / 100.0).ln())
}

#[cfg(not(coverage))]
pub fn visibility_equation(s_i: String, k: String, l: String, lambda: String) -> String {
    format!(
        "{} = \\dfrac{{{} \\times {}}}{{\\ln(1 - \\dfrac{{{}}}{{100}})}}",
        s_i, k, l, lambda,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility() {
        let result = visibility(8.0, 10.0, 95.0);
        let expected = 26.70465606;
        assert!((result - expected).abs() < 1e-6);
    }
}
