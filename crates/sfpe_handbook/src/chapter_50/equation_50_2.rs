pub fn pressure_difference(t_0: f64, t_f: f64, z: f64) -> f64 {
    let t_0_abs = t_0 + 273.0;
    let t_f_abs = t_f + 273.0;
    3460.0 * (1.0 / (t_0_abs) - 1.0 / (t_f_abs)) * z
}

#[cfg(not(coverage))]
pub fn pressure_difference_equation(
    delta_p_so: String,
    t_0: String,
    t_f: String,
    z: String,
) -> String {
    format!(
        "{} = 3460 \\cdot \\left(\\frac{{1}}{{{} + 273}} - \\frac{{1}}{{{} + 273}}\\right) \\cdot {}",
        delta_p_so, t_0, t_f, z
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_difference_fire_compartment() {
        let result = pressure_difference(20.0, 800.0, 1.52);
        let expected = 13.04809010;
        assert!((result - expected).abs() < 1e-6);
    }
}
