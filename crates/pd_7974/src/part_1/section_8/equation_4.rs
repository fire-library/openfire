pub fn q_max_fc(a_f: f64, hrrpua: f64) -> f64 {
    a_f * hrrpua
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = q_max_fc(10.0, 500.0);

        assert!((result - 5000.0).abs() < f64::EPSILON);
    }
}
