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


Example 3: Simple Stairwell Pressurization in a Simple Building
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Part 1
^^^^^^

.. code-block:: python

   """
   OpenFire Example — SFPE Handbook Chapter 50 (Smoke Control)
   Example 3 (Part 1): Simple Stairwell Pressurization in a Simple Building

   Question (Part 1):
   - Stairwells pressurized with UNTREATED outside air.
   - Can the stairwell be pressurized? (Check height limit vs building height)

   Method (OpenFire):
   - Eq. 50.17: Stairwell temperature for untreated pressurization air, T_s
   - Eq. 50.16: Flow area factor, F_r
   - Eq. 50.15: Height limit, H_m

   Decision criterion:
   - If H_m >= building height, then the stairwell can be pressurized (under these assumptions).

   Unit policy:
   - All calculations performed in SI units (m, m², Pa, °C).
   - Inputs provided in Imperial / °F are explicitly converted.

   Given conversions (per user requirement):
   - 1 ft = 0.3048 m
   - 1 in H2O = 249.09 Pa
   """

   import ofire


   # =============================================================================
   # 1) Unit conversions (explicit)
   # =============================================================================
   FT_TO_M = 0.3048
   IN_H2O_TO_PA = 249.09

   # Additional necessary conversions (temperature, area)
   def f_to_c(temp_f: float) -> float:
       """Convert temperature from °F to °C."""
       return (temp_f - 32.0) * (5.0 / 9.0)

   def ft2_to_m2(area_ft2: float) -> float:
       """Convert area from ft² to m² using 1 ft = 0.3048 m."""
       return area_ft2 * (FT_TO_M ** 2)


   # =============================================================================
   # 2) Inputs (as given in the problem statement)
   # =============================================================================
   t_outdoor_f = 10.0
   t_building_f = 70.0

   delta_p_min_in_h2o = 0.10
   delta_p_max_in_h2o = 0.35

   floor_to_floor_height_ft = 10.0
   building_height_ft = 200.0

   a_sb_ft2 = 0.34  # flow area between stairwell and building (typical floor)
   a_bo_ft2 = 0.30  # flow area per stairwell between building and outside

   eta = 0.15  # heat transfer factor (given / suggested)


   # =============================================================================
   # 3) Convert inputs to SI (SI-only variables used in calculations)
   # =============================================================================
   t_outdoor_c = f_to_c(t_outdoor_f)
   t_building_c = f_to_c(t_building_f)

   delta_p_min_pa = delta_p_min_in_h2o * IN_H2O_TO_PA
   delta_p_max_pa = delta_p_max_in_h2o * IN_H2O_TO_PA

   floor_to_floor_height_m = floor_to_floor_height_ft * FT_TO_M
   building_height_m = building_height_ft * FT_TO_M

   a_sb_m2 = ft2_to_m2(a_sb_ft2)
   a_bo_m2 = ft2_to_m2(a_bo_ft2)


   # =============================================================================
   # 4) Calculation (OpenFire — SFPE HB Ch.50 Eq. 50.17, 50.16, 50.15)
   # =============================================================================
   # Eq. 50.17 — stairwell temperature for untreated air
   t_stair_c = ofire.sfpe_handbook.chapter_50.equation_50_17.stairwell_temperature(
       t_0=t_outdoor_c,
       eta=eta,
       t_b=t_building_c,
   )

   # Eq. 50.16 — flow area factor
   f_r = ofire.sfpe_handbook.chapter_50.equation_50_16.factor(
       a_sb=a_sb_m2,
       a_bo=a_bo_m2,
       t_b=t_building_c,
       t_s=t_stair_c,
   )

   # Eq. 50.15 — height limit
   h_m = ofire.sfpe_handbook.chapter_50.equation_50_15.height_limit(
       f_r=f_r,
       delta_p_max=delta_p_max_pa,
       delta_p_min=delta_p_min_pa,
       t_0=t_outdoor_c,
       t_s=t_stair_c,
   )

   can_pressurize = h_m >= building_height_m


   # =============================================================================
   # 5) Print inputs and results (always)
   # =============================================================================
   print("=" * 72)
   print("OpenFire — SFPE Handbook Ch.50 — Example 3 (Part 1)")
   print("Simple Stairwell Pressurization (Untreated Outside Air)")
   print("=" * 72)

   print("\nINPUTS (original, as stated):")
   print(f"  Outdoor temperature, T_o         = {t_outdoor_f:.3f} °F")
   print(f"  Building temperature, T_b        = {t_building_f:.3f} °F")
   print(f"  Min pressure diff, Δp_min        = {delta_p_min_in_h2o:.3f} in H2O")
   print(f"  Max pressure diff, Δp_max        = {delta_p_max_in_h2o:.3f} in H2O")
   print(f"  Floor-to-floor height            = {floor_to_floor_height_ft:.3f} ft")
   print(f"  Building height                  = {building_height_ft:.3f} ft")
   print(f"  Flow area stair→building, A_SB    = {a_sb_ft2:.3f} ft²")
   print(f"  Flow area building→outside, A_BO  = {a_bo_ft2:.3f} ft² (per stairwell)")
   print(f"  Heat transfer factor, η          = {eta:.3f}")

   print("\nINPUTS (converted to SI; used in calculation):")
   print(f"  Outdoor temperature, T_o         = {t_outdoor_c:.6f} °C")
   print(f"  Building temperature, T_b        = {t_building_c:.6f} °C")
   print(f"  Min pressure diff, Δp_min        = {delta_p_min_pa:.6f} Pa")
   print(f"  Max pressure diff, Δp_max        = {delta_p_max_pa:.6f} Pa")
   print(f"  Floor-to-floor height            = {floor_to_floor_height_m:.6f} m")
   print(f"  Building height                  = {building_height_m:.6f} m")
   print(f"  Flow area stair→building, A_SB    = {a_sb_m2:.8f} m²")
   print(f"  Flow area building→outside, A_BO  = {a_bo_m2:.8f} m²")

   print("\nRESULTS (SI):")
   print(f"  Stairwell temperature, T_s       = {t_stair_c:.6f} °C  (Eq. 50.17)")
   print(f"  Flow area factor, F_r            = {f_r:.6f} (-)      (Eq. 50.16)")
   print(f"  Height limit, H_m                = {h_m:.6f} m        (Eq. 50.15)")
   print(f"  Decision (H_m >= building height) = {can_pressurize}")

   print("\nCONCLUSION:")
   if can_pressurize:
       print("  The stairwell CAN be pressurized under these assumptions.")
   else:
       print("  The stairwell CANNOT be pressurized under these assumptions.")

   print("\nNotes:")
   print("  - All calculations performed in SI units (m, m², Pa, °C).")
   print("  - OpenFire functions used:")
   print("      * Eq. 50.17: ofire.sfpe_handbook.chapter_50.equation_50_17.stairwell_temperature")
   print("      * Eq. 50.16: ofire.sfpe_handbook.chapter_50.equation_50_16.factor")
   print("      * Eq. 50.15: ofire.sfpe_handbook.chapter_50.equation_50_15.height_limit")
   print("=" * 72)


