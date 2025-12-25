pub fn mass_particulates_produced(m_f: f64, y_p: f64) -> f64 {
    y_p * m_f
}

#[cfg(not(coverage))]
pub fn mass_particulates_produced_equation(m_p_big: String, m_f: String, y_p: String) -> String {
    format!("{} = {} \\cdot {} ", m_p_big, y_p, m_f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_particulates_produced() {
        let m_f = 2.0;
        let y_p = 0.015;
        let expected = 0.03;

        let result = mass_particulates_produced(m_f, y_p);

        assert!((result - expected).abs() < 1e-4,);
    }
}
