pub fn net_convective_heat_flux_surface(alpha_c: f64, theta_g: f64, theta_m: f64) -> f64 {
    alpha_c * (theta_g - theta_m)
}

#[cfg(not(coverage))]
pub fn net_convective_heat_flux_surface_equation(
    h_net_c: String,
    alpha_c: String,
    theta_g: String,
    theta_m: String,
) -> String {
    format!(
        "{} = {} \\cdot ({} - {})",
        h_net_c, alpha_c, theta_g, theta_m
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_convective_heat_flux_surface() {
        let result = net_convective_heat_flux_surface(50.0, 650.0, 150.0);
        assert_eq!(result, 25000.0);
    }
}
