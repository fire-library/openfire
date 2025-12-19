# OFire - Fire Safety Engineering Library

[![PyPI](https://img.shields.io/pypi/v/ofire)](https://pypi.org/project/ofire/)
[![Python Version](https://img.shields.io/pypi/pyversions/ofire)](https://pypi.org/project/ofire/)

A comprehensive Python package for fire safety engineering calculations and tools, built on industry standard fire engineering documents and implemented in Rust for performance and reliability.

## Overview

OFire provides Python bindings for a wide range of fire engineering calculations from authoritative sources including:

- **BR 187** - External Fire Spread (Building Research Establishment)
- **BS 9999** - Code of Practice for Fire Safety in the Design, Management and Use of Buildings
- **CIBSE Guide E** - Fire Safety Engineering
- **PD 7974** - Application of Fire Safety Engineering Principles to the Design of Buildings
- **SFPE Handbook** - Fire Protection Engineering
- **Introduction to Fire Dynamics** - Fundamental fire behavior principles
- **TR 17** - Fire Safety Engineering Technical Reports

All calculations are implemented in Rust and exposed through Python bindings using PyO3 for optimal performance and memory safety.

## Installation

Install from PyPI using pip:

```bash
pip install ofire
```

## Quick Start

```python
import ofire

# Calculate thermal radiation intensity using BR 187
radiation = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(0.15, 50000.0)
print(f"Received radiation: {radiation:.1f} W/m²")

# Calculate view factor for parallel surfaces using BR 187 
x = ofire.br_187.appendix_a.equation_a3.x(5.0, 10.0)
y = ofire.br_187.appendix_a.equation_a3.y(3.0, 10.0)
view_factor = ofire.br_187.appendix_a.equation_a3.phi(x, y, True)
print(f"View factor: {view_factor:.4f}")

# Calculate heat release rate at flashover using CIBSE Guide E
flashover_hrr = ofire.cibse_guide_e.chapter_6.equation_6_7.heat_release_rate_flashover(2.0, 2.1)
print(f"Flashover HRR: {flashover_hrr:.1f} kW")

# Calculate mean flame height using CIBSE Guide E
flame_height = ofire.cibse_guide_e.chapter_6.equation_6_55.mean_flame_height(1000.0)
print(f"Mean flame height: {flame_height:.2f} m")
```

## 📚 Documentation

### [**📖 Complete API Documentation & Examples →**](https://fire-library.github.io/openfire/)

Find the complete function reference, mathematical formulations, detailed examples, and all supported fire engineering standards and modules in the full documentation.

## Links

- **🏠 Homepage**: [https://www.openfiresoftware.com/](https://www.openfiresoftware.com/)
- **📖 Documentation**: [https://fire-library.github.io/openfire/](https://fire-library.github.io/openfire/)
- **💻 Source Code**: [https://github.com/fire-library/openfire](https://github.com/fire-library/openfire)

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](https://github.com/fire-library/openfire/blob/main/CONTRIBUTING.md) for information on how to get started.

## License

This project is licensed under the terms specified in the PYTHON_LICENSE file.
