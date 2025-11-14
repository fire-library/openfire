pub fn virtual_origin_over_diameter(
    d: f64,
    q: f64,
) -> f64 {
    -1.02 + 0.083 * q.powf(2.0 / 5.0) / d
}

pub fn virtual_origin_over_diameter_equation(
    z_o_over_d: String,
    d: String,
    q: String,
) -> String {
    format!(
        "{} = -1.02 + 0.083 \\frac{{ {}^{{2/5}} }}{{ {} }}",
        z_o_over_d, q, d
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_origin_over_diameter() {
        let d = 2.2;
        let q = 750.0;
        let expected = -0.4870580374;

        let result = virtual_origin_over_diameter(d, q);

        assert!(
            (result - expected).abs() < 1e-4,
            "Result should be approximately {}, but got {}",
            expected,
            result
        );
    }
}