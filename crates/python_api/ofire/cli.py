"""OpenFire CLI for scaffolding fire engineering projects."""

import argparse
import json
import os
import subprocess
import sys
import webbrowser
from pathlib import Path
from textwrap import dedent


def create_project_structure(project_name: str, target_dir: str) -> None:
    """Create the basic project structure."""
    project_path = Path(target_dir).resolve() / project_name
    
    # Create main project directory
    project_path.mkdir(parents=True, exist_ok=True)
    
    # Create subdirectories
    (project_path / "data").mkdir(exist_ok=True)
    (project_path / "results").mkdir(exist_ok=True)
    (project_path / "scripts").mkdir(exist_ok=True)
    
    print(f"Created project structure in: {project_path}")


def create_main_script(project_name: str, target_dir: str) -> None:
    """Create the main calculation script."""
    project_path = Path(target_dir).resolve() / project_name
    main_script = project_path / "main.py"
    
    content = dedent(f'''
        """
        {project_name} - Fire Engineering Calculations
        
        This script demonstrates basic usage of the OpenFire library
        for fire engineering calculations.
        """
        
        import ofire
        
        
        def main():
            """Main calculation function."""
            print("Welcome to {project_name}!")
            print("OpenFire library loaded successfully.")
            
            # Example: Basic fire dynamics calculation
            # Uncomment and modify the following examples as needed:
            
            # Example 1: Heat release rate calculation (CIBSE Guide E)
            # room_area = 50.0  # m²
            # room_height = 3.0  # m
            # hrr = ofire.cibse_guide_e.chapter_6.equation_6_7.heat_release_rate_flashover(
            #     room_area, room_height
            # )
            # print(f"Heat Release Rate at Flashover: {{hrr:.1f}} kW")
            
            # Example 2: Fire dynamics tools
            # Add your calculations here using the ofire library modules:
            # - ofire.br_187         (BR 187 calculations)
            # - ofire.bs9999         (BS 9999 calculations)
            # - ofire.cibse_guide_e  (CIBSE Guide E calculations)
            # - ofire.fire_dynamics_tools (General fire dynamics)
            # - ofire.pd_7974        (PD 7974 calculations)
            # - ofire.sfpe_handbook  (SFPE Handbook calculations)
            # - ofire.tr_17          (TR 17 calculations)
            # - ofire.introduction_to_fire_dynamics (Introductory calculations)
            
            print("\\nAvailable modules:")
            print("- ofire.br_187: BR 187 calculations")
            print("- ofire.bs9999: BS 9999 calculations")
            print("- ofire.cibse_guide_e: CIBSE Guide E calculations")
            print("- ofire.fire_dynamics_tools: General fire dynamics tools")
            print("- ofire.pd_7974: PD 7974 calculations")
            print("- ofire.sfpe_handbook: SFPE Handbook calculations")
            print("- ofire.tr_17: TR 17 calculations")
            print("- ofire.introduction_to_fire_dynamics: Introductory calculations")
        
        
        if __name__ == "__main__":
            main()
    ''').strip()
    
    with open(main_script, 'w') as f:
        f.write(content)
    
    print(f"Created main script: {main_script}")


def get_latest_ofire_version() -> str:
    """Get the latest version of ofire package."""
    try:
        result = subprocess.run(
            [sys.executable, '-m', 'pip', 'index', 'versions', 'ofire'],
            capture_output=True, text=True, timeout=10
        )
        if result.returncode == 0 and 'Available versions:' in result.stdout:
            # Parse the latest version from pip index output
            lines = result.stdout.split('\n')
            for line in lines:
                if 'Available versions:' in line:
                    versions = line.split('Available versions:')[1].strip()
                    if versions:
                        latest = versions.split(',')[0].strip()
                        return latest
    except (subprocess.TimeoutExpired, subprocess.SubprocessError):
        pass
    
    # Fallback to a reasonable version if we can't determine the latest
    return ">=0.1.0"


def create_requirements_file(project_name: str, target_dir: str) -> None:
    """Create a requirements.txt file."""
    project_path = Path(target_dir).resolve() / project_name
    requirements_file = project_path / "requirements.txt"
    
    ofire_version = get_latest_ofire_version()
    
    content = dedent(f'''
        # Fire engineering calculations
        ofire>={ofire_version}
        
        # Data analysis and visualization (uncomment if needed)
        # numpy>=1.20.0
        # pandas>=1.3.0
        # matplotlib>=3.4.0
        # seaborn>=0.11.0
        
        # Jupyter notebook support (uncomment if needed)
        # jupyter>=1.0.0
        # ipywidgets>=7.6.0
    ''').strip()
    
    with open(requirements_file, 'w') as f:
        f.write(content)
    
    print(f"Created requirements file: {requirements_file}")


