pub fn height_smoke_layer_interface(k: f64, q: f64, t: f64, a_c: f64, h_c: f64) -> f64 {
    return (((2.0 * k * q.powf(1.0/3.0) * t) / (3.0 * a_c)) + (1.0 / h_c.powf(2.0/3.0))).powf(-3.0/2.0);
}

pub fn height_smoke_layer_interface_equation(z: String, k: String, q: String, t: String, a_c: String, h_c: String) -> String {
    return format!("{} = \\left( \\frac{{2 \\cdot {} \\cdot {}^{{1/3}} \\cdot {} }}{{3 \\cdot {} }} + \\frac{{1}}{{ {}^{{2/3}} }} \\right)^{{-3/2}}", z, k, q, t, a_c, h_c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_smoke_layer_interface() {
        let result = height_smoke_layer_interface(1.0, 100.0, 300.0, 0.5, 3.0);
        assert!(result > 0.0, "Result should be positive");
    }

}
