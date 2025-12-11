Fire Dynamics Tools
===================

General fire dynamics calculations organized by document chapter.

Overview
--------

The Fire Dynamics Tools module provides calculations from various chapters of fire dynamics engineering documents. Functions are organized hierarchically by chapter for easy navigation and reference.

Accessing the Module
--------------------

.. code-block:: javascript

   import init, { get_fire_dynamics_tools } from 'openfire-wasm';

   await init();
   const fdt = get_fire_dynamics_tools();

Main API Function
-----------------

get_fire_dynamics_tools()
~~~~~~~~~~~~~~~~~~~~~~~~~

Returns an instance that provides access to organized fire dynamics calculations by document chapter.

**Returns:**
  FireDynamicsTools instance with chapter-organized functions

**Example:**

.. code-block:: javascript

   import init, { get_fire_dynamics_tools } from 'openfire-wasm';
   
   await init();
   const fdt = get_fire_dynamics_tools();
   const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);

Chapter Organization
--------------------

Chapter 2 - Compartment Fire Dynamics
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Functions related to fire behavior in compartments, including hot gas temperatures and layer properties.

**Class:** Chapter2

**Methods:**

hot_gas_temperature_increase(heat_release_rate, compartment_area, compartment_height)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Calculates the temperature increase of hot gases in a compartment fire. Simplified version for demonstration of namespace organization.

**Parameters:**
  - ``heat_release_rate`` (number): Heat release rate in kW
  - ``compartment_area`` (number): Floor area of compartment in m²  
  - ``compartment_height`` (number): Height of compartment in m

**Returns:**
  number: Temperature increase in Kelvin

**Example:**

.. code-block:: javascript

   const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
   console.log(`Temperature increase: ${tempIncrease} K`);

**Equation:**

.. math::

   \Delta T_g = 6.85 \times \left(\frac{Q^2}{A \times h}\right)^{1/3}

Where:
  - :math:`\Delta T_g` = Hot gas temperature increase (K)
  - :math:`Q` = Heat release rate (kW)
  - :math:`A` = Compartment floor area (m²)
  - :math:`h` = Compartment height (m)

density_hot_gas_layer(hot_gas_temperature)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Calculates the density of the hot gas layer based on temperature using the ideal gas law.

**Parameters:**
  - ``hot_gas_temperature`` (number): Temperature of hot gas layer in Kelvin

**Returns:**
  number: Density in kg/m³

**Example:**

.. code-block:: javascript

   const density = fdt.chapter_2.density_hot_gas_layer(573); // 300°C
   console.log(`Hot gas density: ${density.toFixed(3)} kg/m³`);

**Equation:**

.. math::

   \rho = \frac{P}{R \times T}

Where:
  - :math:`\rho` = Density (kg/m³)
  - :math:`P` = Pressure (Pa) - assumes standard atmospheric pressure
  - :math:`R` = Specific gas constant for air (287 J/(kg·K))
  - :math:`T` = Temperature (K)

Chapter 4 - Heat Transfer
~~~~~~~~~~~~~~~~~~~~~~~~~

Heat transfer calculations for compartment boundaries and structural elements.

**Class:** Chapter4

**Methods:**

heat_transfer_coefficient_short_times(thermal_conductivity, thermal_penetration_time)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Calculate heat transfer coefficient for short times or thick walls.

**Parameters:**
  - ``thermal_conductivity`` (number): Thermal conductivity in W/(m·K)
  - ``thermal_penetration_time`` (number): Thermal penetration time in seconds

**Returns:**
  number: Heat transfer coefficient in W/(m²·K)

**Example:**

.. code-block:: javascript

   const heatCoeff = fdt.chapter_4.heat_transfer_coefficient_short_times(0.5, 100);
   console.log(`Heat transfer coefficient: ${heatCoeff} W/(m²·K)`);

Chapter 5 - Smoke Movement
~~~~~~~~~~~~~~~~~~~~~~~~~~

Calculations related to smoke movement and thermal penetration.

**Class:** Chapter5

**Methods:**

thermal_penetration_time(thermal_diffusivity, thickness)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Calculate thermal penetration time through a material.

**Parameters:**
  - ``thermal_diffusivity`` (number): Thermal diffusivity in m²/s
  - ``thickness`` (number): Wall thickness in meters

**Returns:**
  number: Thermal penetration time in seconds

**Example:**

.. code-block:: javascript

   const penetrationTime = fdt.chapter_5.thermal_penetration_time(1.2e-7, 0.1);
   console.log(`Penetration time: ${penetrationTime} seconds`);

**Equation:**

.. math::

   t_p = \frac{\delta^2}{\alpha}

Where:
  - :math:`t_p` = Thermal penetration time (s)
  - :math:`\delta` = Material thickness (m)
  - :math:`\alpha` = Thermal diffusivity (m²/s)

Usage Patterns
--------------

Basic Calculation
~~~~~~~~~~~~~~~~~

.. code-block:: javascript

   import init, { get_fire_dynamics_tools } from 'openfire-wasm';

   async function basicCalculation() {
     await init();
     const fdt = get_fire_dynamics_tools();
     
     const result = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
     console.log(result);
   }

Chained Calculations
~~~~~~~~~~~~~~~~~~~~

.. code-block:: javascript

   async function chainedCalculations() {
     await init();
     const fdt = get_fire_dynamics_tools();
     
     // Calculate temperature increase
     const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(1500, 75, 3.5);
     
     // Calculate hot gas temperature
     const ambientTemp = 293; // K
     const hotGasTemp = ambientTemp + tempIncrease;
     
     // Calculate density at that temperature
     const density = fdt.chapter_2.density_hot_gas_layer(hotGasTemp);
     
     console.log(`Final density: ${density} kg/m³`);
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
       console.error('Calculation failed:', error);
       return null;
     }
   }

Related Documentation
---------------------

- :doc:`../guide/getting-started` - Basic usage examples
- :doc:`../guide/fire-dynamics-examples` - Practical calculation examples
- :doc:`../installation` - Setup instructions

Future Modules
--------------

The following modules are planned for future releases:

- **PD 7974** - Fire Safety Engineering Principles
- **BR 187** - External Fire Spread
- **BS 9999** - Fire Safety in Buildings  
- **CIBSE Guide E** - Fire Safety Engineering
- **SFPE Handbook** - Fire Protection Engineering
- **TR 17** - Technical Report