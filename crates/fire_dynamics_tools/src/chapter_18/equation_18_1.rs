pub fn visibility(k: f64, alpha_m: f64, m_p: f64) -> f64 {
    k / (alpha_m * m_p)
}

pub fn visibility_equation(s: String, k: String, alpha_m: String, m_p: String) -> String {
    format!("{} = \\frac{{ {} }}{{ {} \\cdot {} }}", s, k, alpha_m, m_p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility() {
        let k = 8.0;
        let alpha_m = 37000.0;
        let m_p = 0.000006;
        let expected = 36.03603604;

        let result = visibility(k, alpha_m, m_p);

        assert!(
            (result - expected).abs() < 1e-4,
            "Result should be approximately {}, but got {}",
            expected,
            result
        );
    }
}
