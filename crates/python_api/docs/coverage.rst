Test Coverage
=============

.. note::
   **View Current Coverage Report**: `Detailed Coverage Report <_static/coverage/html/index.html>`_

Testing Philosophy
------------------

OpenFire takes a rigorous approach to code quality and testing, particularly for fire safety engineering calculations where accuracy is paramount.

**100% Coverage Requirement**

All calculation functions that perform fire safety engineering computations must have 100% test coverage. This includes mathematical equation implementations, fire dynamics calculations, heat transfer calculations, and smoke movement algorithms.

**Validation Standards**

Test cases are derived from published examples in fire safety standards such as BS 9999 and PD 7974, worked examples from engineering handbooks like CIBSE Guide E and SFPE Handbook, and peer-reviewed research papers. Our tests cover boundary conditions, standard engineering scenarios, edge cases, and error handling.

**Quality Assurance**

High test coverage ensures reliability in calculations, maintainability of code changes, and confidence for engineers using these tools in critical safety applications. Some non-critical code such as equation functions that return string representations of the mathematical equations may have lower coverage requirements and are marked with ``#[cfg(not(coverage))]``.