def create_readme(project_name: str, target_dir: str) -> None:
    """Create a README.md file."""
    project_path = Path(target_dir).resolve() / project_name
    readme_file = project_path / "README.md"
    
    content = dedent(f'''
        # {project_name}
        
        Fire engineering calculations using OpenFire library.
        
        ## Setup
        
        1. Install Python dependencies:
           ```bash
           pip install -r requirements.txt
           ```
        
        2. Run the main script:
           ```bash
           python main.py
           ```
        
        ## Project Structure
        
        - `main.py`: Main calculation script
        - `data/`: Input data files
        - `results/`: Calculation results and outputs
        - `scripts/`: Additional calculation scripts
        - `requirements.txt`: Python package dependencies
        
        ## Available Calculations
        
        This project uses the OpenFire library which provides implementations for:
        
        - **BR 187**: External fire spread calculations
        - **BS 9999**: Fire safety calculations
        - **CIBSE Guide E**: Fire safety engineering calculations
        - **Fire Dynamics Tools**: General fire dynamics calculations
        - **PD 7974**: Fire safety engineering calculations
        - **SFPE Handbook**: Fire protection engineering calculations
        - **TR 17**: Fire calculations
        - **Introduction to Fire Dynamics**: Basic fire dynamics
        
        ## Documentation
        
        For detailed documentation and examples, visit:
        [OpenFire Documentation](https://emberon-tech.github.io/openfire/)
        
        ## Examples
        
        See the `main.py` file for example calculations. Uncomment and modify
        the examples as needed for your specific fire engineering analysis.
    ''').strip()
    
    with open(readme_file, 'w') as f:
        f.write(content)
    
    print(f"Created README: {readme_file}")


def create_virtual_environment(project_name: str, target_dir: str) -> Path:
    """Create a virtual environment for the project."""
    project_path = Path(target_dir).resolve() / project_name
    venv_path = project_path / ".venv"
    
    print("Creating virtual environment...")
    
    try:
        subprocess.run(
            [sys.executable, '-m', 'venv', str(venv_path)],
            check=True,
            capture_output=True,
            text=True
        )
        print(f"Virtual environment created: {venv_path}")
        return venv_path
    except subprocess.CalledProcessError as e:
        print(f"Error creating virtual environment: {e}")
        print(f"stderr: {e.stderr}")
        raise


def install_requirements(project_name: str, target_dir: str, venv_path: Path) -> None:
    """Install requirements in the virtual environment."""
    project_path = Path(target_dir).resolve() / project_name
    requirements_file = project_path / "requirements.txt"
    
    # Determine the pip executable path in the virtual environment
    if sys.platform == "win32":
        pip_exe = venv_path / "Scripts" / "pip.exe"
    else:
        pip_exe = venv_path / "bin" / "pip"
    
    print("Installing requirements...")
    
    try:
        subprocess.run(
            [str(pip_exe), 'install', '-r', str(requirements_file)],
            check=True,
            capture_output=True,
            text=True,
            cwd=str(project_path)
        )
        print("Requirements installed successfully!")
    except subprocess.CalledProcessError as e:
        print(f"Error installing requirements: {e}")
        print(f"stderr: {e.stderr}")
        raise


def create_activation_script(project_name: str, target_dir: str, venv_path: Path) -> None:
    """Create platform-specific activation scripts."""
    project_path = Path(target_dir).resolve() / project_name
    
    if sys.platform == "win32":
        # Windows batch script
        activate_script = project_path / "activate.bat"
        content = f"@echo off\ncall \"{venv_path}\\Scripts\\activate.bat\"\necho Virtual environment activated!\n"
        with open(activate_script, 'w') as f:
            f.write(content)
        print(f"Created activation script: {activate_script}")
    else:
        # Unix shell script
        activate_script = project_path / "activate.sh"
        content = f"#!/bin/bash\nsource \"{venv_path}/bin/activate\"\necho \"Virtual environment activated!\"\n"
        with open(activate_script, 'w') as f:
            f.write(content)
        # Make executable
        activate_script.chmod(0o755)
        print(f"Created activation script: {activate_script}")


