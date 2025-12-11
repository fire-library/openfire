OpenFire WASM API Documentation
=================================

Fire safety engineering tools for WebAssembly and JavaScript applications.

Overview
--------

OpenFire provides high-performance fire safety engineering calculations through WebAssembly bindings. Built in Rust for maximum speed and memory safety, with JavaScript/TypeScript integration for web applications.

Key Features
------------

- **High Performance**: Implemented in Rust and compiled to WebAssembly for maximum speed
- **JavaScript/TypeScript Integration**: Easy-to-use JavaScript API with full TypeScript support  
- **Fire Dynamics**: Tools for fire dynamics calculations organized by document and chapter
- **Safety Engineering**: Comprehensive fire safety analysis tools

Quick Start
-----------

.. code-block:: javascript

   import init, { get_fire_dynamics_tools } from 'openfire-wasm';

   // Initialize the WASM module
   await init();

   // Get the Fire Dynamics Tools API
   const fdt = get_fire_dynamics_tools();

   // Chapter 2 - Compartment Fire Dynamics
   const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
   console.log(`Hot gas temperature increase: ${tempIncrease} K`);

Project Structure
-----------------

- **Rust Core**: High-performance fire engineering calculations
- **WebAssembly Bindings**: Efficient WASM interface  
- **TypeScript Declarations**: Full type safety for JavaScript/TypeScript applications
- **Documentation**: Comprehensive guides and API reference

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   installation
   guide/index
   api/index

Related Projects
----------------

- `OpenFire Python API <https://fire-library.github.io/openfire/>`_ - Python bindings for the same calculations
- `OpenFire Core <https://github.com/fire-library/openfire>`_ - The underlying Rust implementation

Contributing
------------

See our `GitHub repository <https://github.com/fire-library/openfire>`_ for contribution guidelines.

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`