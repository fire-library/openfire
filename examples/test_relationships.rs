use openfire::equation_relationships::EquationRelationships;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing equation relationships TOML file...");

    // Try to load the TOML file
    match EquationRelationships::from_file("equation_relationships.toml") {
        Ok(relationships) => {
            println!("✓ Successfully loaded equation_relationships.toml");

            let stats = relationships.get_statistics();
            println!("📊 Statistics:");
            println!(
                "  - Identical equation groups: {}",
                stats.total_identical_groups
            );
            println!(
                "  - Similar equation groups: {}",
                stats.total_similar_groups
            );
            println!(
                "  - Total equations in identical groups: {}",
                stats.total_equations_in_identical_groups
            );
            println!(
                "  - Total equations in similar groups: {}",
                stats.total_equations_in_similar_groups
            );

            println!("✓ File structure is valid!");
        }
        Err(e) => {
            println!("❌ Error loading equation_relationships.toml: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
