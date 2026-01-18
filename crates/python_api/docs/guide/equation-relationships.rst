Equation Relationships
======================

This page tracks relationships between equations across different fire engineering
documents in the OpenFire library. It helps identify which equations are identical
or similar across different documents, enabling better understanding of the connections
between various fire engineering standards and references.

Understanding Equation Relationships
------------------------------------

**Identical Equations**: Equations that are mathematically identical across different documents.
These represent the same fundamental relationships that appear in multiple standards or references.

**Similar Equations**: Equations that are similar but have variations such as different constants,
additional terms, or different formulations for similar physical phenomena.

This information is valuable for:

- Understanding commonalities across fire engineering standards
- Identifying which equations from different documents can be used interchangeably
- Learning about subtle differences in formulations and when to use each variant
- Cross-referencing equations when working with multiple documents

Identical Equations
-------------------

.. note::
   
   Equations listed in each group are mathematically identical and can be used interchangeably.

.. _identical-equations:

*Groups will be created here as identical equations are identified across documents.*

Similar Equations
-----------------

.. note::
   
   Equations in each group are either similar in form or calculate the same parameter.
   Pay attention to the differences described for each group.

.. _similar-equations:

Heat Release Rate at Flashover
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

*Group 1: Heat Release Rate Calculations for Flashover Conditions*

These equations calculate heat release rate at flashover but use different formulations and constants:

- :func:`ofire.cibse_guide_e.chapter_6.equation_6_7.heat_release_rate_flashover` - CIBSE Guide E, Chapter 6, Equation 6.7
- :func:`ofire.pd_7974.part_1.section_8.equation_28.q_fo` - PD 7974, Part 1, Section 8, Equation 28 (Thomas' method)
- :func:`ofire.pd_7974.part_1.section_8.equation_29.q_fo` - PD 7974, Part 1, Section 8, Equation 29 (McCaffrey's method)

Visibility
~~~~~~~~~~

*Group 2: Visibility Calculations Through Smoke*

These equations calculate visibility distance through smoke but use different formulations:

- :func:`ofire.cibse_guide_e.chapter_10.equation_10_7.visibility` - CIBSE Guide E, Chapter 10, Equation 10.7
- :func:`ofire.fire_dynamics_tools.chapter_18.equation_18_1.visibility` - Fire Dynamics Tools, Chapter 18, Equation 18.1
- :func:`ofire.sfpe_handbook.chapter_50.equation_50_19.visibility` - SFPE Handbook, Chapter 50, Equation 50-19 (from mass concentration)
- :func:`ofire.sfpe_handbook.chapter_50.equation_50_20.visibility` - SFPE Handbook, Chapter 50, Equation 50-20 (from percent obscuration)

Fractional Effective Dose
~~~~~~~~~~~~~~~~~~~~~~~~~

*Group 3: Fractional Effective Dose Calculations*

These equations calculate the fractional effective dose (FED), representing accumulated toxic exposure over time as a fraction of the lethal dose, but use different formulations:

- :func:`ofire.sfpe_handbook.chapter_50.equation_50_18.fed` - SFPE Handbook, Chapter 50, Equation 50-18
- :func:`ofire.cibse_guide_e.chapter_10.equation_10_8.fractional_effective_dose` - CIBSE Guide E, Chapter 10, Equation 10-8

Contributing to Equation Relationships
---------------------------------------

If you discover equation relationships that are not documented here while working with the OpenFire library, we encourage you to get in touch with us or, even better, submit a Pull Request to help improve this documentation. Your contributions help the entire fire engineering community understand the connections between different standards and calculation methods.

Cross-References and Navigation
-------------------------------

- For detailed information about specific equations, see the :doc:`../api/index`
- For practical examples, see the :doc:`../examples/index`
- For getting started with the library, see the :doc:`getting-started`