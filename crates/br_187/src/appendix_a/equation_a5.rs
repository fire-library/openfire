pub fn x(w: f64, s: f64) -> f64 {
    w / s
}

pub fn y(h: f64, s: f64) -> f64 {
    h / s
}

pub fn phi(x: f64, y: f64, additive: bool) -> f64 {
    let a = 1.0 / (y.powi(2) + 1.0).sqrt();
    let b = x / (y.powi(2) + 1.0).sqrt();

    let multiple = 1.0 / (2.0 * std::f64::consts::PI);
    let first = x.atan();
    let second = a * b.atan();

    let total = multiple * (first - second);

    if additive { total } else { -total }
}

#[cfg(not(coverage))]
pub fn x_equation(x: &str, w: &str, s: &str) -> String {
    format!("{} = \\frac{{{}}}{{{}}}", x, w, s)
}
#[cfg(not(coverage))]
pub fn y_equation(y: &str, h: &str, s: &str) -> String {
    format!("{} = \\frac{{{}}}{{{}}}", y, h, s)
}
#[cfg(not(coverage))]
pub fn phi_equation(phi_symbol: &str, x: &str, y: &str) -> String {
    format!(
            "{} = \\frac{{1}}{{2\\pi}}\\left(\\tan^{{-1}}\\left({}\\right)-
           \\frac{{1}}{{\\sqrt{{{}^2 + 1}}}}\\tan^{{-1}}\\left(\\frac{{{}}}{{\\sqrt{{{}^2 + 1}}}}\\right)\\right)",
            phi_symbol, x, y, x, y
        )
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
        let result = phi(x, y, true);

        assert!((expected - result).abs() < f64::EPSILON);
    }
}
