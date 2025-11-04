# Python API Docstring Guide for Fire Engineering Functions

This guide documents the standardized format for writing Python docstrings in the `crates/python_api/` module of this fire engineering library.

## Project Structure

- **Rust crates**: Located in `crates/` directory, each containing fire engineering equations
- **Python API**: Located in `crates/python_api/src/`, provides Python bindings using PyO3/maturin
- **Equation functions**: Each Rust crate has equation functions that should be referenced for LaTeX

## Docstring Format

### Basic Structure
```rust
#[pyfunction]
/// Brief description of what the function calculates (Equation X).
///
/// Longer description explaining the context and use case.
///
/// .. math::
///
///    LATEX_EQUATION_HERE
///
/// where:
///
/// - :math:`SYMBOL` is the description (units)
/// - :math:`SYMBOL` is the description (units)
///
/// Args:
///     param_name (type): Description (units)
///     param_name (type): Description (units)
///
/// Returns:
///     type: Description (units)
///
/// Example:
///     >>> import ofire
///     >>> result = ofire.module.function(args)
fn function_name(params) -> PyResult<type> {
```

### LaTeX Math Requirements

1. **Always include LaTeX equations** using `.. math::` directive
2. **Reference actual Rust functions** - check the corresponding Rust crate for equation functions
3. **Verify equation accuracy** - compare LaTeX with the actual Rust implementation
4. **Use proper notation** - follow the standard symbols used in the engineering documents

#### Example LaTeX Formats:
```latex
I_s = \sigma \cdot \varepsilon \cdot T^4                    # Simple equation
O = \frac{A_s}{A \cdot \sqrt{H}}                           # Fraction with square root
X = \frac{W}{S}                                            # Simple fraction
\phi = \frac{1}{2\pi}\left(\tan^{-1}(X) - ...\right)      # Complex equation
```

### Variable Definitions

1. **Always include `where:` section** after math equation
2. **Add empty line** after `where:` for proper rendering
3. **Use `:math:` role** for mathematical symbols
4. **Include units** in parentheses for physical quantities
5. **Specify dimensionless** for unitless quantities

#### Format:
```rust
/// where:
///
/// - :math:`SYMBOL` is the description (units)
/// - :math:`SYMBOL` is the description (dimensionless)
```

### Parameter Documentation

1. **Include type annotations**: `param_name (type)`
2. **Standard types**: `float`, `bool`, `int`, `str`
3. **Include units** in description
4. **Be consistent** with variable names from LaTeX section

#### Examples:
```rust
/// Args:
///     sigma (float): Stefan-Boltzmann constant (kW/m²K⁴)
///     emissivity (float): Surface emissivity (dimensionless, 0-1)
///     temperature (float): Absolute temperature (K)
///     additive (bool): Whether this view factor is positive or negative
```

### Return Value Documentation

1. **Always specify return type**: `type: Description (units)`
2. **Include units** for physical quantities

#### Examples:
```rust
/// Returns:
///     float: Radiation intensity (kW/m²)
///     float: Ventilation factor (m⁻¹/²)
///     float: View factor (dimensionless)
```

## Common Patterns

### Equation Verification Process
1. **Read the Rust source** in the corresponding crate (e.g., `crates/br_187/src/`)
2. **Find equation functions** (usually named `*_equation()`)
3. **Check actual implementation** to verify formula
4. **Copy LaTeX from equation function** or derive from implementation
5. **Test for accuracy** - ensure LaTeX matches the code

### Module-Level Docstrings
Keep module docstrings concise since equations appear in navigation:

```rust
#[pymodule]
/// Module Name - Brief description.
/// 
/// Explanation of what this module contains and its purpose
/// in the fire engineering context.
/// 
/// These calculations are essential for [specific use case].
pub fn module_name(m: &Bound<'_, PyModule>) -> PyResult<()> {
```

## File Locations

- **Python bindings**: `crates/python_api/src/`
- **Rust source equations**: `crates/{crate_name}/src/`
- **Documentation**: Generated from these docstrings

## Important Notes

1. **Don't duplicate equation lists** - they appear in docs navigation
2. **Verify LaTeX accuracy** - always check against Rust implementation
3. **Use proper symbols** - follow engineering standard notation
4. **Include proper spacing** - empty lines matter for reStructuredText rendering
5. **Test compilation** - run `maturin develop` to verify syntax

## Example Files

See the following for reference implementations:
- `crates/python_api/src/br_187/appendix_a.rs`
- `crates/python_api/src/br_187/chapter_1.rs`

These files demonstrate the complete docstring format with LaTeX equations, proper type annotations, and variable definitions.