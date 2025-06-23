pub fn calculate_exit_width(b: f64, d: f64, s_up: f64, s_dn: f64, x: f64) -> f64 {
    let w_fe;
    if b > 60.0 && d < 2.0 {
        w_fe = s_up + s_dn;
    } else {
        w_fe = (b * x) + 0.75 * s_up;
    }
    return w_fe;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero_condition1() {
        let result = calculate_exit_width(61.0, 1.5, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_zero_condition2a() {
        let result = calculate_exit_width(59.0, 1.5, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_zero_condition2b() {
        let result = calculate_exit_width(61.0, 2.5, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive_condition1() {
        let result = calculate_exit_width(59.0, 1.5, 1000.0, 1000.0, 3.6);

        assert!((result - 962.4).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_positive_condition2a() {
        let result = calculate_exit_width(61.0, 1.5, 1000.0, 1000.0, 3.6);

        assert!((result - 2000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_positive_condition2b() {
        let result = calculate_exit_width(59.0, 2.5, 1000.0, 1000.0, 3.6);

        assert!((result - 962.4).abs() < f64::EPSILON);
    }
}
