pub fn x(w: f64, s: f64) -> f64 {
    w / s
}

pub fn y(h: f64, s: f64) -> f64 {
    h / s
}

pub fn phi(x: f64, y: f64) -> f64 {
    let a = x / (1.0 + x.powi(2)).sqrt();
    let b = y / (1.0 + x.powi(2)).sqrt();
    let c = y / (1.0 + y.powi(2)).sqrt();
    let d = x / (1.0 + y.powi(2)).sqrt();

    let multiple = 1.0 / (2.0 * std::f64::consts::PI);
    let first = a * b.atan();
    let second = c * d.atan();

    return multiple * (first + second);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x() {
        let w = 3.0;
        let s = 7.5;
        let expected = 0.2;
        let result = x(w, s);
        assert!((expected - result).abs() < f64::EPSILON);
    }

    #[test]
    fn test_y() {
        let h = 1.5;
        let s = 7.5;
        let expected = 0.1;
        let result = y(h, s);
        assert!((expected - result).abs() < f64::EPSILON);
    }

    #[test]
    fn test_phi() {
        let x = 0.2;
        let y = 0.1;
        let expected = 0.024647431293237942;
        let result = phi(x, y);
        assert!((expected - result).abs() < f64::EPSILON);
    }
}
