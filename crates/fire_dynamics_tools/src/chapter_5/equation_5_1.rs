pub fn thermal_radiation_point_source(q: f64, r: f64, x_r: f64) -> f64 {
    (x_r * q) / (4.0 * std::f64::consts::PI * r.powf(2.0))
}

pub fn thermal_radiation_point_source_equation(
    q_rad: String,
    q: String,
    r: String,
    x_r: String,
) -> String {
    format!(
        "{} = {{\\frac{{{} \\cdot {}}}{{4 \\cdot \\pi \\cdot {}^2}}}}",
        q_rad, x_r, q, r
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_fire_flame_height() {
        let q = 750.0;
        let r = 2.5;
        let x_r = 0.3;
        let expected = 2.864788975654;

        let result = thermal_radiation_point_source(q, r, x_r);

        assert!((result - expected).abs() < 1e-4,);
    }
}
