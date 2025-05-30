pub mod openfire_runner;

pub fn stair_capacity(w: f64, n: i16) -> f64 {
    let result: f64 = 200.0 * w + 50.0 * (w - 0.3) * (n - 1) as f64;
    return result.floor();
}

pub fn equation(p: String, w: String, n: String) -> String {
    format!(
        "{} = 200 \\cdot {} + 50 \\cdot ({} - 0.3) \\cdot ({} - 1)",
        p, w, w, n
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = stair_capacity(1.2, 6);
        assert_eq!(result, 465.0);
    }
}
