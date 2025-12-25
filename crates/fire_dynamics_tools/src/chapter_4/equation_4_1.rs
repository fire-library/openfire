pub fn wall_fire_flame_height(q: f64) -> f64 {
    0.034 * q.powf(2.0 / 3.0)
}

#[cfg(not(coverage))]
pub fn wall_fire_flame_height_equation(h_f: String, q: String) -> String {
    format!("{} = 0.034 \\cdot {}^{{\\frac{{2}}{{3}}}}", h_f, q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_fire_flame_height() {
        let q = 700.0;
        let expected = 2.680469955456;

        let result = wall_fire_flame_height(q);

        assert!((result - expected).abs() < 1e-4,);
    }
}
