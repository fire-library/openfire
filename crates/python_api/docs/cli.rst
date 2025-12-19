Command Line Interface
======================

The OpenFire CLI provides a convenient command-line interface for creating and managing fire engineering projects. It allows you to scaffold new projects, run applications, and access documentation directly from your terminal.

Installation
------------

The CLI is installed automatically when you install the OpenFire package:

.. code-block:: bash

   pip install ofire

After installation, the ``ofire`` command will be available in your terminal.

Basic Usage
-----------

The general syntax for the OpenFire CLI is:

.. code-block:: bash

   ofire <command> [options]

To see all available commands and options:

.. code-block:: bash

   ofire --help

Commands
--------

new
~~~

Create a new OpenFire fire engineering project with all necessary files and setup.

**Syntax:**

.. code-block:: bash

   ofire new <project_name> [options]

**Arguments:**

* ``project_name`` - Name of the new project (required)

**Options:**

* ``-d, --directory <path>`` - Directory to create the project in (default: current directory)

**Example:**

.. code-block:: bash

   # Create a new project in the current directory
   ofire new my_fire_project

**What gets created:**

When you run ``ofire new``, the following structure is created:

.. code-block:: text

   project_name/
   ├── main.py          # Main Streamlit application
   ├── requirements.txt # Python dependencies
   ├── README.md        # Project documentation
   ├── AGENTS.md        # AI agent guidance
   ├── CLAUDE.md        # Claude AI instructions
   ├── .venv/           # Virtual environment (auto-created)
   ├── activate.sh      # Virtual environment activation (Unix)
   └── activate.bat     # Virtual environment activation (Windows)

The project includes:

* A complete virtual environment with all dependencies installed
* A web-based fire engineering application using Streamlit
* Pre-configured settings for AI coding assistants
* Example calculations and documentation

run
~~~

Run a fire engineering application using Streamlit.

**Syntax:**

.. code-block:: bash

   ofire run [target]

**Arguments:**

* ``target`` - File path or URL to run (optional, default: main.py)

**Examples:**

.. code-block:: bash

   # Run the default main.py application
   ofire run

   # Run a specific Python file
   ofire run my_calculations.py

   # Run an application from a URL
   ofire run https://example.com/fire_app.py

**Note:** This command starts a Streamlit web server. The application will be accessible in your browser, typically at ``http://localhost:8501``. Press ``Ctrl+C`` to stop the server.

docs
~~~~

Open the OpenFire documentation in your default web browser.

**Syntax:**

.. code-block:: bash

   ofire docs

**Example:**

.. code-block:: bash

   ofire docs

This opens the comprehensive OpenFire documentation at `https://emberon-tech.github.io/openfire/ <https://emberon-tech.github.io/openfire/>`_ in your default browser.

version
~~~~~~~

Display the current version of the OpenFire CLI.

**Syntax:**

.. code-block:: bash

   ofire version

**Example:**

.. code-block:: bash

   ofire version

Workflows
---------

Creating and Running a New Project
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Here's a typical workflow for creating and running a new fire engineering project:

1. **Create the project:**

   .. code-block:: bash

      ofire new building_analysis

2. **Navigate to the project:**

   .. code-block:: bash

      cd building_analysis

3. **Activate the virtual environment:**

   .. code-block:: bash

      # On Unix/Linux/macOS
      source activate.sh

      # On Windows
      activate.bat

4. **Run the application:**

   .. code-block:: bash

      ofire run

5. **Open your browser** to the URL shown in the terminal (usually http://localhost:8501)

Development Workflow
~~~~~~~~~~~~~~~~~~~~

For ongoing development work:

1. **Activate the virtual environment** (if not already active):

   .. code-block:: bash

      source .venv/bin/activate  # Unix/Linux/macOS
      # or
      .venv\Scripts\activate.bat  # Windows

2. **Edit your calculations** in ``main.py`` or create new files

3. **Run your application** to test changes:

   .. code-block:: bash

      ofire run

Virtual Environment Management
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The OpenFire CLI automatically creates and manages virtual environments for new projects:

* **Automatic creation**: Virtual environment is created during ``ofire new``
* **Dependency installation**: All required packages are installed automatically
* **Activation scripts**: Platform-specific activation scripts are generated
* **Isolation**: Each project has its own isolated environment

Integration with AI Assistants
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Projects created with ``ofire new`` include configuration files for AI coding assistants:

* **CLAUDE.md**: Instructions for Claude AI to always read the agents guide
* **AGENTS.md**: Comprehensive guidance for AI assistants working on fire engineering projects

These files help AI assistants understand the project context and provide more accurate assistance with fire engineering calculations.

Troubleshooting
---------------

Common Issues
~~~~~~~~~~~~~

**Command not found: ofire**

* Ensure OpenFire is installed: ``pip install ofire``
* Check that your Python scripts directory is in your PATH
* Try reinstalling: ``pip uninstall ofire && pip install ofire``

**Virtual environment issues**

* Ensure Python venv module is available: ``python -m venv --help``
* On some systems, you may need ``python3`` instead of ``python``
* Check permissions in the target directory

**Streamlit not starting**

* Ensure all dependencies are installed: ``pip install -r requirements.txt``
* Check if port 8501 is already in use
* Try specifying a different port: ``streamlit run main.py --server.port 8502``

**Permission errors on Unix systems**

* The activation script should be executable: ``chmod +x activate.sh``
* Check directory permissions for the target location

Getting Help
~~~~~~~~~~~~

* Use ``ofire --help`` for general help
* Use ``ofire <command> --help`` for command-specific help
* Visit the documentation: ``ofire docs``
* Check the project README.md for project-specific information

Examples
--------

Quick Calculation Script
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   # Create a simple project for quick calculations
   ofire new quick_calc

   # cd into the project directory, activate the virtual environment, and run the app
   cd quick_calc
   source activate.sh
   ofire run

Running External Applications
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   # Run an app from GitHub
   ofire run https://raw.githubusercontent.com/example/fire-calc/main/app.py

   # Run a local file from anywhere on your system
   ofire run /path/to/my/fire_analysis.py