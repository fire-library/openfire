pub fn height_smoke_layer_interface_natural_ventilation(
    k: f64,
    q: f64,
    t: f64,
    a_c: f64,
    h_c: f64,
) -> f64 {
    let top_left = 2.0 * k * q.powf(1.0 / 3.0) * t;
    let bottom_left = 3.0 * a_c;
    (top_left / bottom_left + 1.0 / h_c.powf(2.0 / 3.0)).powf(-3.0 / 2.0)
}

pub fn height_smoke_layer_interface_natural_ventilation_equation(
    z: String,
    k: String,
    q: String,
    t: String,
    a_c: String,
    h_c: String,
) -> String {
    format!(
        "{} = \\left( \\frac{{2 \\cdot {} \\cdot {}^{{1/3}} \\cdot {} }}{{3 \\cdot {} }} + \\frac{{1}}{{ {}^{{2/3}} }} \\right)^{{-3/2}}",
        z, k, q, t, a_c, h_c
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_smoke_layer_interface_natural_ventilation() {
        let k = 0.12;
        let q = 1000.0;
        let a_c = 250.0;
        let h_c = 4.5;
        let time_values = 90.0;
        let expected_results = 1.886933556;

        let result = height_smoke_layer_interface_natural_ventilation(k, q, time_values, a_c, h_c);

        assert!(
            (result - expected_results).abs() < 1e-8,
            "Result should be approximately {}, but got {}",
            expected_results,
            result
        );
    }
}
