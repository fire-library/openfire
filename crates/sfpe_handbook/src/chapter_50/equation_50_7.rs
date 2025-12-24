pub fn effective_area(a_sr: f64, a_ir: f64, a_io: f64) -> f64 {
    (1.0 / a_sr.powf(2.0) + 1.0 / a_ir.powf(2.0) + 1.0 / a_io.powf(2.0)).powf(-0.5)
}

#[cfg(not(coverage))]
pub fn effective_area_equation(a_sr: String, a_ir: String, a_io: String) -> String {
    format!(
        "{} = \\left(\\frac{1}{{{}}}^2 + \\frac{1}{{{}}}^2 + \\frac{1}{{{}}}^2\\right)^{{-0.5}}",
        "A_{effective}", a_sr, a_ir, a_io
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_area() {
        let result = effective_area(0.5, 0.75, 2.5);
        let expected = 0.41038174;
        assert!((result - expected).abs() < 1e-6);
    }
}
