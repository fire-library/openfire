"""Sphinx configuration for OpenFire documentation."""

import sys
import os
import tomllib

# Add the python_api crate directory to Python path so sphinx can import ofire
sys.path.insert(0, os.path.abspath('..'))

# Read version from Cargo.toml
with open(os.path.join(os.path.dirname(__file__), '..', 'Cargo.toml'), 'rb') as f:
    cargo_toml = tomllib.load(f)
    release = cargo_toml['package']['version']

# Project information
project = 'OpenFire'
copyright = '2024, EmberonTech'
author = 'EmberonTech'

# Extensions
extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.viewcode',
    'sphinx.ext.napoleon',
    'sphinx.ext.intersphinx',
    'sphinx.ext.autosummary',
    'sphinx_autodoc_typehints',
]

# Templates path
templates_path = ['_templates']

# List of patterns to ignore when looking for source files
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store', 'stubs', '.venv']

# HTML theme
html_theme = 'furo'

# HTML theme options
html_theme_options = {
    "source_repository": "https://github.com/fire-library/openfire/",
    "source_branch": "main",
    "source_directory": "",
}

# HTML static path
html_static_path = ['_static']

# HTML context for global template variables
html_context = {
    'coverage_report_url': '_static/coverage/html/index.html'
}

# HTML title
html_title = f"{project} {release}"

# Autodoc settings
autodoc_default_options = { # pyright: ignore[reportUnknownVariableType]
    'members': True,
    'undoc-members': True,
    'show-inheritance': True,
    'special-members': '__init__',
    'imported-members': True,
    'exclude-members': '__weakref__',
}

# Additional autodoc settings
autodoc_member_order = 'bysource'
autodoc_typehints = 'description'
autodoc_typehints_description_target = 'documented'

# Napoleon settings for Google-style docstrings
napoleon_google_docstring = True
napoleon_numpy_docstring = False
napoleon_include_init_with_doc = False
napoleon_include_private_with_doc = False

# Intersphinx mapping
intersphinx_mapping = {
    'python': ('https://docs.python.org/3', None),
}

# Autosummary settings
autosummary_generate = True