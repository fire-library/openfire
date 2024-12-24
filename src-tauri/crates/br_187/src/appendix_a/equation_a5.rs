pub fn x(w: f64, s: f64) -> f64 {
    w / s
}

pub fn y(h: f64, s: f64) -> f64 {
    h / s
}

pub fn phi(x: f64, y: f64) -> f64 {
    let a = 1.0 / (y.powi(2) + 1.0).sqrt();
    let b = x / (y.powi(2) + 1.0).sqrt();

    let multiple = 1.0 / (2.0 * std::f64::consts::PI);
    let first = x.atan();
    let second = a * b.atan();

    return multiple * (first - second);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x() {
        let w = 3.0;
        let s = 5.0;
        let expected = 0.6;
        let result = x(w, s);
        assert!((expected - result).abs() < f64::EPSILON);
    }

    #[test]
    fn test_y() {
        let h = 2.5;
        let s = 5.0;
        let expected = 0.5;
        let result = y(h, s);
        assert!((expected - result).abs() < f64::EPSILON);
    }

    #[test]
    fn test_phi() {
        let x = 0.6;
        let y = 0.5;
        let expected = 0.015896008909912128;
        let result = phi(x, y);
        println!("{}", result);
        assert!((expected - result).abs() < f64::EPSILON);
    }
}
