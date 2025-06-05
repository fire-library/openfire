pub mod openfire_runner;

pub fn required_width_stair(p: i32, n: i32) -> f64 {
    let p = p as f64;
    let n = n as f64;
    return (p + 15.0 * n - 15.0) / (150.0 + 50.0 * n);
}

pub fn equation(w: String, p: String, n: String) -> String {
    format!(
        "{} = \\frac{{{} + 15 \\cdot {} - 15}}{{150 + 50 \\cdot {}}}",
        w, p, n, n
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = required_width_stair(550, 6);
        assert_eq!(result, 1.3888888888888889);
    }
}
