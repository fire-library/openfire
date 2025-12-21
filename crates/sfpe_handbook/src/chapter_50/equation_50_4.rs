pub fn wind_pressure(c_w: f64, rho_0: f64, u_h: f64) -> f64 {
    0.5 * c_w * rho_0 * u_h.powf(2.0)
}

pub fn pressure_exerted_wind_equation(
    p_w: String,
    c_w: String,
    rho_0: String,
    u_h: String,
) -> String {
    format!(
        "{} = 0.5 \\cdot {} \\cdot {} \\cdot {}^2",
        p_w, c_w, rho_0, u_h
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_exerted_wind() {
        let result = wind_pressure(0.8, 1.2, 15.0);
        let expected = 108.0;
        assert!((result - expected).abs() < 1e-6);
    }
}
