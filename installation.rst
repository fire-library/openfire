Installation
============

From PyPI (Recommended)
-----------------------

Install the latest stable version from PyPI:

.. code-block:: bash

   pip install ofire

From Source
-----------

Prerequisites
~~~~~~~~~~~~~

- Python 3.8 or higher
- Rust toolchain (for development)

Development Installation
~~~~~~~~~~~~~~~~~~~~~~~~

1. Clone the repository:

.. code-block:: bash

   git clone https://github.com/fire-library/openfire.git
   cd openfire

2. Install in development mode:

.. code-block:: bash

   cd crates/python_api
   pip install -e .

Verify Installation
-------------------

Test that the installation works:

.. code-block:: python

   import ofire
   print("OpenFire installed successfully!")

Requirements
------------

- Python 3.8+
- Operating Systems: Linux, macOS, Windows