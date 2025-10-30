# Installation

## From PyPI (Recommended)

Install the latest stable version from PyPI:

```bash
pip install ofire
```

## From Source

### Prerequisites

- Python 3.8 or higher
- Rust toolchain (for development)

### Development Installation

1. Clone the repository:
```bash
git clone https://github.com/fire-library/openfire.git
cd openfire
```

2. Install in development mode:
```bash
cd crates/python_api
pip install -e .
```

## Verify Installation

Test that the installation works:

```python
import ofire
print("OpenFire installed successfully!")
```

## Requirements

- Python 3.8+
- Operating Systems: Linux, macOS, Windows