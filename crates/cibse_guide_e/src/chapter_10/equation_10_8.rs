pub mod openfire_runner;

pub fn fractional_effective_dose(m_f: f64, t: f64, lc_50: f64) -> f64 {
    return m_f * t / lc_50;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        let result = fractional_effective_dose(2.0, 120.0, 1000.0);
        assert_eq!(result, 0.24);
    }
}
