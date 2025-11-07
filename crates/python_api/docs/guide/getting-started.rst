Getting Started
===============

This guide will help you get started with OpenFire for fire safety engineering calculations.

Library Philosophy
------------------

OpenFire is designed as a comprehensive toolkit of fundamental fire engineering building blocks rather than a collection of prescriptive, all-in-one solutions. This deliberate architectural choice reflects the inherent complexity and diversity of fire safety engineering applications. Rather than providing monolithic functions that attempt to solve entire problems with a single equation, OpenFire offers carefully tested, reliable components that can be composed and combined to address the specific requirements of your unique engineering challenge.

The fire safety engineering field encompasses an extraordinary range of scenarios, from simple stair width calculations to complex zone models, each with distinct regulatory frameworks, design criteria, and performance objectives. Even within specific categories like zone modeling, there are multiple established equations and methodologies available to achieve the same outcome, and we don't force any particular approach on you. By providing granular, well-documented functions that implement individual equations and calculation methods from established standards like BR 187, PD 7974, and others, OpenFire empowers engineers to construct solutions that precisely match their project requirements without being constrained by assumptions embedded in higher-level abstractions.

This modular approach ensures that you maintain complete control over your calculation workflows, can easily audit and verify each step of your analysis, and can adapt the methodology as standards evolve or project requirements change. The library serves as your reliable foundation of validated calculations, allowing you to focus on the engineering judgment and problem-solving that defines expert fire safety practice, rather than reimplementing fundamental equations or worrying about calculation accuracy.

Basic Usage
-----------

After installation, you can import and use OpenFire in your Python projects. Here's a simple example calculating thermal radiation exposure:

.. code-block:: python

   import ofire

   # Calculate thermal radiation from a radiating surface
   # Step 1: Source radiation intensity
   I_s = ofire.br_187.appendix_a.equation_a1.radiation_intensity(
       sigma=5.67e-11,  # Stefan-Boltzmann constant (kW/m²K⁴)
       emissivity=0.9,  # Surface emissivity
       temperature=1273 # Surface temperature (K)
   )

   # Step 2: Dimensionless parameters
   X = ofire.br_187.appendix_a.equation_a3.x(w=4.0, s=12.0)
   Y = ofire.br_187.appendix_a.equation_a3.y(h=2.5, s=12.0)

   # Step 3: View factor and received intensity
   phi = ofire.br_187.appendix_a.equation_a3.phi(X, Y, additive=True)
   I_R = ofire.br_187.appendix_a.equation_a2.radiation_intensity_at_receiver(phi, I_s)

   print(f"Received intensity: {I_R:.1f} kW/m²")

Next Steps
----------

- Check out the :doc:`examples` for practical use cases
- Browse the :doc:`../api/index` for detailed function documentation
- Visit our `GitHub repository <https://github.com/fire-library/openfire>`_ for the latest updates