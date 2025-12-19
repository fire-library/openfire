pub fn comparment_interior_surface_area(w_c: f64, l_c: f64, h_c: f64, a_v: f64) -> f64 {
    (2.0 * (w_c * l_c) + 2.0 * (h_c * w_c) + 2.0 * (h_c * l_c)) - a_v
}

pub fn comparment_interior_surface_area_equation(
    a_t: String,
    w_c: String,
    l_c: String,
    h_c: String,
    a_v: String,
) -> String {
    format!(
        "{} = 2 \\cdot ({} \\cdot {}) + 2 \\cdot ({} \\cdot {}) + 2 \\cdot ({} \\cdot {}) - {}",
        a_t, w_c, l_c, h_c, w_c, h_c, l_c, a_v
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compartment_interior_surface_area() {
        let w_c = 7.5;
        let l_c = 4.0;
        let h_c = 2.75;
        let a_v = 4.5;

        let result = comparment_interior_surface_area(w_c, l_c, h_c, a_v);
        let expected_result = 118.75;

        assert!(
            (result - expected_result).abs() < 1.0e-6,
            "Result should be approximately {}, but got {}",
            expected_result,
            result
        );
    }
}
