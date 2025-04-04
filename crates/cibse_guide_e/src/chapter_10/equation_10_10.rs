pub mod openfire_runner;

pub fn limiting_velocity(g: f64, h: f64, t_f: f64, t_0: f64) -> f64 {
    return 0.64 * (g * h  * (t_f - t_0)/t_f).powf(0.5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = limiting_velocity(9.8, 2.2, 973.0, 293.0);
        assert_eq!(result, 2.52310759628797);
    }

}
