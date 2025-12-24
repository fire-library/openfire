/// Equation A2: Radiation Intensity
/// phi = configuration factor
/// i_s = Radiation intensity at emitter (kW/m^2)
pub fn radiation_intensity_at_receiver(phi: f64, i_s: f64) -> f64 {
    phi * i_s
}

#[cfg(not(coverage))]
pub fn radiation_intensity_at_receiver_equation(q_symbol: &str, phi: &str, i_s: &str) -> String {
    format!("{} = {} \\cdot {}", q_symbol, phi, i_s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiation_intensity_at_receiver() {
        let phi = 0.5;
        let i_s = 100.0;
        let expected = 50.0;
        let result = radiation_intensity_at_receiver(phi, i_s);
        assert!((expected - result).abs() < f64::EPSILON);
    }
}
