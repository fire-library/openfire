pub mod openfire_runner;

pub fn maximum_flowrate_persons(w: f64) -> f64 {
    return 1.333*w;
}

pub fn equation(f: String, w: String) -> String {
    format!(
        "{} = 1.333 \\cdot {}",
        f, w,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = required_width_stair(1.2);
        assert_eq!(result, 1.5996);
    }
}
