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
- :func:`ofire.pd_7974.part_1.section_8.equation_28.function_name` - PD 7974, Part 1, Section 8, Equation 28
- :func:`ofire.pd_7974.part_1.section_8.equation_29.function_name` - PD 7974, Part 1, Section 8, Equation 29

**Key Differences**: The CIBSE Guide E version uses different constants and may include additional factors compared to the PD 7974 formulations. PD 7974 provides two variations (equations 28 and 29) for different scenarios or assumptions.

**When to Use Each**:

- Use CIBSE Guide E version for UK-specific applications and when following CIBSE methodologies
- Use PD 7974 equations for European applications and when following Eurocode methodologies
- Consult the specific requirements of your local fire safety codes to determine the appropriate equation

Adding New Equation Relationships
---------------------------------

When you identify new equation relationships while working with the library, please follow this process:

Identifying Identical Equations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. **Compare mathematical formulations** - Check if equations have exactly the same mathematical structure
2. **Verify variable definitions** - Ensure variables represent the same physical quantities
3. **Check units** - Confirm that units are consistent across equations
4. **Test with sample calculations** - Verify equations produce identical results

Adding Identical Equations to Documentation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. **Choose appropriate group** - Add to existing group if related, or create new group
2. **Add descriptive heading** - Use clear, descriptive group names
3. **Link to actual functions** - Use Sphinx ``:func:`` directive to link to API documentation
4. **Include source references** - Mention the document, chapter, and equation number

Example format::

    Heat Release Rate from Fuel Properties
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    *Group X: Heat release rate calculation from fuel mass and heat of combustion*

    These equations calculate heat release rate using fuel consumption rate and heat of combustion:

    - :func:`ofire.document1.chapter_x.equation_y.function_name` - Document 1, Chapter X, Equation Y
    - :func:`ofire.document2.chapter_a.equation_b.function_name` - Document 2, Chapter A, Equation B

Adding Similar Equations to Documentation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. **Create descriptive group** - Focus on the physical phenomenon being calculated
2. **Document key differences** - Clearly explain how the equations differ
3. **Provide usage guidance** - Explain when to use each variation
4. **Link to functions** - Use appropriate Sphinx directives

Example format::

    Smoke Layer Temperature Calculations
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    *Group X: Smoke layer temperature with different assumptions*

    These equations calculate smoke layer temperature but use different assumptions about heat loss:

    - :func:`ofire.document1.chapter_x.equation_y.function_name` - Document 1, Chapter X, Equation Y  
    - :func:`ofire.document2.chapter_a.equation_b.function_name` - Document 2, Chapter A, Equation B

    **Key Differences**: Document 1 assumes adiabatic conditions while Document 2 includes heat loss to boundaries.

    **When to Use Each**:

    - Use Document 1 version for conservative estimates in early design phases
    - Use Document 2 version for detailed analyses where heat loss is significant

Maintenance Guidelines
----------------------

This page should be updated whenever:

1. **New equations are added** to the library that relate to existing equations
2. **Relationships are discovered** between existing equations  
3. **Function names change** in the API (links will need updating)
4. **New insights emerge** about when to use different equation variants

The documentation provides compile-time checking for function references, so broken links will be caught during the documentation build process.

Cross-References and Navigation
-------------------------------

- For detailed information about specific equations, see the :doc:`../api/index`
- For practical examples, see the :doc:`examples`  
- For getting started with the library, see the :doc:`getting-started`

.. note::

   This page is manually maintained. As you work with equations and identify relationships, please contribute by adding them to this documentation. This helps the entire fire engineering community understand the connections between different standards and calculation methods.