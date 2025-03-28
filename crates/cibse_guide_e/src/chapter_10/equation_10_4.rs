pub mod openfire_runner;

pub fn time_burning_skin(q: f64) -> f64 {
    return 1.33 * q.powf(-1.35);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        let result = time_burning_skin(2.5);
        assert_eq!(result, 0.3860402864984395);
    }
}
