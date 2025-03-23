/// Calculate the radiation intensity.
/// sigma: Stefan-Boltzmann constant (should be: 5.67 x 10^-11 kW/m^2K^4)
/// emissivity: Emissivity of the surface
/// temperature: Temperature of the surface (K)
pub fn radiation_intensity(sigma: f64, emissivity: f64, temperature: f64) -> f64 {
    sigma * emissivity * temperature.powi(4)
}

pub fn radiation_intensity_equation(
    q_symbol: &str,
    sigma: &str,
    emissivity: &str,
    temperature: &str,
) -> String {
    format!(
        "{} = {} \\cdot {} \\cdot {}^4",
        q_symbol, sigma, emissivity, temperature
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiation_intensity() {
        let sigma = 5.67e-11;
        let emissivity = 0.9;
        let temperature = 300.0;
        let expected = 0.9 * 5.67e-11 * 300.0_f64.powi(4);
        let result = radiation_intensity(sigma, emissivity, temperature);
        assert!((expected - result).abs() < f64::EPSILON);
    }
}
