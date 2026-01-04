BR 187
======

This page demonstrates practical examples of thermal radiation calculations using BR 187 equations.

Complete Thermal Radiation Assessment
--------------------------------------

This example demonstrates a complete thermal radiation analysis workflow using BR 187 equations.
We'll calculate the thermal radiation received from a radiating surface (such as a fire through a window opening).

.. code-block:: python

   import ofire

   # Step 1: Calculate source radiation intensity from the radiating surface
   sigma = 5.67e-11  # Stefan-Boltzmann constant (kW/m²K⁴)
   emissivity = 0.9  # Surface emissivity
   temperature = 1273  # Surface temperature (K) - approximately 1000°C
   
   I_s = ofire.br_187.appendix_a.equation_a1.radiation_intensity(
       sigma, emissivity, temperature
   )
   print(f"Source intensity: {I_s:.0f} kW/m²")

   # Step 2: Calculate dimensionless parameters for the radiating surface
   width = 4.0     # Radiating surface width (m) - e.g., window width
   height = 2.5    # Radiating surface height (m) - e.g., window height
   distance = 12.0 # Distance from radiating surface to receiver (m)
   
   X = ofire.br_187.appendix_a.equation_a3.x(width, distance)
   Y = ofire.br_187.appendix_a.equation_a3.y(height, distance)
   
   print(f"Dimensionless width parameter (X): {X:.3f}")
   print(f"Dimensionless height parameter (Y): {Y:.3f}")
   
   # Step 3: Calculate view factor for parallel, centre-aligned surfaces
   phi = ofire.br_187.appendix_a.equation_a3.phi(X, Y, additive=True)
   print(f"View factor: {phi:.4f}")
   
   # Step 4: Calculate received radiation intensity at the target
   I_R = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(
       phi, I_s
   )
   print(f"Received intensity: {I_R:.1f} kW/m²")

This workflow calculates the thermal radiation from a radiating surface (like flames visible through a window)
to a target receiver surface on an adjacent building, which is essential for external fire spread assessment.

Multi-Configuration Analysis
----------------------------

Find received radiation at different distances:

.. code-block:: python

   import ofire

   # Source parameters
   I_s = 120.0  # Source intensity (kW/m²)
   width = 4.0  # Radiating surface width (m)
   height = 2.5 # Radiating surface height (m)
   
   # Test different distances
   distances = [8.0, 12.0, 16.0, 20.0]  # Various separation distances (m)
   
   print("Distance (m) | View Factor | Received Intensity (kW/m²)")
   print("-" * 55)
   
   for distance in distances:
       # Calculate parameters for centre-aligned configuration
       X = ofire.br_187.appendix_a.equation_a3.x(width, distance)
       Y = ofire.br_187.appendix_a.equation_a3.y(height, distance)
       phi = ofire.br_187.appendix_a.equation_a3.phi(X, Y, additive=True)
       
       I_R = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(
           phi, I_s
       )
       
       print(f"{distance:8.1f}     | {phi:10.4f}  | {I_R:18.1f}")

The results can be further analyzed by plotting the relationship between distance and received intensity
using libraries like matplotlib or plotly for visualization.

Different Surface Configurations
--------------------------------

Compare different view factor equations for various geometric arrangements:

.. code-block:: python

   import ofire

   # Common parameters
   width = 3.0     # Radiating surface width (m)
   height = 2.0    # Radiating surface height (m)
   distance = 10.0 # Distance to receiver (m)
   I_s = 100.0     # Source intensity (kW/m²)

   print("Configuration Analysis")
   print("=" * 50)
   
   # Configuration 1: Centre-aligned parallel surfaces (Equation A3)
   X_a3 = ofire.br_187.appendix_a.equation_a3.x(width, distance)
   Y_a3 = ofire.br_187.appendix_a.equation_a3.y(height, distance)
   phi_a3 = ofire.br_187.appendix_a.equation_a3.phi(X_a3, Y_a3, additive=True)
   I_R_a3 = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(phi_a3, I_s)
   
   print(f"A3 - Centre-aligned parallel: {I_R_a3:.1f} kW/m²")
   
   # Configuration 2: Corner-aligned parallel surfaces (Equation A4)
   X_a4 = ofire.br_187.appendix_a.equation_a4.x(width, distance)
   Y_a4 = ofire.br_187.appendix_a.equation_a4.y(height, distance)
   phi_a4 = ofire.br_187.appendix_a.equation_a4.phi(X_a4, Y_a4, additive=True)
   I_R_a4 = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(phi_a4, I_s)
   
   print(f"A4 - Corner-aligned parallel: {I_R_a4:.1f} kW/m²")
   
   # Configuration 3: Corner-aligned perpendicular surfaces (Equation A5)
   X_a5 = ofire.br_187.appendix_a.equation_a5.x(width, distance)
   Y_a5 = ofire.br_187.appendix_a.equation_a5.y(height, distance)
   phi_a5 = ofire.br_187.appendix_a.equation_a5.phi(X_a5, Y_a5, additive=True)
   I_R_a5 = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(phi_a5, I_s)
   
   print(f"A5 - Corner-aligned perpendicular: {I_R_a5:.1f} kW/m²")