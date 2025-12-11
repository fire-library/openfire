Getting Started
===============

This guide will help you get up and running with OpenFire WASM for fire safety engineering calculations.

Overview
--------

OpenFire WASM provides fire safety engineering calculations organized by source documents and chapters, matching the structure used in fire engineering standards and guides.

Basic Usage
-----------

Initialize the Library
~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: javascript

   import init, { get_fire_dynamics_tools } from 'openfire-wasm';

   // Initialize the WASM module (required for web browsers)
   await init();

   // Get the main API namespace
   const fdt = get_fire_dynamics_tools();

Your First Calculation
~~~~~~~~~~~~~~~~~~~~~~~

Let's calculate the hot gas temperature increase in a compartment fire:

.. code-block:: javascript

   // Fire parameters
   const heatReleaseRate = 1000; // kW
   const compartmentArea = 50;   // m²
   const compartmentHeight = 3;  // m

   // Calculate temperature increase using Fire Dynamics Tools, Chapter 2
   const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
     heatReleaseRate, 
     compartmentArea, 
     compartmentHeight
   );

   console.log(`Hot gas temperature increase: ${tempIncrease.toFixed(1)} K`);

Hot Gas Layer Density
~~~~~~~~~~~~~~~~~~~~~

Calculate the density of the hot gas layer:

.. code-block:: javascript

   // Temperature of hot gas layer
   const hotGasTemperature = 573; // K (300°C)

   // Calculate density
   const density = fdt.chapter_2.density_hot_gas_layer(hotGasTemperature);

   console.log(`Hot gas layer density: ${density.toFixed(3)} kg/m³`);

API Organization
----------------

The OpenFire WASM API is organized by document and chapter, similar to fire engineering standards:

Fire Dynamics Tools
~~~~~~~~~~~~~~~~~~~~

- **Chapter 2**: Compartment Fire Dynamics
  
  - Hot gas temperature calculations
  - Gas layer density calculations

- **Chapter 4**: Heat Transfer
  
  - Heat transfer coefficients

- **Chapter 5**: Smoke Movement
  
  - Thermal penetration time

Future Modules (Coming Soon)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- **PD 7974**: Fire Safety Engineering Principles
- **BR 187**: External Fire Spread  
- **BS 9999**: Fire Safety in Buildings
- **CIBSE Guide E**: Fire Safety Engineering

Complete Example
----------------

Here's a complete example that demonstrates multiple calculations:

.. code-block:: javascript

   import init, { get_fire_dynamics_tools, calculate_circle_area } from 'openfire-wasm';

   async function fireAnalysis() {
     // Initialize WASM
     await init();
     
     // Get Fire Dynamics Tools
     const fdt = get_fire_dynamics_tools();
     
     // Compartment properties
     const heatReleaseRate = 2000; // kW
     const compartmentArea = 75;   // m²
     const compartmentHeight = 3.5; // m
     
     // Chapter 2 calculations
     console.log('=== Fire Dynamics Analysis ===');
     
     // Hot gas temperature increase
     const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
       heatReleaseRate, compartmentArea, compartmentHeight
     );
     console.log(`Temperature increase: ${tempIncrease.toFixed(1)} K`);
     
     // Calculate hot gas temperature (ambient + increase)
     const ambientTemp = 293; // K (20°C)
     const hotGasTemp = ambientTemp + tempIncrease;
     console.log(`Hot gas temperature: ${hotGasTemp.toFixed(1)} K`);
     
     // Hot gas layer density
     const density = fdt.chapter_2.density_hot_gas_layer(hotGasTemp);
     console.log(`Hot gas density: ${density.toFixed(3)} kg/m³`);
     
     // Chapter 4 - Heat transfer coefficient
     const thermalConductivity = 0.5; // W/(m·K)
     const thermalPenetrationTime = 100; // s
     const heatTransferCoeff = fdt.chapter_4.heat_transfer_coefficient_short_times(
       thermalConductivity, thermalPenetrationTime
     );
     console.log(`Heat transfer coefficient: ${heatTransferCoeff.toFixed(2)} W/(m²·K)`);
     
     // Chapter 5 - Thermal penetration time
     const thermalDiffusivity = 1.2e-7; // m²/s
     const wallThickness = 0.1; // m
     const penetrationTime = fdt.chapter_5.thermal_penetration_time(
       thermalDiffusivity, wallThickness
     );
     console.log(`Thermal penetration time: ${penetrationTime.toFixed(0)} s`);
     
     // Utility function example
     const ventilationArea = calculate_circle_area(1.0); // 1m radius
     console.log(`Circular vent area: ${ventilationArea.toFixed(2)} m²`);
   }

   // Run the analysis
   fireAnalysis().catch(console.error);

Error Handling
--------------

Always wrap WASM initialization and calculations in try-catch blocks:

.. code-block:: javascript

   async function safeCalculation() {
     try {
       await init();
       const fdt = get_fire_dynamics_tools();
       const result = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
       console.log(`Result: ${result}`);
     } catch (error) {
       console.error('Calculation failed:', error);
     }
   }

Next Steps
----------

- :doc:`fire-dynamics-examples` - More detailed calculation examples
- :doc:`../api/fire-dynamics-tools` - Complete function documentation
- :doc:`../installation` - Setup and troubleshooting