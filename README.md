# OpenFire

[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://openfire.readthedocs.io/)
[![PyPI](https://img.shields.io/pypi/v/ofire.svg)](https://pypi.org/project/ofire/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Fire safety engineering tools implemented in Rust with Python bindings.

## Overview

OpenFire provides a comprehensive toolkit of fundamental fire engineering building blocks rather than prescriptive, all-in-one solutions. This modular approach empowers engineers to construct solutions that precisely match their project requirements without being constrained by assumptions embedded in higher-level abstractions.

The library implements individual equations and calculation methods from established standards like BR 187, PD 7974, and others, allowing you to maintain complete control over your calculation workflows while ensuring accuracy and reliability.

## Key Features

- **High Performance**: Implemented in Rust for maximum speed and memory safety
- **Python Integration**: Easy-to-use Python API for rapid development
- **Modular Design**: Building blocks that can be composed for specific needs
- **Standards-Based**: Implements equations from established fire engineering standards
- **Fully Tested**: Reliable calculations you can trust in critical applications
- **Flexible**: No opinionated workflows - use the equations you need

## Installation Requirements

Before installing OpenFire, you'll need:

- **Python**: [Download and install Python](https://www.python.org/downloads/) (version 3.8 or higher)
- **Rust**: [Install Rust](https://rustup.rs/)
  - **Windows users**: Additional Visual Studio Build Tools are required. See the [Windows installation guide](https://rust-lang.github.io/rustup/installation/windows-msvc.html) for complete setup instructions.

## Quick Start

### Python Installation

```bash
pip install ofire
```

### Basic Example

```python
import ofire

# Calculate thermal radiation from a radiating surface
# Step 1: Source radiation intensity
I_s = ofire.br_187.appendix_a.equation_a1.radiation_intensity(
    sigma=5.67e-11,  # Stefan-Boltzmann constant (kW/m²K⁴)
    emissivity=0.9,  # Surface emissivity
    temperature=1273 # Surface temperature (K)
)

# Step 2: Dimensionless parameters
X = ofire.br_187.appendix_a.equation_a3.x(w=4.0, s=12.0)
Y = ofire.br_187.appendix_a.equation_a3.y(h=2.5, s=12.0)

# Step 3: View factor and received intensity
phi = ofire.br_187.appendix_a.equation_a3.phi(X, Y, additive=True)
I_R = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(phi, I_s)

print(f"Received intensity: {I_R:.1f} kW/m²")
```

## Available Standards

- **BR 187**: External fire spread calculations
- **PD 7974**: Application of fire safety engineering principles
- **CIBSE Guide E**: Fire engineering design guide
- **SFPE Handbook**: Fire protection engineering calculations
- *More standards coming soon...*

## Documentation

- **[Getting Started Guide](https://openfire.readthedocs.io/guide/getting-started/)**: Learn the library philosophy and basic usage
- **[Examples](https://openfire.readthedocs.io/guide/examples/)**: Practical workflows and use cases
- **[API Reference](https://openfire.readthedocs.io/api/)**: Complete function documentation
- **[Installation](https://openfire.readthedocs.io/installation/)**: Detailed installation instructions

## Library Philosophy

OpenFire's design reflects the complexity and diversity of fire safety engineering applications. Rather than providing monolithic functions that attempt to solve entire problems, we offer carefully tested, reliable components that can be composed for your specific engineering challenge.

The fire safety engineering field encompasses scenarios from simple stair width calculations to complex zone models, each with distinct regulatory frameworks and performance objectives. Even within specific categories like zone modeling, multiple established equations exist, and we don't force any particular approach on you.

This modular approach ensures you maintain complete control over your calculation workflows, can easily audit each step, and can adapt as standards evolve or requirements change.

## Project Structure

The project is organized as a Cargo workspace with the following structure:

```
openfire/
├── Cargo.toml (workspace root)
├── crates/
│   ├── br_187/                # Crate for BR 187 document
│   ├── bs9999/                # Crate for BS 9999 document
│   ├── cibse_guide_e/         # Crate for CIBSE Guide E document
│   ├── framework/             # Core framework crate
│   ├── introduction_to_fire_dynamics/ # Crate for Introduction to Fire Dynamics
│   ├── openfire_cli/          # Command-line interface for OpenFire
│   ├── pd_7974/               # Crate for PD 7974 document
│   ├── python_api/            # Python bindings using PyO3
│   ├── sfpe_handbook/         # Crate for SFPE Handbook
│   └── tr17/                  # Crate for TR 17 document
└── src/                       # Main library source that only exposes the crates
```

### Domain-Specific Crates

Each crate in the `crates/` directory corresponds to a specific document or domain in fire engineering.

## Development

### Rust Development

```bash
# Clone the repository
git clone https://github.com/fire-library/openfire
cd openfire

# Run tests
cargo test

# Build the library
cargo build --release
```

### Using the Library from Source

To use the library from source in Python:

```bash
# Create a Python virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install maturin
pip install maturin

# Build and install the Python package from source
maturin develop --manifest-path crates/python_api/Cargo.toml

# Now you can import and use the library in Python
python -c "import ofire; print('OpenFire loaded successfully')"
```

### Generating Python Documentation

To generate the Python API documentation:

```bash
# Create a Python virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install documentation dependencies
pip install -r crates/python_api/docs/docs-requirements.txt

# Install maturin and build the Python package
pip install maturin
maturin develop --manifest-path crates/python_api/Cargo.toml

# Generate the documentation
sphinx-build -b html ./crates/python_api/docs _build
```

The generated documentation will be available in the `_build` directory. Open `_build/index.html` in your browser to view the documentation.

## Contributing

We welcome contributions! Please see our [contributing guidelines](CONTRIBUTING.md) for details on:

- Code style and standards
- Testing requirements
- Documentation guidelines
- Pull request process

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [PyO3](https://pyo3.rs/) for Rust-Python interoperability
- Documentation powered by [Sphinx](https://www.sphinx-doc.org/)
- Inspired by the need for reliable, composable fire engineering tools
