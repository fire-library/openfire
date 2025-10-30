pub fn vent_controlled_rate_of_burning(a_t: f64, a_o: f64, h_o: f64, w: f64, d: f64) -> f64 {
    return 0.02 * ((a_t - a_o) * (a_o * h_o.powf(0.5)) * (w / d)).powf(0.5);
}

pub fn equation(r: String, a_t: String, a_o: String, h_o: String, w: String, d: String) -> String {
    format!(
        "{} = 0.02 [({} - {}) \\cdot ({} \\sqrt {{{}}}) \\cdot ({} / {})]^{{1/2}}",
        r, a_t, a_o, a_o, h_o, w, d
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = vent_controlled_rate_of_burning(45.0, 2.1, 2.1, 3.0, 4.0);
        assert_eq!(result, 0.1979036228367894);
    }
}
