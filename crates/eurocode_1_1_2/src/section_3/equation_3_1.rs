pub fn net_heat_flux_surface(h_net_c: f64, h_net_r: f64) -> f64 {
    h_net_c + h_net_r
}

#[cfg(not(coverage))]
pub fn equation(h_net: String, h_net_c: String, h_net_r: String) -> String {
    format!("{} = {} + {}", h_net, h_net_c, h_net_r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_heat_flux_surface() {
        let result = net_heat_flux_surface(15000.0, 25000.0);
        assert_eq!(result, 40000.0);
    }
}
