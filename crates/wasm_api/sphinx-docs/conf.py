"""Sphinx configuration for OpenFire WASM documentation."""

import sys
import os

# Configuration file for the Sphinx documentation builder.

# -- Project information -----------------------------------------------------

project = 'OpenFire WASM API'
copyright = '2024, OpenFire Contributors'
author = 'OpenFire Contributors'
release = '0.1.0'

# -- General configuration ---------------------------------------------------

extensions = [
    'sphinx.ext.viewcode',
    'sphinx.ext.napoleon',
    'sphinx.ext.intersphinx',
    'sphinx.ext.autosummary',
]

# Templates path
templates_path = ['_templates']

# List of patterns to ignore when looking for source files
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

# -- sphinx-js configuration ------------------------------------------------
# Temporarily disabled due to TypeScript Symbol.dispose compatibility issues
# We'll use manual documentation instead

# Use TypeScript instead of JSDoc
# js_language = 'typescript'

# Path to the generated TypeScript declarations  
# js_source_path = '../pkg/wasm_api.d.ts'

# Set the primary domain to JavaScript (even for TypeScript)
# primary_domain = 'js'

# -- HTML theme configuration ------------------------------------------------

html_theme = 'furo'

html_theme_options = {
    "source_repository": "https://github.com/fire-library/openfire/",
    "source_branch": "main",
    "source_directory": "crates/wasm_api",
    "light_css_variables": {
        "color-brand-primary": "#e74c3c",
        "color-brand-content": "#e74c3c",
    },
    "dark_css_variables": {
        "color-brand-primary": "#e74c3c", 
        "color-brand-content": "#e74c3c",
    },
}

# HTML static path
html_static_path = ['_static']

# HTML title
html_title = f"{project} {release}"

# -- Napoleon settings for Google-style docstrings -------------------------

napoleon_google_docstring = True
napoleon_numpy_docstring = False
napoleon_include_init_with_doc = False
napoleon_include_private_with_doc = False

# -- Intersphinx mapping ----------------------------------------------------

intersphinx_mapping = {
    'python_api': ('https://fire-library.github.io/openfire/', None),
}

# -- Autosummary settings ---------------------------------------------------

autosummary_generate = True