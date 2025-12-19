pub fn maximum_people_in_stair(p: f64, a: f64, s: i32) -> i32 {
    let s = s as f64;
    let result = p * a * s;
    result.floor() as i32
}

pub fn equation(n_c: String, p: String, a: String, s: String) -> String {
    format!("{} = {} \\cdot {} \\cdot {}", n_c, p, a, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = maximum_people_in_stair(3.5, 8.0, 6);
        assert_eq!(result, 168);
    }
}
