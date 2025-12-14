"""OpenFire project creation and management logic."""

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
    """Create the main fire engineering application script."""
    project_path = Path(target_dir).resolve() / project_name
    main_script = project_path / "main.py"
    
    content = dedent(f'''
        """
        {project_name} - Fire Engineering Tool
        
        A web application for fire engineering calculations
        using the OpenFire library.
        """
        
        import streamlit as st
        import ofire
        
        
        def main():
            """Main Streamlit application."""
            st.set_page_config(
                page_title="{project_name}",
                page_icon="🔥",
                layout="wide",
                initial_sidebar_state="expanded"
            )
            
            st.title("🔥 {project_name}")
            st.markdown("Fire Engineering Calculations using OpenFire")
            
            # Sidebar for navigation
            st.sidebar.title("Navigation")
            page = st.sidebar.selectbox(
                "Select a calculation:",
                ["Welcome", "Heat Release Rate", "Smoke Layer Analysis", "About"]
            )
            
            if page == "Welcome":
                welcome_page()
            elif page == "Heat Release Rate":
                heat_release_rate_page()
            elif page == "Smoke Layer Analysis":
                smoke_filling_page()
            elif page == "About":
                about_page()
        
        
        def welcome_page():
            """Display the welcome page."""
            st.header("Welcome to {project_name}")
            
            st.markdown("""
            This fire engineering tool provides calculations for:
            
            - **Heat Release Rate**: Calculate HRR at flashover using CIBSE Guide E
            - **Smoke Layer Analysis**: Calculate smoke layer interface height and properties using Fire Dynamics Tools
            - **Custom Calculations**: Add your own fire engineering calculations
            
            Select a calculation from the sidebar to get started.
            """)
            
            st.info("💡 This tool is built using the OpenFire library for fire engineering calculations.")
            
            # Display available modules
            with st.expander("Available OpenFire Modules"):
                st.markdown("""
                - `ofire.br_187`: BR 187 calculations
                - `ofire.bs9999`: BS 9999 calculations  
                - `ofire.cibse_guide_e`: CIBSE Guide E calculations
                - `ofire.fire_dynamics_tools`: General fire dynamics tools
                - `ofire.pd_7974`: PD 7974 calculations
                - `ofire.sfpe_handbook`: SFPE Handbook calculations
                - `ofire.tr_17`: TR 17 calculations
                - `ofire.introduction_to_fire_dynamics`: Introductory calculations
                """)
        
        
        def heat_release_rate_page():
            """Heat release rate calculation page."""
            st.header("Heat Release Rate at Flashover")
            st.markdown("Calculate the heat release rate at flashover using CIBSE Guide E, Chapter 6.")
            
            col1, col2 = st.columns([1, 1])
            
            with col1:
                st.subheader("Input Parameters")
                room_area = st.number_input(
                    "Room Floor Area (m²)",
                    min_value=1.0,
                    max_value=10000.0,
                    value=50.0,
                    step=1.0,
                    help="Floor area of the room in square meters"
                )
                
                room_height = st.number_input(
                    "Room Height (m)",
                    min_value=1.0,
                    max_value=50.0,
                    value=3.0,
                    step=0.1,
                    help="Height of the room in meters"
                )
                
                if st.button("Calculate HRR", type="primary"):
                    try:
                        # Example calculation - replace with actual ofire function when available
                        # hrr = ofire.cibse_guide_e.chapter_6.equation_6_7.heat_release_rate_flashover(
                        #     room_area, room_height
                        # )
                        
                        # Placeholder calculation for demonstration
                        hrr = room_area * room_height * 250  # Simplified example
                        
                        st.session_state.hrr_result = hrr
                        st.success("Calculation completed!")
                        
                    except Exception as e:
                        st.error(f"Calculation error: {{e}}")
            
            with col2:
                st.subheader("Results")
                if hasattr(st.session_state, 'hrr_result'):
                    st.metric(
                        "Heat Release Rate at Flashover",
                        f"{{st.session_state.hrr_result:,.0f}} kW",
                        help="Heat release rate at the onset of flashover"
                    )
                    
                    # Additional information
                    st.info("""
                    **Note**: This calculation is based on CIBSE Guide E methodology.
                    Replace the placeholder calculation with the actual OpenFire function
                    when implementing your specific requirements.
                    """)
                else:
                    st.info("Enter room parameters and click 'Calculate HRR' to see results.")
        
        
        def smoke_filling_page():
            """Smoke layer analysis calculation page."""
            st.header("Smoke Layer Analysis")
            st.markdown("Calculate smoke layer interface height and properties using OpenFire fire dynamics tools.")
            
            col1, col2 = st.columns([1, 1])
            
            with col1:
                st.subheader("Room Parameters")
                room_length = st.number_input(
                    "Room Length (m)",
                    min_value=1.0,
                    max_value=100.0,
                    value=10.0,
                    step=0.1,
                    help="Length of the room in meters"
                )
                
                room_width = st.number_input(
                    "Room Width (m)",
                    min_value=1.0,
                    max_value=100.0,
                    value=8.0,
                    step=0.1,
                    help="Width of the room in meters"
                )
                
                room_height = st.number_input(
                    "Room Height (m)",
                    min_value=1.0,
                    max_value=50.0,
                    value=3.0,
                    step=0.1,
                    help="Height of the room in meters"
                )
                
                st.subheader("Fire Parameters")
                heat_release_rate = st.number_input(
                    "Heat Release Rate (kW)",
                    min_value=10.0,
                    max_value=50000.0,
                    value=1000.0,
                    step=10.0,
                    help="Heat release rate of the fire in kilowatts"
                )
                
                time_after_ignition = st.number_input(
                    "Time After Ignition (s)",
                    min_value=1.0,
                    max_value=3600.0,
                    value=90.0,
                    step=1.0,
                    help="Time elapsed since ignition in seconds"
                )
                
                hot_gas_temp = st.number_input(
                    "Hot Gas Temperature (K)",
                    min_value=300.0,
                    max_value=1500.0,
                    value=500.0,
                    step=10.0,
                    help="Temperature of the hot gas layer in Kelvin"
                )
                
                if st.button("Calculate Smoke Layer Properties", type="primary"):
                    try:
                        # Calculate room floor area
                        floor_area = room_length * room_width
                        
                        # Calculate hot gas density using OpenFire
                        hot_gas_density = ofire.fire_dynamics_tools.chapter_2.equation_2_13.density_hot_gas_layer(
                            hot_gas_temp
                        )
                        
                        # Calculate entrainment coefficient using simplified method
                        k_coefficient = ofire.fire_dynamics_tools.chapter_2.equation_2_12.k_constant_smoke_layer_height_post_substitution(
                            hot_gas_density
                        )
                        
                        # Calculate smoke layer interface height using Yamana-Tanaka correlation
                        interface_height = ofire.fire_dynamics_tools.chapter_2.equation_2_10.height_smoke_layer_interface_natural_ventilation(
                            k_coefficient, heat_release_rate, time_after_ignition, floor_area, room_height
                        )
                        
                        # Calculate derived properties
                        smoke_layer_depth = room_height - interface_height
                        smoke_volume = floor_area * max(0, smoke_layer_depth)
                        percent_filled = (max(0, smoke_layer_depth) / room_height) * 100
                        
                        # Store results in session state
                        st.session_state.smoke_results = {{
                            'hot_gas_density': hot_gas_density,
                            'k_coefficient': k_coefficient,
                            'interface_height': interface_height,
                            'smoke_layer_depth': smoke_layer_depth,
                            'smoke_volume': smoke_volume,
                            'percent_filled': percent_filled,
                            'floor_area': floor_area
                        }}
                        
                        st.success("Smoke layer analysis completed!")
                        
                    except Exception as e:
                        st.error(f"Calculation error: {{e}}")
                        st.info("Note: Ensure all parameters are within reasonable ranges for fire engineering calculations.")
            
            with col2:
                st.subheader("Results")
                if hasattr(st.session_state, 'smoke_results'):
                    results = st.session_state.smoke_results
                    
                    # Primary results
                    st.metric(
                        "Smoke Layer Interface Height",
                        f"{{results['interface_height']:.2f}} m",
                        help="Height of the interface between clear air below and smoke layer above"
                    )
                    
                    st.metric(
                        "Smoke Layer Depth",
                        f"{{results['smoke_layer_depth']:.2f}} m",
                        help="Thickness of the smoke layer from interface to ceiling"
                    )
                    
                    st.metric(
                        "Room Smoke-Filled",
                        f"{{results['percent_filled']:.1f}}%",
                        help="Percentage of room height filled with smoke"
                    )
                    
                    # Additional properties in expandable section
                    with st.expander("Advanced Properties"):
                        col3, col4 = st.columns([1, 1])
                        
                        with col3:
                            st.metric("Hot Gas Density", f"{{results['hot_gas_density']:.3f}} kg/m³")
                            st.metric("Floor Area", f"{{results['floor_area']:.1f}} m²")
                        
                        with col4:
                            st.metric("Entrainment Coefficient", f"{{results['k_coefficient']:.6f}}")
                            st.metric("Smoke Volume", f"{{results['smoke_volume']:.1f}} m³")
                    
                    
                    # Methodology information
                    st.info("""
                    **Calculation Method**: 
                    - Yamana-Tanaka correlation for smoke layer interface height
                    - Natural ventilation conditions assumed
                    - Based on Fire Dynamics Tools, Chapter 2
                    """)
                    
                else:
                    st.info("Enter room and fire parameters, then click 'Calculate Smoke Layer Properties' to see results.")
                    
                    st.markdown("""
                    **This calculation determines:**
                    - Smoke layer interface height
                    - Smoke layer depth and volume
                    - Hot gas layer properties
                    - Safety assessment for evacuation
                    
                    **Based on OpenFire implementation of:**
                    - Fire Dynamics Tools Chapter 2 equations
                    - Yamana-Tanaka correlation for transient conditions
                    """)
        
        
        def about_page():
            """Display information about the application."""
            st.header("About {project_name}")
            
            st.markdown("""
            This fire engineering tool was created using:
            
            - **OpenFire Library**: Comprehensive fire engineering calculations
            - **Python**: Programming language for scientific computing
            - **Web Interface**: Modern interactive calculation interface
            
            ### How to Extend This Tool
            
            1. Add new calculation pages by creating functions like `your_calculation_page()`
            2. Add the new page to the sidebar navigation
            3. Implement calculations using OpenFire library functions
            4. Use web interface components for interactive inputs and results display
            
            ### OpenFire Documentation
            
            Visit [OpenFire Documentation](https://emberon-tech.github.io/openfire/) for detailed
            information about available calculations and usage examples.
            """)
            
            st.markdown("---")
            st.markdown("Built with ❤️ using OpenFire")
        
        
        if __name__ == "__main__":
            main()
    ''').strip()
    
    with open(main_script, 'w') as f:
        f.write(content)
    
    print(f"Created fire engineering application: {main_script}")


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
        streamlit>=1.28.0
        
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


