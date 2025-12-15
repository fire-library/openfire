pub fn net_radiative_heat_flux_surface(
    phi: f64,
    epsilon_m: f64,
    epsilon_f: f64,
    sigma: f64,
    theta_r: f64,
    theta_m: f64,
) -> f64 {
    phi * epsilon_m
        * epsilon_f
        * sigma
        * ((theta_r + 273.0).powf(4.0) - (theta_m + 273.0).powf(4.0))
}

pub fn net_radiative_heat_flux_surface_equation(
    h_net_r: String,
    phi: String,
    epsilon_m: String,
    epsilon_f: String,
    sigma: String,
    delta_r: String,
    delta_m: String,
) -> String {
    format!(
        "{} = {} \\cdot {} \\cdot {} \\cdot {} \\cdot \\left( ({} + 273)^4 - ({} + 273)^4 \\right)",
        h_net_r, phi, epsilon_m, epsilon_f, sigma, delta_r, delta_m
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_radiative_heat_flux_surface() {
        let result = net_radiative_heat_flux_surface(0.8, 0.8, 0.9, 5.67e-8, 650.0, 150.0);
        let expected = 22657.8893804928;
        assert!((result - expected).abs() < 1e-6);
    }
}
