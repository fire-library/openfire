pub mod openfire_runner;

pub fn exit_capacity_stair(w_s: f64, t: f64, a: f64, s: i32) -> i32 {
    let s = s as f64;
    let result = 1.333*w_s * t + 3.5 * a * (s - 1.0);
    return result.floor() as i32;
}

pub fn equation(n_in: String, w_s: String, t: String, a: String, s: String) -> String {
    format!("{} = 1.333 \\cdot {} \\cdot {} + 3.5 \\cdot {} \\cdot ({} - 1)", n_in, w_s, t, a, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = exit_capacity_stair(1.2, 150.0, 5.0, 10);
        assert_eq!(result, 379);
    }
}
