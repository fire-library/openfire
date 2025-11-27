#!/bin/bash

# Build documentation for the OpenFire Python API
# This script checks for a .venv and builds the docs using the correct Python environment

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
VENV_PATH="$PROJECT_ROOT/.venv"
PYTHON_API_DIR="$PROJECT_ROOT/crates/python_api"
DOCS_DIR="$PYTHON_API_DIR/docs"
BUILD_DIR="$PYTHON_API_DIR/_build"

echo -e "${GREEN}OpenFire Documentation Builder${NC}"
echo "================================"

# Check if .venv exists
if [ ! -d "$VENV_PATH" ]; then
    echo -e "${RED}Error: Virtual environment not found at $VENV_PATH${NC}"
    echo ""
    echo "Please create a virtual environment first:"
    echo -e "${YELLOW}  python -m venv .venv${NC}"
    echo -e "${YELLOW}  source .venv/bin/activate${NC}"
    echo -e "${YELLOW}  pip install -r crates/python_api/docs/docs-requirements.txt${NC}"
    echo ""
    exit 1
fi

# Check if activation script exists
if [ ! -f "$VENV_PATH/bin/activate" ]; then
    echo -e "${RED}Error: Virtual environment activation script not found${NC}"
    echo "Expected: $VENV_PATH/bin/activate"
    exit 1
fi

echo "✓ Found virtual environment at $VENV_PATH"

# Activate the virtual environment
echo "✓ Activating virtual environment..."
source "$VENV_PATH/bin/activate"

# Verify we're using the right Python
PYTHON_VERSION=$(python --version 2>&1)
PYTHON_PATH=$(which python)
echo "✓ Using Python: $PYTHON_VERSION ($PYTHON_PATH)"

# Check if we're in the venv
if [[ "$PYTHON_PATH" != *".venv"* ]]; then
    echo -e "${YELLOW}Warning: Python path doesn't contain .venv - make sure virtual environment is activated${NC}"
fi

# Navigate to python_api directory
# cd "$PYTHON_API_DIR"

# Install/update docs dependencies
echo "✓ Installing documentation dependencies..."
pip install -r crates/python_api/docs/docs-requirements.txt > /dev/null

# Install ofire package for API docs
echo "✓ Installing ofire package..."
pip install maturin > /dev/null 2>&1
maturin develop --manifest-path crates/python_api/Cargo.toml --quiet

# Verify ofire installation
echo "✓ Verifying ofire installation..."
python -c "import ofire; print('ofire modules:', len([x for x in dir(ofire) if not x.startswith('_')]))" 

# Clean previous build
if [ -d "$BUILD_DIR" ]; then
    echo "✓ Cleaning previous build..."
    rm -rf "$BUILD_DIR"
fi

# Build documentation
echo "✓ Building documentation with Sphinx..."
python -m sphinx -b html crates/python_api/docs _build

# Check if build was successful
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Documentation built successfully!${NC}"
    echo ""
    echo "Output directory: $BUILD_DIR"
    echo "Open in browser: file://$BUILD_DIR/index.html"
    
    # Optionally open in browser (uncomment if desired)
    # open "$BUILD_DIR/index.html"  # macOS
    # xdg-open "$BUILD_DIR/index.html"  # Linux
else
    echo -e "${RED}✗ Documentation build failed${NC}"
    exit 1
fi