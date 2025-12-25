pub fn effective_diameter(a_f: f64) -> f64 {
    (4.0 * a_f / std::f64::consts::PI).powf(0.5)
}

#[cfg(not(coverage))]
pub fn effective_diameter_equation(d: String, a_f: String) -> String {
    format!("{} = (\\frac{{4 \\cdot {}}}{{\\pi}})^{{1/2}}", d, a_f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_diameter() {
        let a_f = 3.205;
        let expected = 2.02;

        let result = effective_diameter(a_f);

        assert!((result - expected).abs() < 1e-4,);
    }
}
