pub fn calculate_exit_width(
    b: f64,
    n: f64,
    d: f64,
    s_up: f64,
    s_dn: f64,
    w_se: f64,
    x: f64,
) -> f64 {
    let w_fe;
    if b + n > 60.0 && d < 2.0 {
        w_fe = s_up + s_dn + w_se;
    } else {
        w_fe = (b * x) + (n * x) + 0.75 * s_up;
    }
    return w_fe;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero_condition1() {
        let result = calculate_exit_width(30.0, 31.0, 1.5, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_zero_condition2a() {
        let result = calculate_exit_width(30.0, 29.0, 1.5, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_zero_condition2b() {
        let result = calculate_exit_width(30.0, 31.0, 2.5, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive_condition1() {
        let result = calculate_exit_width(30.0, 31.0, 1.5, 1000.0, 1000.0, 850.0, 3.6);

        assert!((result - 2850.00).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_positive_condition2a() {
        let result = calculate_exit_width(30.0, 29.0, 1.5, 1000.0, 1000.0, 850.0, 3.6);

        assert!((result - 962.4).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_positive_condition2b() {
        let result = calculate_exit_width(30.0, 29.0, 2.5, 1000.0, 1000.0, 850.0, 3.6);

        assert!((result - 962.4).abs() < f64::EPSILON);
    }
}
