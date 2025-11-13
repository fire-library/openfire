# Equation Relationship Tracking System

## Overview

This system tracks relationships between equations across different fire engineering documents in the OpenFire library. It helps identify which equations are identical or similar across different documents, enabling better organization and user guidance.

## Files

### Primary Files
- **`equation_relationships.toml`** - Main data file containing the lists of lists for tracking equation relationships
- **`src/equation_relationships.rs`** - Rust module for programmatic access to the relationship data

## Structure

### TOML File Format

The `equation_relationships.toml` file uses TOML format with the following structure:

```toml
[metadata]
version = "1.0"
description = "Tracks relationships between equations across fire engineering documents"
created_date = "2025-11-13"
last_updated = "2025-11-13"

# Identical Equations - Each list represents equations that are mathematically identical
[[identical_equations]]
description = "Description of what these equations calculate"
equations = [
    "document1/chapter/equation_name",
    "document2/chapter/equation_name",
]

# Similar Equations - Each list represents equations with minor variations
[[similar_equations]]
description = "Description of what these equations calculate"
variations_note = "Description of how they differ"
equations = [
    "document1/chapter/equation_name",
    "document2/chapter/equation_name",
]
```

### Equation Path Format

Equations are referenced using the path format: `crate_name/chapter_name/equation_file_name`

**Examples:**
- `br_187/chapter_1/equation_1`
- `cibse_guide_e/chapter_6/equation_6_7`
- `fire_dynamics_tools/chapter_5/equation_5_1`
- `pd_7974/part_1/section_8/equation_8_3`

**Note:** Do not include the `.rs` file extension in the path.

## Usage

### Manual Data Entry

#### Adding Identical Equations

When you find equations that are mathematically identical across documents:

1. Open `equation_relationships.toml`
2. Add a new `[[identical_equations]]` section
3. Provide a descriptive description
4. List all identical equations

Example:
```toml
[[identical_equations]]
description = "Heat release rate calculation using area and heat release rate per unit area"
equations = [
    "br_187/chapter_1/equation_1",
    "cibse_guide_e/chapter_6/equation_6_7",
    "pd_7974/part_1/section_3/equation_3_1",
]
```

#### Adding Similar Equations

When you find equations that are similar but with minor variations:

1. Open `equation_relationships.toml`
2. Add a new `[[similar_equations]]` section
3. Provide a descriptive description
4. Add a variations_note explaining the differences
5. List all similar equations

Example:
```toml
[[similar_equations]]
description = "Thermal radiation from point sources"
variations_note = "Different view factor calculations and geometric constants"
equations = [
    "fire_dynamics_tools/chapter_5/equation_5_1",
    "pd_7974/part_1/section_8/equation_8_3",
    "sfpe_handbook/chapter_14/equation_14_12",
]
```

### Programmatic Access

The Rust module provides several methods for working with the relationship data:

```rust
use openfire::equation_relationships::EquationRelationships;

// Load relationships from file
let relationships = EquationRelationships::from_file("equation_relationships.toml")?;

// Find identical equations
let identical = relationships.find_identical_equations("br_187/chapter_1/equation_1");

// Find similar equations  
let similar = relationships.find_similar_equations("fire_dynamics_tools/chapter_5/equation_5_1");

// Get statistics
let stats = relationships.get_statistics();
println!("Total identical groups: {}", stats.total_identical_groups);
println!("Total similar groups: {}", stats.total_similar_groups);

// Add new groups programmatically
let mut relationships = EquationRelationships::from_file("equation_relationships.toml")?;
relationships.add_identical_group(
    "New identical group".to_string(),
    vec!["doc1/ch1/eq1".to_string(), "doc2/ch2/eq2".to_string()]
);
relationships.to_file("equation_relationships.toml")?;
```

## Workflow for Adding Relationships

### Step-by-Step Process

1. **Identify Potential Relationships**
   - While working with equations, note when you see similar formulations
   - Compare equations across different documents and chapters
   - Look for identical mathematical expressions or very similar ones

2. **Determine Relationship Type**
   - **Identical**: Exactly the same mathematical expression and meaning
   - **Similar**: Same general approach but with variations (different constants, additional terms, etc.)

3. **Add to TOML File**
   - Open `equation_relationships.toml`
   - Choose the appropriate section (`identical_equations` or `similar_equations`)
   - Add a new list with descriptive information
   - Use the correct path format for equation references

4. **Document Variations** (for similar equations)
   - In the `variations_note` field, clearly describe how the equations differ
   - Include information about different constants, additional terms, or different applications

### Best Practices

- **Be Specific**: Use clear, descriptive names in the `description` field
- **Document Differences**: For similar equations, always explain the variations
- **Verify Accuracy**: Double-check that equations are truly identical or similar before adding
- **Update Metadata**: Remember to update the `last_updated` field in metadata when making changes
- **Group Logically**: Keep related equations in the same group rather than creating many small groups

## Future Enhancements

The system is designed to be extensible for future features:

- **User Interface**: The structured data can be used to build user interfaces showing equation relationships
- **Search Functionality**: Users could search for equations and see related ones
- **Documentation Generation**: Auto-generate documentation showing equation relationships
- **Validation Tools**: Build tools to verify that referenced equations actually exist
- **Cross-Reference Reports**: Generate reports showing the most commonly reused equations

## Examples

### Current Document Structure
```
crates/
├── br_187/src/chapter_1/equation_1.rs
├── cibse_guide_e/src/chapter_6/equation_6_7.rs  
├── fire_dynamics_tools/src/chapter_5/equation_5_1.rs
└── pd_7974/src/part_1/section_8/equation_8_3.rs
```

### Corresponding TOML Entries
```toml
[[identical_equations]]
description = "Heat release rate calculations"
equations = [
    "br_187/chapter_1/equation_1",
    "cibse_guide_e/chapter_6/equation_6_7",
]

[[similar_equations]]  
description = "Thermal radiation point source calculations"
variations_note = "Different geometric factors and view factor calculations"
equations = [
    "fire_dynamics_tools/chapter_5/equation_5_1", 
    "pd_7974/part_1/section_8/equation_8_3",
]
```

This system provides a solid foundation for tracking equation relationships while remaining simple enough for manual maintenance and flexible enough for future programmatic enhancements.