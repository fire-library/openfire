pub fn maximum_centerline_temperature_plume(
    t_a: f64,
    q_c: f64,
    g: f64,
    c_p: f64,
    rho_a: f64,
    z: f64,
    z_o: f64,
) -> f64 {
    let top =
        9.1 * (t_a / (g * c_p.powf(2.0) * rho_a.powf(2.0))).powf(1.0 / 3.0) * (q_c).powf(2.0 / 3.0);
    let bottom = (z - z_o).powf(5.0 / 3.0);
    top / bottom
}

pub fn maximum_centerline_temperature_plume_equation(
    delta_t_p: String,
    t_a: String,
    q_c: String,
    g: String,
    c_p: String,
    rho_a: String,
    z: String,
    z_o: String,
) -> String {
    format!(
        "{} = 9.1 \\frac{{ {} }}{{ {} \\cdot {}^2 \\cdot {}^2 }} \\frac{{ {}^{{2/3}} }}{{ ({} - {})^{{5/3}} }}",
        delta_t_p, t_a, g, c_p, rho_a, q_c, z, z_o
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_centerline_temperature_plume() {
        let t_a = 288.0;
        let q_c = 700.0;
        let g = 9.8;
        let c_p = 1.0;
        let rho_a = 1.2;
        let z = 2.0;
        let z_o = -0.25;
        let expected = 507.4623919;

        let result = maximum_centerline_temperature_plume(t_a, q_c, g, c_p, rho_a, z, z_o);

        assert!(
            (result - expected).abs() < 1e-4,
            "Result should be approximately {}, but got {}",
            expected,
            result
        );
    }
}
