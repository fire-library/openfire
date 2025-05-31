pub mod openfire_runner;

pub fn mean_flame_height_655(q_t: f64) -> f64 {
    return 0.2 * q_t.powf(2.0 / 5.0);
}

pub fn mean_flame_height_656(q_t: f64, d_f: f64) -> f64 {
    return 0.235 * q_t.powf(2.0 / 5.0) - 1.02 * d_f;
}

pub fn equation_655(z_f: String, q_t: String) -> String {
    format!("{} = 0.2 * {} ^ {{2/5}}", z_f, q_t)
}

pub fn equation_656(z_f: String, q_t: String, d_f: String) -> String {
    format!("{} = 0.235 * {} ^ {{2/5}} - 1.02 * {}", z_f, q_t, d_f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_655() {
        let result = mean_flame_height_655(1000.0);
        assert_eq!(result, 3.1697863849222276);
    }

    fn test_656() {
        let result = mean_flame_height_656(1000.0, 1.5);
        assert_eq!(result, 2.19449900228362);
    }
}
