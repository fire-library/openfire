Chapter 50
----------

Example 2: Door Opening Force
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   """
   OpenFire Example — SFPE Handbook Chapter 50
   Topic: Door opening force for a side-hinged swinging door (Eq. 50.14)

   Unit policy:
   - All calculations performed in SI units (m, Pa, N).
   - Inputs may be provided in non-SI, but are converted explicitly before use.

   Given conversions (per user requirement):
   - 1 ft = 0.3048 m
   - 1 in = 0.0254 m
   - 1 in H2O = 249.09 Pa

   Additional necessary conversion:
   - 1 lbf = 4.4482216152605 N
   """

   import ofire


   # =============================================================================
   # 1) Unit conversions (explicit)
   # =============================================================================
   FT_TO_M = 0.3048
   IN_TO_M = 0.0254
   IN_H2O_TO_PA = 249.09
   LBF_TO_N = 4.4482216152605


   # =============================================================================
   # 2) Inputs (as given in the problem statement)
   # =============================================================================
   door_width_ft = 3.0
   door_height_ft = 7.0
   closer_force_lbf = 9.0
   delta_p_in_h2o = 0.35
   knob_offset_from_edge_in = 3.0  # knob is 3 in from the door edge (knob side)


   # =============================================================================
   # 3) Convert inputs to SI (SI-only variables used in calculations)
   # =============================================================================
   W_m = door_width_ft * FT_TO_M
   H_m = door_height_ft * FT_TO_M
   A_m2 = W_m * H_m

   F_dc_N = closer_force_lbf * LBF_TO_N
   delta_p_Pa = delta_p_in_h2o * IN_H2O_TO_PA
   d_m = knob_offset_from_edge_in * IN_TO_M


   # =============================================================================
   # 4) Calculation (OpenFire — SFPE HB Ch.50 Eq. 50.14)
   # =============================================================================
   F_open_N = ofire.sfpe_handbook.chapter_50.equation_50_14.door_opening_force(
       f_dc=F_dc_N,
       w=W_m,
       a=A_m2,
       delta_p=delta_p_Pa,
       d=d_m,
   )


   # =============================================================================
   # 5) Print inputs and results (always)
   # =============================================================================
   print("=" * 72)
   print("OpenFire — SFPE Handbook Ch.50 — Door Opening Force (Eq. 50.14)")
   print("=" * 72)

   print("\nINPUTS (original, as stated):")
   print(f"  Door width                 = {door_width_ft:.3f} ft")
   print(f"  Door height                = {door_height_ft:.3f} ft")
   print(f"  Door closer force          = {closer_force_lbf:.3f} lbf")
   print(f"  Pressure difference, Δp    = {delta_p_in_h2o:.3f} in H2O")
   print(f"  Knob offset from edge      = {knob_offset_from_edge_in:.3f} in")

   print("\nINPUTS (converted to SI; used in calculation):")
   print(f"  Door width, W              = {W_m:.6f} m")
   print(f"  Door height, H             = {H_m:.6f} m")
   print(f"  Door area, A               = {A_m2:.6f} m^2")
   print(f"  Door closer force, F_dc    = {F_dc_N:.6f} N")
   print(f"  Pressure difference, Δp    = {delta_p_Pa:.6f} Pa")
   print(f"  Knob offset, d             = {d_m:.6f} m")

   print("\nRESULTS (SI):")
   print(f"  Total door opening force, F = {F_open_N:.6f} N")

   print("\nNotes:")
   print("  - All calculations performed in SI units (m, Pa, N).")
   print("  - OpenFire function: ofire.sfpe_handbook.chapter_50.equation_50_14.door_opening_force")
   print("=" * 72)


Example 3: Simple Stairwell Pressurisation in a Simple Building
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Part 1
^^^^^^

.. code-block:: python

   # Code to be added soon


Part 2
^^^^^^

.. code-block:: python

   # Code to be added soon


Part 3
^^^^^^

.. code-block:: python

   # Code to be added soon