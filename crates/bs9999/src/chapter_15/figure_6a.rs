pub mod openfire_runner;

pub fn calculate_exit_width(s_up: f64, w_se: f64, n: f64, d: f64, x: f64) -> f64 {
    let w_fe;
    if n > 60.0 && d < 2.0 {
        w_fe = s_up + w_se;
    } else {
        w_fe = (n * x) + 0.75 * s_up;
    }
    return w_fe;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_zero_condition1() {
        let result = calculate_exit_width(0.0, 0.0, 61.0, 1.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_zero_condition2a() {
        let result = calculate_exit_width(0.0, 0.0, 59.0, 1.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_zero_condition2b() {
        let result = calculate_exit_width(0.0, 0.0, 61.0, 3.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_positive_condition1() {
        let result = calculate_exit_width(1000.0, 850.0, 61.0, 1.0, 3.6);

        assert!((result - 1850.00).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_positive_condition2a() {
        let result = calculate_exit_width(1000.0, 850.0, 59.0, 1.0, 3.6);

        assert!((result - 962.4).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_positive_condition2b() {
        let result = calculate_exit_width(1000.0, 850.0, 61.0, 3.0, 3.6);

        assert!((result - 969.6).abs() < f64::EPSILON);
    }
}
