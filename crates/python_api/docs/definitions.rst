Definitions
===========

This page contains explanations of key terms and concepts used throughout the OpenFire fire engineering documentation.

.. _stack-effect:

Stack Effect
------------

The **stack effect** is a physical phenomenon that occurs when there is a temperature difference between indoor and outdoor environments, or between different zones within a building. This temperature difference creates a density difference in the air masses, resulting in a pressure differential that drives airflow.

**Physical Mechanism:**

When air inside a building (or shaft) is warmer than the outside air, the warm air becomes less dense and tends to rise. This creates a pressure difference where the pressure at the bottom of the building is lower than outside, and the pressure at the top is higher than outside. There exists a neutral pressure level (NPL) where the indoor and outdoor pressures are equal.

**Mathematical Representation:**

The pressure difference due to stack effect can be calculated using:

.. math::

   \Delta P_{so} = 3460 \cdot \left(\frac{1}{T_0 + 273} - \frac{1}{T_s + 273}\right) \cdot z

where:

- :math:`\Delta P_{so}` is the pressure difference due to stack effect (Pa)
- :math:`T_0` is the outdoor (or surrounding) temperature (°C) 
- :math:`T_s` is the shaft (or compartment) temperature (°C)
- :math:`z` is the height above the neutral pressure level (m)

**Engineering Significance:**

Stack effect is important in fire engineering for:

- **Smoke movement**: Understanding how smoke will move through buildings during a fire
- **Ventilation design**: Calculating natural ventilation rates and airflow patterns
- **Smoke control systems**: Designing mechanical systems to overcome or utilize stack effect
- **Fire compartmentalization**: Predicting pressure differentials that affect fire spread

**Related Equations:**

The stack effect calculation appears in several OpenFire modules:

- :ref:`SFPE Handbook Equation 50.1 <ofire.sfpe_handbook.chapter_50.equation_50_1.pressure_difference>` - Basic stack effect pressure difference
- :ref:`SFPE Handbook Equation 50.2 <ofire.sfpe_handbook.chapter_50.equation_50_2.pressure_difference>` - Stack effect in fire compartments

**Limitations:**

Stack effect calculations may not be applicable for:

- Complex buildings with multiple shafts of varying heights
- Buildings with varying shaft temperatures
- Situations with significant wind effects that override buoyancy forces