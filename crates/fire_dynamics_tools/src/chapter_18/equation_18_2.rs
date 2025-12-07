pub fn concentration_particulates(
    m_p: f64,
    v: f64,
) -> f64 {
    m_p / v
}

pub fn concentration_particulates_equation(
    m_p: String,
    m_p_big: String, 
    v: String,
) -> String {
    format!(
        "{} = \\frac{{ {} }}{{ {} }}",
        m_p, m_p_big, v
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concentration_particulates() {
        let m_p = 0.059;
        let v = 90000.0;
        let expected = 0.000006555555556;

        let result = concentration_particulates(m_p, v);

        assert!(
            (result - expected).abs() < 1e-4,
            "Result should be approximately {}, but got {}",
            expected,
            result
        );
    }
}