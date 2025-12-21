pub fn pressure_difference(
    rho: f64,
    a_s: f64,
    a_e: f64,
    u: f64,
    a_a: f64,
    a_ir: f64,
    c_c: f64,
) -> f64 {
    (rho / 2.0) * ((a_s * a_e * u) / (a_a * a_ir * c_c)).powf(2.0)
}

pub fn pressure_difference_equation(
    delta_p_usi: String,
    rho: String,
    a_s: String,
    a_e: String,
    u: String,
    a_a: String,
    a_ir: String,
    c_c: String,
) -> String {
    format!(
        "{} = \\frac{{{}}}{{2}} \\cdot \\left(\\frac{{{} \\cdot {} \\cdot {}}}{{{} \\cdot {} \\cdot {}}}\\right)^2",
        delta_p_usi, rho, a_s, a_e, u, a_a, a_ir, c_c
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_difference() {
        let result = pressure_difference(0.8, 2.0, 1.5, 2.5, 1.0, 1.0, 0.84);
        let expected = 31.88775510;
        assert!((result - expected).abs() < 1e-6);
    }
}
