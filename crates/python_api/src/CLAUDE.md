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

## Exposing Documentation in Sphinx

After creating or updating docstrings, you must also update the Sphinx documentation configuration to expose the new modules and functions.

### Module Structure Requirements

1. **Use `#[pymodule]` decorator**: All module functions must have the `#[pymodule]` decorator
2. **Use `wrap_pymodule!` pattern**: Main modules should use `wrap_pymodule!` to expose submodules
3. **Avoid naming conflicts**: Use descriptive suffixes based on the source document if module names conflict

#### Naming Convention for Conflicts:
When multiple documents have the same chapter numbers (e.g., Chapter 6), use a suffix that reflects the source document:
- `chapter_6_intro` for "Introduction to Fire Dynamics" 
- `chapter_6_br187` for "BR 187"
- `chapter_6_pd7974` for "PD 7974"

**Do not use generic suffixes like `_intro` without context** - the suffix should clearly indicate the source document.

#### Example Module Structure:
```rust
// In main module file (e.g., introduction_to_fire_dynamics.rs)
#[pymodule]
/// Module Name - Brief description.
pub fn module_name(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(chapter_6::chapter_6_intro))?;
    m.add_wrapped(wrap_pymodule!(chapter_10::chapter_10_intro))?;
    Ok(())
}

// In chapter file (e.g., chapter_6.rs)
#[pymodule]
/// Chapter Name - Brief description.
pub fn chapter_6_intro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_6_32))?;
    Ok(())
}
```

### Updating Sphinx Documentation

1. **Navigate to docs/api/** directory
2. **Find or create** the `.rst` file for your module (e.g., `introduction-to-fire-dynamics.rst`)
3. **Add automodule directives** following the hierarchy pattern:

#### Example Documentation Structure:
```rst
Module Name
===========

Brief description of the module.

.. automodule:: ofire.module_name
   :members:
   :undoc-members:
   :show-inheritance:

Chapter Name
------------

.. automodule:: ofire.module_name.chapter_name_suffix
   :members:
   :undoc-members:
   :show-inheritance:

Equation Modules
~~~~~~~~~~~~~~~~

Equation X.Y - Brief Description
"""""""""""""""""""""""""""""""""

.. automodule:: ofire.module_name.chapter_name_suffix.equation_x_y
   :members:
   :undoc-members:
   :show-inheritance:
```

### Building and Testing

1. **Build the Python module**: Run `maturin develop` to compile changes
2. **Test imports**: Verify modules can be imported in Python
3. **Build documentation**: Run Sphinx to generate documentation
4. **Check for warnings**: Fix any import or module errors

### Common Issues

1. **ModuleNotFoundError**: Check that module names in `.rst` files match actual Python module names
2. **Missing `#[pymodule]`**: Ensure all module functions have the decorator
3. **Naming conflicts**: Use descriptive suffixes based on source document (not generic ones)
4. **Import path mismatches**: Verify the full import path matches the module hierarchy

## Example Files

See the following for reference implementations:
- `crates/python_api/src/br_187/appendix_a.rs`
- `crates/python_api/src/br_187/chapter_1.rs`
- `crates/python_api/src/introduction_to_fire_dynamics/chapter_6.rs` (uses `_intro` suffix for "Introduction" document)
- `crates/python_api/src/pd_7974/part_1/section_8.rs`

And their corresponding documentation files:
- `docs/api/br-187.rst`
- `docs/api/introduction-to-fire-dynamics.rst`
- `docs/api/pd-7974.rst`

These files demonstrate the complete docstring format with LaTeX equations, proper type annotations, variable definitions, and proper Sphinx integration.