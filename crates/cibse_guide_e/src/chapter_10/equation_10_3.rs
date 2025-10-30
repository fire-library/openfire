pub fn volumetric_flow_rate(m: f64, t_s: f64, rho_0: f64, t_0: f64) -> f64 {
    return (m * t_s) / (rho_0 * t_0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        let result = volumetric_flow_rate(2.0, 473.0, 1.2, 293.0);
        assert_eq!(result, 2.6905574516496022);
    }
}