def create_agents_guide(project_name: str, target_dir: str) -> None:
    """Create an AGENTS.md file with guidance for AI coding agents."""
    project_path = Path(target_dir).resolve() / project_name
    agents_file = project_path / "AGENTS.md"
    
    content = dedent('''
        # AI Agent Guide for Fire Engineering Projects
        
        This document provides guidance for AI coding agents working on fire engineering projects with users who have limited programming experience.
        
        ## Project Overview
        
        This is a **Python project** that uses **Streamlit** for the user interface. It is designed to create fire engineering calculation tools for fire safety professionals.
        
        ## Key Information for AI Agents
        
        ### Technology Stack
        - **Language**: Python 3.8+
        - **UI Framework**: Streamlit (web-based interface)
        - **Fire Engineering Library**: OpenFire (ofire)
        - **Environment**: Virtual environment (.venv) in project root
        - **Package Manager**: pip with requirements.txt
        
        ### Project Structure
        ```
        project/
        ├── .venv/              # Virtual environment (contains ofire library)
        ├── main.py             # Main Streamlit application
        ├── data/               # Input data files
        ├── results/            # Output files
        ├── scripts/            # Additional Python scripts
        ├── requirements.txt    # Python dependencies
        ├── README.md          # User documentation
        └── AGENTS.md          # This file
        ```
        
        ### OpenFire Library Usage
        
        **IMPORTANT**: For ALL fire engineering calculations, use the **ofire library** when possible.
        
        The ofire library is already installed in the virtual environment and provides:
        
        #### Available Modules
        - `ofire.br_187` - BR 187 external fire spread calculations
        - `ofire.bs9999` - BS 9999 fire safety calculations  
        - `ofire.cibse_guide_e` - CIBSE Guide E fire safety engineering
        - `ofire.fire_dynamics_tools` - General fire dynamics calculations
        - `ofire.pd_7974` - PD 7974 fire safety engineering
        - `ofire.sfpe_handbook` - SFPE Handbook calculations
        - `ofire.tr_17` - TR 17 fire calculations
        - `ofire.introduction_to_fire_dynamics` - Basic fire dynamics
        
        #### Example Usage
        ```python
        import ofire
        
        # Heat release rate calculation
        hrr = ofire.cibse_guide_e.chapter_6.equation_6_7.heat_release_rate_flashover(
            room_area=50.0, 
            room_height=3.0
        )
        
        # Smoke layer interface height
        interface_height = ofire.fire_dynamics_tools.chapter_2.equation_2_10.height_smoke_layer_interface_natural_ventilation(
            k=0.12, q=1000.0, t=90.0, a_c=250.0, h_c=4.5
        )
        ```
        
        ### Streamlit Application Guidelines
        
        #### Basic Structure
        All fire engineering tools should follow this pattern:
        
        ```python
        import streamlit as st
        import ofire
        
        def main():
            st.set_page_config(
                page_title="Fire Engineering Tool",
                page_icon="🔥",
                layout="wide"
            )
            
            st.title("🔥 Your Fire Engineering Tool")
            
            # Sidebar navigation for multiple calculation pages
            st.sidebar.title("Navigation")
            page = st.sidebar.selectbox("Select calculation:", ["Page 1", "Page 2"])
            
            if page == "Page 1":
                calculation_page_1()
            elif page == "Page 2":
                calculation_page_2()
        
        def calculation_page_1():
            st.header("Calculation Name")
            
            col1, col2 = st.columns([1, 1])
            
            with col1:
                st.subheader("Input Parameters")
                # Add input widgets here
                param1 = st.number_input("Parameter 1", value=10.0)
                
                if st.button("Calculate", type="primary"):
                    # Use ofire library for calculations
                    result = ofire.module.function(param1)
                    st.session_state.result = result
            
            with col2:
                st.subheader("Results")
                if hasattr(st.session_state, 'result'):
                    st.metric("Result", f"{st.session_state.result:.2f}")
        
        if __name__ == "__main__":
            main()
        ```
        
        #### UI Best Practices
        - Use `st.columns()` for side-by-side layout (inputs left, results right)
        - Use `st.number_input()` for numerical parameters with appropriate min/max values
        - Use `st.metric()` to display calculation results
        - Use `st.session_state` to store calculation results between interactions
        - Include help text for parameters: `help="Description of parameter"`
        - Use `st.info()`, `st.warning()`, `st.error()` for informational messages
        
        ### Running the Application
        
        Users will run the application with:
        ```bash
        ofire run
        ```
        
        This automatically activates the virtual environment and runs: `streamlit run main.py`
        
        ### Common Fire Engineering Calculations
        
        When users ask for fire engineering calculations, prioritize these areas:
        
        1. **Heat Release Rate**: Use CIBSE Guide E or SFPE methods
        2. **Smoke Layer Analysis**: Use fire dynamics tools for interface height
        3. **Visibility Analysis**: Calculate visibility through smoke
        4. **Hot Gas Temperature**: Use appropriate correlations
        5. **Evacuation Time**: SFPE or PD 7974 methods
        6. **Fire Growth**: t-squared or other growth models
        7. **Ventilation**: Natural or mechanical ventilation calculations
        
        ### Error Handling
        
        Always include proper error handling:
        ```python
        try:
            result = ofire.module.function(parameters)
            st.success("Calculation completed!")
        except Exception as e:
            st.error(f"Calculation error: {e}")
            st.info("Please check input parameters are within valid ranges.")
        ```
        
        ### File Operations
        
        For data input/output:
        - Save results to `results/` directory
        - Read input data from `data/` directory  
        - Use `st.file_uploader()` for user file uploads
        - Use `st.download_button()` for result downloads
        
        ### Documentation Standards
        
        - Add clear docstrings to all functions
        - Include parameter descriptions with units
        - Reference fire engineering standards (BS, CIBSE, SFPE, etc.)
        - Provide methodology information in the UI
        
        ### User Experience Considerations
        
        Remember that users are fire engineers, not programmers:
        - Use fire engineering terminology
        - Provide clear parameter descriptions with units
        - Include relevant engineering context and assumptions
        - Show calculation methodology and references
        - Validate inputs for realistic engineering values
        - Provide meaningful error messages
        
        ### Testing and Validation
        
        When creating calculations:
        1. Test with known values from fire engineering handbooks
        2. Include reasonable default values
        3. Set appropriate min/max limits on inputs
        4. Compare results with manual calculations where possible
        
        ### Getting Help
        
        - OpenFire documentation: https://emberon-tech.github.io/openfire/
        - Streamlit documentation: https://docs.streamlit.io/
        - This project's README.md for user instructions
        
        ## Summary for AI Agents
        
        When helping fire engineers:
        1. **Always use ofire library** for fire engineering calculations
        2. **Use Streamlit** for all user interfaces
        3. **Follow the established patterns** in main.py
        4. **Focus on fire engineering needs**, not programming complexity
        5. **Include proper documentation** and engineering context
        6. **Test calculations** with realistic fire engineering values
        7. **Make interfaces intuitive** for non-programmers
    ''').strip()
    
    with open(agents_file, 'w') as f:
        f.write(content)
    
    print(f"Created AI agent guide: {agents_file}")


