pub mod openfire_runner;

pub fn visibility(k: f64, d: f64) -> f64 {
    return k / (2.303 * d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        let result = visibility(8.0, 0.5);
        assert_eq!(result, 6.94745983499783);
    }
}
