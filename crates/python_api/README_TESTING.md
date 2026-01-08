# CLI Testing Guide

This document explains how to run and develop tests for the OpenFire CLI tool.

## Test Structure

```
crates/python_api/
├── tests/
│   ├── __init__.py
│   ├── conftest.py          # Test fixtures and configuration
│   └── test_cli.py          # CLI test cases
└── pyproject.toml           # Test dependencies and pytest config
```

## Running Tests Locally

### Prerequisites

1. **Install the package in development mode:**
   ```bash
   cd crates/python_api
   maturin develop --release
   ```

2. **Install test dependencies:**
   ```bash
   pip install -e ".[test]"
   ```

### Running Tests

**Run all tests:**
```bash
cd crates/python_api
pytest tests/ -v
```

**Run specific test classes:**
```bash
pytest tests/test_cli.py::TestVersionCommand -v
pytest tests/test_cli.py::TestNewCommand -v
```

**Run tests in parallel:**
```bash
pytest tests/ -n auto -v
```

**Run tests with coverage:**
```bash
pytest tests/ --cov=ofire --cov-report=html
```

## Test Categories

### 1. CLI Availability Tests
- Verify `ofire` command is accessible
- Test module importability
- Basic command execution

### 2. Command-Specific Tests
- **Version Command:** Output format and content
- **Help Command:** Subcommand listing and help text
- **New Command:** Project creation, file structure, various names
- **Run Command:** Basic functionality and error handling
- **Docs Command:** Execution without errors

### 3. Cross-Platform Tests
- Path handling (Windows vs Unix)
- File permissions (Unix-specific)
- Platform-specific behaviors

### 4. Error Handling Tests
- Invalid commands
- Missing arguments
- Graceful failure scenarios

## CI/CD Testing

The GitHub Actions workflow tests across:
- **Operating Systems:** Ubuntu, Windows, macOS
- **Python Versions:** 3.8, 3.9, 3.10, 3.11, 3.12
- **Test Modes:** Sequential and parallel execution

### Matrix Strategy
- Full Python version matrix on Ubuntu
- Reduced matrix on Windows/macOS for efficiency
- Separate Rust and Python test jobs

## Writing New Tests

### Test Fixtures Available

**`cli_runner`** - Execute CLI commands:
```python
def test_my_command(cli_runner):
    result = cli_runner("new", "my_project", "-d", "/tmp")
    assert result.returncode == 0
```

**`temp_dir`** - Temporary directory:
```python
def test_file_creation(temp_dir):
    test_file = temp_dir / "test.txt"
    test_file.write_text("content")
    assert test_file.exists()
```

**`ofire_available`** - Skip if CLI not installed:
```python
def test_cli_function(ofire_available, cli_runner):
    # Test only runs if ofire is properly installed
    result = cli_runner("version")
    assert result.returncode == 0
```

### Test Naming Conventions
- Test files: `test_*.py`
- Test classes: `Test*`
- Test functions: `test_*`

### Platform-Specific Tests
```python
@pytest.mark.skipif(os.name == 'nt', reason="Unix-specific test")
def test_unix_feature():
    # Unix-only test code

@pytest.mark.skipif(os.name != 'nt', reason="Windows-specific test")
def test_windows_feature():
    # Windows-only test code
```

## Test Configuration

The `pyproject.toml` includes pytest configuration:
- Test discovery patterns
- Verbose output by default
- Custom markers for slow/integration tests
- Short traceback format

## Debugging Tests

**Run with detailed output:**
```bash
pytest tests/ -v --tb=long
```

**Run specific test with debugging:**
```bash
pytest tests/test_cli.py::test_version_command -v -s
```

**Stop on first failure:**
```bash
pytest tests/ -x
```

## Common Issues

1. **"ofire CLI not available"** - Run `maturin develop` first
2. **Import errors** - Ensure you're in the `python_api` directory
3. **Permission errors** - Check temp directory permissions
4. **Path issues** - Use `pathlib.Path` for cross-platform compatibility

## Contributing

When adding new CLI functionality:
1. Add corresponding tests in `test_cli.py`
2. Update this README if new test patterns are introduced
3. Ensure tests pass on all platforms via CI/CD
4. Consider edge cases and error conditions