def create_example_notebook(project_name: str, target_dir: str) -> None:
    """Create an example Jupyter notebook."""
    project_path = Path(target_dir).resolve() / project_name
    notebook_file = project_path / "example_calculations.ipynb"
    
    # Basic Jupyter notebook structure
    notebook_content = {
        "cells": [
            {
                "cell_type": "markdown",
                "metadata": {},
                "source": [
                    f"# {project_name} - Fire Engineering Calculations\\n",
                    "\\n",
                    "This notebook demonstrates basic usage of the OpenFire library for fire engineering calculations."
                ]
            },
            {
                "cell_type": "code",
                "execution_count": None,
                "metadata": {},
                "outputs": [],
                "source": [
                    "import ofire\\n",
                    "import numpy as np\\n",
                    "import pandas as pd\\n",
                    "import matplotlib.pyplot as plt\\n",
                    "\\n",
                    "print('OpenFire library loaded successfully!')"
                ]
            },
            {
                "cell_type": "markdown",
                "metadata": {},
                "source": [
                    "## Available Modules\\n",
                    "\\n",
                    "The OpenFire library provides the following calculation modules:\\n",
                    "\\n",
                    "- `ofire.br_187`: BR 187 calculations\\n",
                    "- `ofire.bs9999`: BS 9999 calculations\\n",
                    "- `ofire.cibse_guide_e`: CIBSE Guide E calculations\\n",
                    "- `ofire.fire_dynamics_tools`: General fire dynamics tools\\n",
                    "- `ofire.pd_7974`: PD 7974 calculations\\n",
                    "- `ofire.sfpe_handbook`: SFPE Handbook calculations\\n",
                    "- `ofire.tr_17`: TR 17 calculations\\n",
                    "- `ofire.introduction_to_fire_dynamics`: Introductory calculations"
                ]
            },
            {
                "cell_type": "markdown",
                "metadata": {},
                "source": [
                    "## Example Calculation\\n",
                    "\\n",
                    "Add your fire engineering calculations in the cells below."
                ]
            },
            {
                "cell_type": "code",
                "execution_count": None,
                "metadata": {},
                "outputs": [],
                "source": [
                    "# Example: Uncomment and run a calculation\\n",
                    "# room_area = 50.0  # m²\\n",
                    "# room_height = 3.0  # m\\n",
                    "# hrr = ofire.cibse_guide_e.chapter_6.equation_6_7.heat_release_rate_flashover(room_area, room_height)\\n",
                    "# print(f'Heat Release Rate at Flashover: {hrr:.1f} kW')"
                ]
            }
        ],
        "metadata": {
            "kernelspec": {
                "display_name": "Python 3",
                "language": "python",
                "name": "python3"
            },
            "language_info": {
                "name": "python",
                "version": "3.8.0"
            }
        },
        "nbformat": 4,
        "nbformat_minor": 4
    }
    
    import json
    with open(notebook_file, 'w') as f:
        json.dump(notebook_content, f, indent=2)
    
    print(f"Created example notebook: {notebook_file}")


def open_docs(args):
    """Open the OpenFire documentation in the default browser."""
    docs_url = "https://emberon-tech.github.io/openfire/"
    
    print(f"Opening documentation: {docs_url}")
    
    try:
        webbrowser.open(docs_url)
        print("Documentation opened in your default browser!")
    except Exception as e:
        print(f"Error opening browser: {e}")
        print(f"You can manually visit: {docs_url}")


def scaffold_project(args):
    """Scaffold a new OpenFire project."""
    project_name = args.name
    target_dir = args.directory
    
    print(f"Creating OpenFire project: {project_name}")
    print(f"Target directory: {target_dir}")
    
    try:
        # Create project structure
        create_project_structure(project_name, target_dir)
        
        # Create files
        create_main_script(project_name, target_dir)
        create_requirements_file(project_name, target_dir)
        create_readme(project_name, target_dir)
        
        if args.notebook:
            create_example_notebook(project_name, target_dir)
        
        # Create virtual environment
        venv_path = create_virtual_environment(project_name, target_dir)
        
        # Install requirements in the virtual environment
        install_requirements(project_name, target_dir, venv_path)
        
        # Create activation script
        create_activation_script(project_name, target_dir, venv_path)
        
        project_path = Path(target_dir).resolve() / project_name
        
        print("\n" + "="*50)
        print("Project created successfully!")
        print("="*50)
        print(f"\nProject location: {project_path}")
        print("\nNext steps:")
        print(f"1. cd {project_path}")
        
        if sys.platform == "win32":
            print("2. activate.bat")
            print("3. python main.py")
        else:
            print("2. source activate.sh  # or: source .venv/bin/activate")
            print("3. python main.py")
        
        if args.notebook:
            print("4. jupyter notebook example_calculations.ipynb")
        
        print("\nVirtual environment created with all dependencies installed!")
        print("For documentation, visit: https://emberon-tech.github.io/openfire/")
        
    except Exception as e:
        print(f"\nError creating project: {e}")
        sys.exit(1)


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        description="OpenFire CLI - Tools for fire engineering projects",
        prog="ofire"
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # New project command
    new_parser = subparsers.add_parser(
        'new', 
        help='Create a new OpenFire project'
    )
    new_parser.add_argument(
        'name',
        help='Name of the new project'
    )
    new_parser.add_argument(
        '-d', '--directory',
        default='.',
        help='Directory to create the project in (default: current directory)'
    )
    new_parser.add_argument(
        '--notebook',
        action='store_true',
        help='Include an example Jupyter notebook'
    )
    new_parser.set_defaults(func=scaffold_project)
    
    # Docs command
    docs_parser = subparsers.add_parser(
        'docs',
        help='Open OpenFire documentation in browser'
    )
    docs_parser.set_defaults(func=open_docs)
    
    # Version command
    version_parser = subparsers.add_parser(
        'version',
        help='Show OpenFire version'
    )
    version_parser.set_defaults(func=lambda args: print("OpenFire CLI v1.0.0"))
    
    # Parse arguments
    args = parser.parse_args()
    
    if args.command is None:
        parser.print_help()
        return
    
    # Execute the command
    args.func(args)


if __name__ == '__main__':
    main()