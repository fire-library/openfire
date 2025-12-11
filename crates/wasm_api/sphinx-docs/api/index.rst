API Reference
=============

Complete API reference for all OpenFire WASM modules, organized by source document.

Overview
--------

The OpenFire WASM API provides fire safety engineering calculations through a WebAssembly interface. Functions are organized hierarchically by source document and chapter to match fire engineering standards and guides.

Module Organization
-------------------

Available Modules
~~~~~~~~~~~~~~~~~

Fire Dynamics Tools
^^^^^^^^^^^^^^^^^^^^

General fire dynamics calculations organized by chapter:

- **Chapter 2**: Compartment Fire Dynamics
- **Chapter 4**: Heat Transfer  
- **Chapter 5**: Smoke Movement

.. toctree::
   :maxdepth: 2

   fire-dynamics-tools

Coming Soon
~~~~~~~~~~~

The following modules are planned for future releases:

PD 7974 - Fire Safety Engineering Principles
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- Part 1: Application of fire safety engineering principles
- Additional parts as implemented

BR 187 - External Fire Spread
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- Appendix A: External fire spread calculations
- Chapter 1: Building separation requirements

BS 9999 - Fire Safety in Buildings
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- Chapter 15: Smoke control systems

CIBSE Guide E - Fire Safety Engineering
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- Chapter 6: Compartment fires
- Chapter 7: Structural fire protection
- Chapter 10: Fire dynamics

SFPE Handbook
^^^^^^^^^^^^^
- Chapter 14: Heat transfer calculations

TR 17 - Technical Report
^^^^^^^^^^^^^^^^^^^^^^^^^
- Section 2: Fire safety calculations

Core Module Functions
----------------------

Main API Access
~~~~~~~~~~~~~~~

**get_fire_dynamics_tools()**
  Returns an instance that provides access to organized fire dynamics calculations by document chapter.

Utility Functions
~~~~~~~~~~~~~~~~~

Basic Calculations
^^^^^^^^^^^^^^^^^^

**calculate_circle_area(radius)**
  Calculate the area of a circle. This is a minimal example function to demonstrate WASM documentation generation.
  
  - ``radius`` (number): The radius of the circle in meters
  - Returns: The area of the circle in square meters

**calculate_fire_spread_risk(separation_distance, building_height)**
  Calculate fire spread risk based on building separation distance. This demonstrates a more domain-specific function for fire safety engineering.
  
  - ``separation_distance`` (number): Distance between buildings in meters  
  - ``building_height`` (number): Height of the source building in meters
  - Returns: Risk factor (0.0 = no risk, 1.0 = maximum risk)

API Design Principles
---------------------

Hierarchical Organization
~~~~~~~~~~~~~~~~~~~~~~~~~

Functions are organized to match fire engineering document structure:

.. code-block:: javascript

   // Document -> Chapter -> Function
   fdt.chapter_2.hot_gas_temperature_increase(...)
   fdt.chapter_4.heat_transfer_coefficient_short_times(...)
   fdt.chapter_5.thermal_penetration_time(...)

Consistent Parameter Units
~~~~~~~~~~~~~~~~~~~~~~~~~~

All functions use consistent SI units:

- **Length**: meters (m)
- **Area**: square meters (m²)
- **Temperature**: Kelvin (K) 
- **Heat Release Rate**: kilowatts (kW)
- **Time**: seconds (s)
- **Density**: kilograms per cubic meter (kg/m³)
- **Thermal Properties**: SI standard units

Clear Function Names
~~~~~~~~~~~~~~~~~~~~

Function names clearly indicate their purpose and source equation where applicable:

- ``hot_gas_temperature_increase`` - calculates temperature rise
- ``density_hot_gas_layer`` - calculates gas layer density
- ``thermal_penetration_time`` - calculates heat penetration timing

Quick Reference
---------------

Fire Dynamics Tools
~~~~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1

   * - Function
     - Chapter
     - Description
   * - ``hot_gas_temperature_increase``
     - 2
     - Temperature increase in compartment fire
   * - ``density_hot_gas_layer``
     - 2
     - Hot gas layer density calculation
   * - ``heat_transfer_coefficient_short_times``
     - 4
     - Heat transfer coefficient (short times)
   * - ``thermal_penetration_time``
     - 5
     - Thermal penetration through materials

Utility Functions
~~~~~~~~~~~~~~~~~

.. list-table::
   :header-rows: 1

   * - Function
     - Description
   * - ``calculate_circle_area``
     - Calculate area of circle (demonstration)
   * - ``calculate_fire_spread_risk``
     - Fire spread risk assessment (example)

Usage Examples
--------------

Basic Fire Analysis
~~~~~~~~~~~~~~~~~~~

.. code-block:: javascript

   import init, { get_fire_dynamics_tools } from 'openfire-wasm';

   async function fireAnalysis() {
     await init();
     const fdt = get_fire_dynamics_tools();
     
     // Input parameters
     const heatReleaseRate = 1000; // kW
     const compartmentArea = 50;   // m²
     const compartmentHeight = 3;  // m
     
     // Calculate temperature increase
     const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
       heatReleaseRate, compartmentArea, compartmentHeight
     );
     
     // Calculate hot gas temperature
     const hotGasTemp = 293 + tempIncrease; // K (20°C ambient + increase)
     
     // Calculate density
     const density = fdt.chapter_2.density_hot_gas_layer(hotGasTemp);
     
     console.log(`Temperature increase: ${tempIncrease.toFixed(1)} K`);
     console.log(`Hot gas density: ${density.toFixed(3)} kg/m³`);
   }

Error Handling
~~~~~~~~~~~~~~

.. code-block:: javascript

   async function safeCalculation() {
     try {
       await init();
       const fdt = get_fire_dynamics_tools();
       
       const result = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
       return result;
     } catch (error) {
       console.error('API call failed:', error);
       return null;
     }
   }

Documentation Structure
-----------------------

This API reference follows the same organizational pattern as the `OpenFire Python API <https://fire-library.github.io/openfire/api/>`_:

1. **Installation** - Setup and configuration
2. **User Guide** - Getting started and examples
3. **API Reference** - Complete function documentation

Related Resources
-----------------

- :doc:`../guide/getting-started` - Basic usage guide
- :doc:`../guide/fire-dynamics-examples` - Practical examples
- :doc:`../installation` - Setup instructions
- `Python API Documentation <https://fire-library.github.io/openfire/>`_ - Equivalent Python interface