Part 2
^^^^^^

.. code-block:: python

   """
   OpenFire Example — SFPE Handbook Chapter 50 (Smoke Control)
   Example 3 (Part 2): Simple Stairwell Pressurization in a Simple Building

   Question (Part 2):
   - Stairwells pressurized with TREATED air at 70°F (i.e., supply air temperature).
   - Can the stairwell be pressurized? (Check height limit vs building height)

   Method (OpenFire):
   - Eq. 50.16: Flow area factor, F_r
   - Eq. 50.15: Height limit, H_m

   Decision criterion:
   - If H_m >= building height, then the stairwell can be pressurized (under these assumptions).

   Unit policy:
   - All calculations performed in SI units (m, m², Pa, °C).
   - Inputs provided in Imperial / °F are explicitly converted.

   Given conversions (per user requirement):
   - 1 ft = 0.3048 m
   - 1 in H2O = 249.09 Pa
   """

   import ofire


   # =============================================================================
   # 1) Unit conversions (explicit)
   # =============================================================================
   FT_TO_M = 0.3048
   IN_H2O_TO_PA = 249.09

   # Additional necessary conversions (temperature, area)
   def f_to_c(temp_f: float) -> float:
       """Convert temperature from °F to °C."""
       return (temp_f - 32.0) * (5.0 / 9.0)

   def ft2_to_m2(area_ft2: float) -> float:
       """Convert area from ft² to m² using 1 ft = 0.3048 m."""
       return area_ft2 * (FT_TO_M ** 2)


   # =============================================================================
   # 2) Inputs (as given in the problem statement)
   # =============================================================================
   t_outdoor_f = 10.0
   t_building_f = 70.0

   # Part 2 condition: treated stair pressurization air temperature
   t_stair_supply_f = 70.0

   delta_p_min_in_h2o = 0.10
   delta_p_max_in_h2o = 0.35

   floor_to_floor_height_ft = 10.0
   building_height_ft = 200.0

   a_sb_ft2 = 0.34  # flow area between stairwell and building (typical floor)
   a_bo_ft2 = 0.30  # flow area per stairwell between building and outside


   # =============================================================================
   # 3) Convert inputs to SI (SI-only variables used in calculations)
   # =============================================================================
   t_outdoor_c = f_to_c(t_outdoor_f)
   t_building_c = f_to_c(t_building_f)
   t_stair_c = f_to_c(t_stair_supply_f)

   delta_p_min_pa = delta_p_min_in_h2o * IN_H2O_TO_PA
   delta_p_max_pa = delta_p_max_in_h2o * IN_H2O_TO_PA

   floor_to_floor_height_m = floor_to_floor_height_ft * FT_TO_M
   building_height_m = building_height_ft * FT_TO_M

   a_sb_m2 = ft2_to_m2(a_sb_ft2)
   a_bo_m2 = ft2_to_m2(a_bo_ft2)


   # =============================================================================
   # 4) Calculation (OpenFire — SFPE HB Ch.50 Eq. 50.16 and 50.15)
   # =============================================================================
   # Eq. 50.16 — flow area factor
   f_r = ofire.sfpe_handbook.chapter_50.equation_50_16.factor(
       a_sb=a_sb_m2,
       a_bo=a_bo_m2,
       t_b=t_building_c,
       t_s=t_stair_c,
   )

   # Eq. 50.15 — height limit
   h_m = ofire.sfpe_handbook.chapter_50.equation_50_15.height_limit(
       f_r=f_r,
       delta_p_max=delta_p_max_pa,
       delta_p_min=delta_p_min_pa,
       t_0=t_outdoor_c,
       t_s=t_stair_c,
   )

   can_pressurize = h_m >= building_height_m


   # =============================================================================
   # 5) Print inputs and results (always)
   # =============================================================================
   print("=" * 72)
   print("OpenFire — SFPE Handbook Ch.50 — Example 3 (Part 2)")
   print("Simple Stairwell Pressurization (Treated Air at 70°F)")
   print("=" * 72)

   print("\nINPUTS (original, as stated):")
   print(f"  Outdoor temperature, T_o          = {t_outdoor_f:.3f} °F")
   print(f"  Building temperature, T_b         = {t_building_f:.3f} °F")
   print(f"  Treated stair supply temp, T_s    = {t_stair_supply_f:.3f} °F")
   print(f"  Min pressure diff, Δp_min         = {delta_p_min_in_h2o:.3f} in H2O")
   print(f"  Max pressure diff, Δp_max         = {delta_p_max_in_h2o:.3f} in H2O")
   print(f"  Floor-to-floor height             = {floor_to_floor_height_ft:.3f} ft")
   print(f"  Building height                   = {building_height_ft:.3f} ft")
   print(f"  Flow area stair→building, A_SB     = {a_sb_ft2:.3f} ft²")
   print(f"  Flow area building→outside, A_BO   = {a_bo_ft2:.3f} ft² (per stairwell)")

   print("\nINPUTS (converted to SI; used in calculation):")
   print(f"  Outdoor temperature, T_o          = {t_outdoor_c:.6f} °C")
   print(f"  Building temperature, T_b         = {t_building_c:.6f} °C")
   print(f"  Stairwell temperature, T_s        = {t_stair_c:.6f} °C")
   print(f"  Min pressure diff, Δp_min         = {delta_p_min_pa:.6f} Pa")
   print(f"  Max pressure diff, Δp_max         = {delta_p_max_pa:.6f} Pa")
   print(f"  Floor-to-floor height             = {floor_to_floor_height_m:.6f} m")
   print(f"  Building height                   = {building_height_m:.6f} m")
   print(f"  Flow area stair→building, A_SB     = {a_sb_m2:.8f} m²")
   print(f"  Flow area building→outside, A_BO   = {a_bo_m2:.8f} m²")

   print("\nRESULTS (SI):")
   print(f"  Flow area factor, F_r             = {f_r:.6f} (-)      (Eq. 50.16)")
   print(f"  Height limit, H_m                 = {h_m:.6f} m        (Eq. 50.15)")
   print(f"  Decision (H_m >= building height) = {can_pressurize}")

   print("\nCONCLUSION:")
   if can_pressurize:
       print("  The stairwell CAN be pressurized under these assumptions.")
   else:
       print("  The stairwell CANNOT be pressurized under these assumptions.")

   print("\nNotes:")
   print("  - All calculations performed in SI units (m, m², Pa, °C).")
   print("  - For treated air, T_s is taken as the treated supply temperature.")
   print("  - OpenFire functions used:")
   print("      * Eq. 50.16: ofire.sfpe_handbook.chapter_50.equation_50_16.factor")
   print("      * Eq. 50.15: ofire.sfpe_handbook.chapter_50.equation_50_15.height_limit")
   print("=" * 72)