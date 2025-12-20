pub fn pressure_exerted_wind(c_w: f64, rho_0: f64, u_h: f64) -> f64 {
    0.5 * c_w * rho_0 * u_h.powf(2.0)
}
    
pub fn pressure_exerted_wind_equation(p_w: String, c_w: String, rho_0: String, u_h: String) -> String {
    format!("{} = 0.5 \\cdot {} \\cdot {} \\cdot {}^2", p_w, c_w, rho_0, u_h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_exerted_wind() {
        let result = pressure_exerted_wind(20.0, 800.0, 1.52);
        let expected = 23296.0;
        assert!((result - expected).abs() < 1e-6);
    }
}