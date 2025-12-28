OpenFire Documentation
======================

Fire safety engineering tools implemented in Rust with Python bindings.

Join Our Community
-------------------

🔥 **Join the OpenFire Discord Community!** 🔥

Connect with fire safety engineers, ask questions, stay up-to-date with the latest developments, and participate in discussions about OpenFire and fire engineering in general.

`Join our Discord server <https://discord.gg/TeBRS5ew3y>`_

- Get help with OpenFire usage and fire engineering calculations
- Discuss new features and improvements
- Share your projects and use cases
- Connect with other fire safety professionals
- Get announcements about new releases and updates

Overview
--------

OpenFire provides a comprehensive set of tools for fire safety engineering calculations and analysis. Built in Rust for performance and safety, with Python bindings for ease of use.

Key Features
------------

- **High Performance**: Implemented in Rust for maximum speed and memory safety
- **Python Integration**: Easy-to-use Python API for rapid development  
- **Building Blocks**: Tools for creating all types of fire safety engineering calculations

Quick Start
-----------

.. code-block:: python

   import ofire

   # Calculate heat release rate of a fuel controlled fire (PD 7974 Part 1)
   a_t = 20.0  # Total internal surface area (m²)
   a_v = 4.0    # Area of ventilation opening (m²)  
   h_v = 2.1    # Height of ventilation opening (m)
   q_fo = ofire.pd_7974.part_1.section_8.equation_28.q_fo(a_t, a_v, h_v)
   print(f"Heat release rate of fuel controlled fire: {q_fo:.0f} kW")

Project Structure
-----------------

- **Rust Core**: High-performance fire engineering calculations
- **Python API**: User-friendly Python interface
- **Documentation**: Comprehensive guides and API reference

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   installation
   guide/index
   api/index
   definitions
   coverage


Contributing
------------

See our `GitHub repository <https://github.com/fire-library/openfire>`_ for contribution guidelines.

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`