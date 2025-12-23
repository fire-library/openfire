pub fn door_opening_force(f_dc: f64, w: f64, a: f64, delta_p: f64, d: f64) -> f64 {
    f_dc + (w * a * delta_p) / (2.0 * (w - d))
}

pub fn door_opening_force_equation(
    f_dc: String,
    w: String,
    a: String,
    delta_p: String,
    d: String,
) -> String {
    format!(
        "{} = {} + \\frac{{{} \\times {} \\times {}}}{{2 \\times ({} - {})}}",
        "F_{door}", f_dc, w, a, delta_p, w, d
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door_opening_force() {
        let result = door_opening_force(40.0, 0.9, 1.9, 25.0, 0.05);
        let expected = 65.1470588235294;
        assert!((result - expected).abs() < 1e-6);
    }
}
