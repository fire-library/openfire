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
    
    print(f"Created project structure in: {project_path}")


def create_main_script(project_name: str, target_dir: str) -> None:
    """Create the main fire engineering application script."""
    project_path = Path(target_dir).resolve() / project_name
    main_script = project_path / "main.py"
    
    # Read template file
    template_path = Path(__file__).parent / "templates" / "main.py"
    try:
        with open(template_path, 'r') as f:
            template_content = f.read()
    except FileNotFoundError:
        # Fallback content if template file is missing
        template_content = '"""Template file not found"""'
        print(f"Warning: Template file not found at {template_path}, using fallback content")
    
    # Format template with project name
    content = template_content.format(project_name=project_name)
    
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
    ''').strip()
    
    with open(requirements_file, 'w') as f:
        f.write(content)
    
    print(f"Created requirements file: {requirements_file}")


def create_claude_guide(project_name: str, target_dir: str) -> None:
    """Create a CLAUDE.md file that instructs Claude to always read AGENTS.md."""
    project_path = Path(target_dir).resolve() / project_name
    claude_file = project_path / "CLAUDE.md"
    
    content = "Always read @AGENTS.md"
    
    with open(claude_file, 'w') as f:
        f.write(content)
    
    print(f"Created Claude guide: {claude_file}")


def create_agents_guide(project_name: str, target_dir: str) -> None:
    """Create an AGENTS.md file with guidance for AI coding agents."""
    project_path = Path(target_dir).resolve() / project_name
    agents_file = project_path / "AGENTS.md"
    
    # Read the template file
    template_path = Path(__file__).parent / "templates" / "agents_template.md"
    try:
        with open(template_path, 'r') as f:
            content = f.read()
    except FileNotFoundError:
        # Fallback content if template file is missing
        content = "# AI Agent Guide\n\nAlways use ofire library for fire engineering calculations."
        print(f"Warning: Template file not found at {template_path}, using fallback content")
    
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


def scaffold_new_project(project_name: str, target_dir: str) -> None:
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
        create_claude_guide(project_name, target_dir)
        
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
        
        print("\nVirtual environment created with all dependencies installed!")
        print("🔥 Your fire engineering web application is ready to run!")
        print("For documentation, visit: https://emberon-tech.github.io/openfire/")
        
    except Exception as e:
        print(f"\nError creating project: {e}")
        sys.exit(1)