# Contributing to OpenFire

Thank you for your interest in contributing to OpenFire! This guide will help you get started with contributing to our fire safety engineering toolkit.

## Getting Started

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Python**: Version 3.8 or higher
- **Git**: For version control

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/openfire.git
   cd openfire
   ```

2. **Set up Rust development**
   ```bash
   # Install development dependencies
   cargo install cargo-nextest  # Fast test runner
   cargo install cargo-watch    # File watching for development
   
   # Run tests to ensure everything works
   cargo test
   ```

3. **Set up Python development**
   ```bash
   # Create virtual environment
   python -m venv .venv
   source .venv/bin/activate  # On Windows: .venv\Scripts\activate
   
   # Install Python development dependencies
   pip install maturin pytest pytest-cov black isort mypy
   
   # Build and install Python package
   maturin develop
   ```

## Code Standards

### Rust Code Style

- **Follow Rust conventions**: Use `cargo fmt` to format code
- **Use clippy**: Run `cargo clippy` to catch common mistakes
- **Document public APIs**: All public functions must have comprehensive documentation
- **Write tests**: Include unit tests for all new functionality

#### Example Function Documentation

```rust
/// Calculate dimensionless width parameter (Equation A3).
///
/// .. math::
///
///    X = \frac{W}{2 \cdot S}
///
/// where:
///
/// - :math:`X` is the dimensionless width parameter
/// - :math:`W` is the width of radiation source (m)
/// - :math:`S` is the distance from source to receiver (m)
///
/// # Arguments
///
/// * `w` - Width of radiation source (m)
/// * `s` - Distance from source to receiver (m)
///
/// # Returns
///
/// Dimensionless width parameter
///
/// # Examples
///
/// ```
/// use openfire::br_187::appendix_a::equation_a3;
/// let x = equation_a3::x(4.0, 12.0);
/// assert_eq!(x, 0.16666666666666666);
/// ```
pub fn x(w: f64, s: f64) -> f64 {
    w / (2.0 * s)
}
```

### Python Bindings Style

- **Follow PyO3 conventions**: Use appropriate PyO3 attributes
- **Include comprehensive docstrings**: Use reStructuredText format with LaTeX math
- **Reference Rust implementation**: Always verify LaTeX matches Rust code
- **Provide examples**: Include usage examples in docstrings

#### Example Python Function Documentation

```rust
#[pyfunction]
/// Calculate dimensionless width parameter (Equation A3).
///
/// .. math::
///
///    X = \frac{W}{2 \cdot S}
///
/// where:
///
/// - :math:`X` is the dimensionless width parameter
/// - :math:`W` is the width of radiation source (m)
/// - :math:`S` is the distance from source to receiver (m)
///
/// Args:
///     w (float): Width of radiation source (m)
///     s (float): Distance from source to receiver (m)
///
/// Returns:
///     float: Dimensionless width parameter
///
/// Example:
///     >>> import ofire
///     >>> X = ofire.br_187.appendix_a.equation_a3.x(4.0, 12.0)
///     >>> X
///     0.16666666666666666
fn x_a3(w: f64, s: f64) -> PyResult<f64> {
    Ok(rust_equation_a3::x(w, s))
}
```

## Testing Requirements

### Rust Tests

- **Unit tests**: Test individual functions with known values
- **Integration tests**: Test workflows and interactions between functions
- **Property-based tests**: Use `proptest` for mathematical properties
- **Documentation tests**: Ensure examples in docs compile and run

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### Python Tests

- **Function tests**: Verify Python bindings match Rust behavior
- **Workflow tests**: Test complete calculation workflows
- **Documentation tests**: Ensure examples work correctly

```bash
# Run Python tests
python -m pytest

# Run with coverage
python -m pytest --cov=ofire --cov-report=html
```

### Test Data Sources

When adding new equations:
- **Reference original standards**: Always cite the source document
- **Use published examples**: Prefer test cases from standards or textbooks
- **Document assumptions**: Clearly state any simplifications or limitations

## Documentation Guidelines

### Adding New Standards

1. **Create new crate**: Follow existing naming conventions (e.g., `bs_9999`)
2. **Organize by document structure**: Mirror the standard's organization
3. **Include equation references**: Always reference the source equation number
4. **Provide context**: Explain when and how to use each calculation

### Documentation Structure

```
docs/
├── api/
│   └── new-standard.rst     # API reference for new standard
├── guide/
│   └── new-examples.rst     # Usage examples
└── installation.rst         # Update if new dependencies needed
```

### Writing Examples

- **Use realistic values**: Choose parameters from actual engineering scenarios
- **Show complete workflows**: Demonstrate how functions work together
- **Include units**: Always specify units in comments and documentation
- **Explain context**: Describe the engineering scenario being modeled

## Submission Process

### Pull Request Guidelines

1. **Create feature branch**: Use descriptive names like `add-bs9999-ventilation`
2. **Write clear commit messages**: Follow conventional commit format
3. **Update documentation**: Include docs for new functionality
4. **Add tests**: Ensure good test coverage
5. **Run all checks**: Format, lint, and test before submitting

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

Types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`

Examples:
- `feat(br187): add equation A5 for perpendicular surfaces`
- `docs(examples): add thermal radiation workflow`
- `fix(python): correct parameter names in equation A3`

### Pre-submission Checklist

- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Examples work correctly
- [ ] Commit messages are clear
- [ ] No unnecessary dependencies added

## Adding New Standards

### Process

1. **Research**: Study the standard and identify key equations
2. **Plan structure**: Design the crate organization
3. **Implement core**: Start with fundamental equations
4. **Add tests**: Verify against known examples
5. **Create bindings**: Add Python API
6. **Document**: Write comprehensive documentation
7. **Examples**: Create practical usage examples

### File Organization

```
crates/new_standard/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Main module exports
│   ├── chapter_1/          # Organized by document structure
│   │   ├── mod.rs
│   │   ├── equation_1.rs
│   │   └── equation_2.rs
│   └── appendix_a/
│       ├── mod.rs
│       └── equation_a1.rs
└── tests/
    └── integration_tests.rs
```

## Code Review Process

### What We Look For

- **Correctness**: Mathematics matches the source standard
- **Performance**: Efficient implementation without premature optimization
- **Documentation**: Clear, comprehensive documentation
- **Testing**: Adequate test coverage with realistic scenarios
- **API design**: Consistent with existing patterns

### Review Timeline

- **Initial review**: Within 48 hours
- **Detailed feedback**: Within 1 week
- **Final approval**: After all feedback addressed

## Community Guidelines

- **Be respectful**: Treat all contributors with respect
- **Be constructive**: Provide helpful feedback and suggestions
- **Be patient**: Remember that everyone is learning
- **Ask questions**: Don't hesitate to ask for clarification

## Getting Help

- **GitHub Issues**: For bugs, feature requests, and questions
- **Discussions**: For general questions and community discussion
- **Documentation**: Check existing docs first
- **Code examples**: Look at existing implementations for patterns

## Recognition

Contributors will be acknowledged in:
- Release notes for significant contributions
- README acknowledgments section
- Git commit history

Thank you for contributing to OpenFire! Your contributions help make fire safety engineering more accessible and reliable for everyone.