pub fn mean_flame_height(q_t: f64) -> f64 {
    0.2 * q_t.powf(2.0 / 5.0)
}

pub fn equation(z_f: String, q_t: String) -> String {
    format!("{} = 0.2 * {} ^ {{2/5}}", z_f, q_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = mean_flame_height(1000.0);
        assert_eq!(result, 3.1697863849222276);
    }
}
