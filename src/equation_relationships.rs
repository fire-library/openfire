use serde::{Deserialize, Serialize};

/// Represents the metadata for the equation relationships file
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub version: String,
    pub description: String,
    pub created_date: String,
    pub last_updated: String,
}

/// Represents a group of identical equations
#[derive(Debug, Serialize, Deserialize)]
pub struct IdenticalEquationGroup {
    pub description: String,
    pub equations: Vec<String>,
}

/// Represents a group of similar equations with variations
#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarEquationGroup {
    pub description: String,
    pub variations_note: String,
    pub equations: Vec<String>,
}

/// Complete structure for equation relationships
#[derive(Debug, Serialize, Deserialize)]
pub struct EquationRelationships {
    pub metadata: Metadata,
    pub identical_equations: Vec<IdenticalEquationGroup>,
    pub similar_equations: Vec<SimilarEquationGroup>,
}

impl EquationRelationships {
    /// Load equation relationships from TOML file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let relationships: EquationRelationships = toml::from_str(&content)?;
        Ok(relationships)
    }

    /// Save equation relationships to TOML file
    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Find all equations that are identical to a given equation
    pub fn find_identical_equations(&self, equation_path: &str) -> Vec<String> {
        for group in &self.identical_equations {
            if group.equations.contains(&equation_path.to_string()) {
                return group
                    .equations
                    .iter()
                    .filter(|eq| *eq != equation_path)
                    .cloned()
                    .collect();
            }
        }
        Vec::new()
    }

    /// Find all equations that are similar to a given equation
    pub fn find_similar_equations(&self, equation_path: &str) -> Vec<(String, String)> {
        for group in &self.similar_equations {
            if group.equations.contains(&equation_path.to_string()) {
                return group
                    .equations
                    .iter()
                    .filter(|eq| *eq != equation_path)
                    .map(|eq| (eq.clone(), group.variations_note.clone()))
                    .collect();
            }
        }
        Vec::new()
    }

    /// Get statistics about equation relationships
    pub fn get_statistics(&self) -> EquationStatistics {
        let identical_count: usize = self
            .identical_equations
            .iter()
            .map(|group| group.equations.len())
            .sum();

        let similar_count: usize = self
            .similar_equations
            .iter()
            .map(|group| group.equations.len())
            .sum();

        EquationStatistics {
            total_identical_groups: self.identical_equations.len(),
            total_similar_groups: self.similar_equations.len(),
            total_equations_in_identical_groups: identical_count,
            total_equations_in_similar_groups: similar_count,
        }
    }

    /// Add a new group of identical equations
    pub fn add_identical_group(&mut self, description: String, equations: Vec<String>) {
        self.identical_equations.push(IdenticalEquationGroup {
            description,
            equations,
        });
    }

    /// Add a new group of similar equations
    pub fn add_similar_group(
        &mut self,
        description: String,
        variations_note: String,
        equations: Vec<String>,
    ) {
        self.similar_equations.push(SimilarEquationGroup {
            description,
            variations_note,
            equations,
        });
    }
}

/// Statistics about equation relationships
#[derive(Debug)]
pub struct EquationStatistics {
    pub total_identical_groups: usize,
    pub total_similar_groups: usize,
    pub total_equations_in_identical_groups: usize,
    pub total_equations_in_similar_groups: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_equation_relationships_roundtrip() {
        let mut relationships = EquationRelationships {
            metadata: Metadata {
                version: "1.0".to_string(),
                description: "Test relationships".to_string(),
                created_date: "2025-11-13".to_string(),
                last_updated: "2025-11-13".to_string(),
            },
            identical_equations: vec![],
            similar_equations: vec![],
        };

        relationships.add_identical_group(
            "Test identical equations".to_string(),
            vec![
                "br_187/chapter_1/equation_1".to_string(),
                "cibse_guide_e/chapter_6/equation_6_7".to_string(),
            ],
        );

        relationships.add_similar_group(
            "Test similar equations".to_string(),
            "Different constants".to_string(),
            vec![
                "fire_dynamics_tools/chapter_5/equation_5_1".to_string(),
                "pd_7974/part_1/section_8/equation_8_3".to_string(),
            ],
        );

        // Test serialization and deserialization
        let temp_file = NamedTempFile::new().unwrap();
        relationships
            .to_file(temp_file.path().to_str().unwrap())
            .unwrap();

        let loaded_relationships =
            EquationRelationships::from_file(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(loaded_relationships.identical_equations.len(), 1);
        assert_eq!(loaded_relationships.similar_equations.len(), 1);
    }

    #[test]
    fn test_find_identical_equations() {
        let mut relationships = EquationRelationships {
            metadata: Metadata {
                version: "1.0".to_string(),
                description: "Test".to_string(),
                created_date: "2025-11-13".to_string(),
                last_updated: "2025-11-13".to_string(),
            },
            identical_equations: vec![],
            similar_equations: vec![],
        };

        relationships.add_identical_group(
            "Heat release rate".to_string(),
            vec![
                "br_187/chapter_1/equation_1".to_string(),
                "cibse_guide_e/chapter_6/equation_6_7".to_string(),
                "pd_7974/part_1/section_3/equation_3_1".to_string(),
            ],
        );

        let identical = relationships.find_identical_equations("br_187/chapter_1/equation_1");
        assert_eq!(identical.len(), 2);
        assert!(identical.contains(&"cibse_guide_e/chapter_6/equation_6_7".to_string()));
        assert!(identical.contains(&"pd_7974/part_1/section_3/equation_3_1".to_string()));
    }
}
