/// Calculate radiation intensity from a fire source using Stefan-Boltzmann law (Equation A1).
/// 
/// The radiation intensity is calculated using the Stefan-Boltzmann law:
/// 
/// ```text
/// I = σ ε T⁴
/// ```
/// 
/// Where:
/// - `I` is the radiation intensity (kW/m²)
/// - `σ` is the Stefan-Boltzmann constant (5.67 × 10⁻¹¹ kW/m²K⁴)
/// - `ε` is the surface emissivity (dimensionless, 0-1)
/// - `T` is the absolute temperature (K)
/// 
/// # Arguments
/// * `sigma` - Stefan-Boltzmann constant (kW/m²K⁴)
/// * `emissivity` - Surface emissivity (dimensionless, 0-1)
/// * `temperature` - Absolute temperature (K)
/// 
/// # Returns
/// Radiation intensity (kW/m²)
/// 
/// # Example
/// ```
/// use br_187::appendix_a::equation_a1::radiation_intensity;
/// 
/// let sigma = 5.67e-11; // kW/m²K⁴
/// let emissivity = 0.9;
/// let temperature = 1273.15; // K (1000°C)
/// let intensity = radiation_intensity(sigma, emissivity, temperature);
/// ```
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