def create_readme(project_name: str, target_dir: str) -> None:
    """Create a README.md file."""
    project_path = Path(target_dir).resolve() / project_name
    readme_file = project_path / "README.md"
    
    content = dedent(f'''
        # {project_name}
        
        A web-based fire engineering application using the OpenFire library.
        
        ## Setup
        
        1. Install Python dependencies:
           ```bash
           pip install -r requirements.txt
           ```
        
        2. Run the fire engineering application:
           ```bash
           ofire run
           ```
        
        3. Open your browser to the URL shown in the terminal (usually `http://localhost:8501`)
        
        ## Project Structure
        
        - `main.py`: Main fire engineering application with calculation tools
        - `data/`: Input data files and datasets
        - `results/`: Calculation results and outputs
        - `scripts/`: Additional calculation scripts
        - `requirements.txt`: Python package dependencies
        
        ## Features
        
        This fire engineering tool includes:
        
        - **Interactive Web Interface**: User-friendly calculation interface
        - **Heat Release Rate Calculator**: CIBSE Guide E calculations
        - **Smoke Filling Analysis**: Room smoke filling estimations
        - **Extensible Framework**: Easy to add new calculations
        
        ## Available OpenFire Modules
        
        This project uses the OpenFire library which provides implementations for:
        
        - **BR 187**: External fire spread calculations
        - **BS 9999**: Fire safety calculations
        - **CIBSE Guide E**: Fire safety engineering calculations
        - **Fire Dynamics Tools**: General fire dynamics calculations
        - **PD 7974**: Fire safety engineering calculations
        - **SFPE Handbook**: Fire protection engineering calculations
        - **TR 17**: Fire calculations
        - **Introduction to Fire Dynamics**: Basic fire dynamics
        
        ## Adding New Calculations
        
        To add a new calculation page:
        
        1. Create a new function in `main.py` following the pattern of existing pages
        2. Add the page to the sidebar navigation selectbox
        3. Implement your calculations using OpenFire library functions
        4. Use the web interface components for inputs and results display
        
        Example:
        ```python
        def your_calculation_page():
            st.header("Your Calculation")
            # Add input widgets and calculation logic here
        ```
        
        ## Documentation
        
        For detailed documentation and examples, visit:
        [OpenFire Documentation](https://emberon-tech.github.io/openfire/)
        
        ## Running Specific Files
        
        To run a specific file or URL:
        ```bash
        ofire run your_app.py
        ofire run https://example.com/fire_app.py
        ```
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
    
    with open(notebook_file, 'w') as f:
        json.dump(notebook_content, f, indent=2)
    
    print(f"Created example notebook: {notebook_file}")


def open_documentation() -> None:
    """Open the OpenFire documentation in the default browser."""
    docs_url = "https://emberon-tech.github.io/openfire/"
    
    print(f"Opening documentation: {docs_url}")
    
    try:
        webbrowser.open(docs_url)
        print("Documentation opened in your default browser!")
    except Exception as e:
        print(f"Error opening browser: {e}")
        print(f"You can manually visit: {docs_url}")


def run_fire_app(target: str = None) -> None:
    """Run a fire engineering application."""
    if target is None:
        # Default to main.py
        target = "main.py"
        print("Running fire engineering app (main.py)")
    elif target.startswith(('http://', 'https://')):
        # URL provided
        print(f"Running fire engineering app from URL: {target}")
    elif os.path.exists(target):
        # File path provided
        print(f"Running fire engineering app: {target}")
    else:
        # Assume it's a file that might not exist yet
        print(f"Running fire engineering app: {target}")
    
    try:
        # Build the command to run the web app
        cmd = [sys.executable, '-m', 'streamlit', 'run', target]
        
        print("Starting fire engineering web application...")
        print("Press Ctrl+C to stop the application")
        
        # Run the app - this will block until the user stops it
        subprocess.run(cmd, check=True)
        
    except subprocess.CalledProcessError as e:
        print(f"Error running fire engineering app: {e}")
        if e.returncode == 2:
            print("Required dependencies not installed. Please install with: pip install -r requirements.txt")
        sys.exit(1)
    except KeyboardInterrupt:
        print("\nFire engineering application stopped")
    except FileNotFoundError:
        print("Error: Python not found in PATH")
        sys.exit(1)


def scaffold_new_project(project_name: str, target_dir: str, include_notebook: bool = False) -> None:
    """Scaffold a new OpenFire project with all required files and setup."""
    print(f"Creating OpenFire project: {project_name}")
    print(f"Target directory: {target_dir}")
    
    try:
        # Create project structure
        create_project_structure(project_name, target_dir)
        
        # Create files
        create_main_script(project_name, target_dir)
        create_requirements_file(project_name, target_dir)
        create_readme(project_name, target_dir)
        create_agents_guide(project_name, target_dir)
        
        if include_notebook:
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
            print("3. ofire run")
        else:
            print("2. source activate.sh  # or: source .venv/bin/activate")
            print("3. ofire run")
        
        print("4. Open your browser to http://localhost:8501")
        
        if include_notebook:
            print("5. jupyter notebook example_calculations.ipynb")
        
        print("\nVirtual environment created with all dependencies installed!")
        print("🔥 Your fire engineering web application is ready to run!")
        print("For documentation, visit: https://emberon-tech.github.io/openfire/")
        
    except Exception as e:
        print(f"\nError creating project: {e}")
        sys.exit(1)