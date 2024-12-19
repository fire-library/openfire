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

    return multiple * (first + second);
}
