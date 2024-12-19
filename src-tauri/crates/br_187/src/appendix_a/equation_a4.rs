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
