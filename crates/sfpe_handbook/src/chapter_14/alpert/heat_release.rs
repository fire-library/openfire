pub fn from_temperature_and_position(
    temp: f64,
    temp_amb: f64,
    height: f64,
    radial_position: f64,
) -> f64 {
    let mut q;
    if radial_position / height <= 0.18 {
        q = ((temp - temp_amb) * height.powf(5.0 / 3.0)) / 16.9;
        q = q.powf(3.0 / 2.0);
    } else {
        q = (temp - temp_amb) * (radial_position / height).powf(2.0 / 3.0);
        q *= height.powf(5.0 / 3.0);
        q /= 5.38;
        q = q.powf(3.0 / 2.0);
    }

    q
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_radial_position() {
        let temp = 500.0;
        let temp_amb = 300.0;
        let height = 10.0;
        let radial_position = 0.0;
        let result = from_temperature_and_position(temp, temp_amb, height, radial_position);
        let expected = ((temp - temp_amb) * height.powf(5.0 / 3.0) / 16.9).powf(3.0 / 2.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_radial_position_at_threshold() {
        let temp = 600.0;
        let temp_amb = 300.0;
        let height = 10.0;
        let radial_position = 0.18 * height;
        let result = from_temperature_and_position(temp, temp_amb, height, radial_position);
        let expected = ((temp - temp_amb) * height.powf(5.0 / 3.0) / 16.9).powf(3.0 / 2.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_radial_position_less_than_threshold() {
        let temp = 450.0;
        let temp_amb = 300.0;
        let height = 8.0;
        let radial_position = 1.0;
        let result = from_temperature_and_position(temp, temp_amb, height, radial_position);
        let expected = ((temp - temp_amb) * height.powf(5.0 / 3.0) / 16.9).powf(3.0 / 2.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_radial_position_greater_than_threshold() {
        let temp = 400.0;
        let temp_amb = 300.0;
        let height = 10.0;
        let radial_position = 3.0;
        let result = from_temperature_and_position(temp, temp_amb, height, radial_position);
        let expected = ((temp - temp_amb)
            * (radial_position / height).powf(2.0 / 3.0)
            * height.powf(5.0 / 3.0)
            / 5.38)
            .powf(3.0 / 2.0);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_equal_temperature_and_ambient() {
        let temp = 300.0;
        let temp_amb = 300.0;
        let height = 10.0;
        let radial_position = 2.0;
        let result = from_temperature_and_position(temp, temp_amb, height, radial_position);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_large_values() {
        let temp = 10000.0;
        let temp_amb = 3000.0;
        let height = 100.0;
        let radial_position = 50.0;
        let result = from_temperature_and_position(temp, temp_amb, height, radial_position);
        let expected = ((temp - temp_amb)
            * (radial_position / height).powf(2.0 / 3.0)
            * height.powf(5.0 / 3.0)
            / 5.38)
            .powf(3.0 / 2.0);
        assert!((result - expected).abs() < 1e-6);
    }
}
