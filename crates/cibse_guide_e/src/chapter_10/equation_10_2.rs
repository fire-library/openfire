pub fn min_separation_dist(v_e: f64) -> f64 {
    return 0.9 * v_e.powf(0.5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        let result = min_separation_dist(0.3);
        assert_eq!(result, 0.4929503017546495);
    }
}
