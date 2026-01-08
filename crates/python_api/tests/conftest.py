import pytest
import tempfile
import shutil
import subprocess
import sys
import os
from pathlib import Path


@pytest.fixture
def temp_dir():
    """Create a temporary directory for testing."""
    temp_path = tempfile.mkdtemp()
    yield Path(temp_path)
    shutil.rmtree(temp_path, ignore_errors=True)


@pytest.fixture
def cli_runner():
    """Helper to run CLI commands with proper error handling."""
    def run_cli(*args, check=True, cwd=None, input=None):
        """Run ofire CLI command and return CompletedProcess result."""
        cmd = [sys.executable, "-m", "ofire"] + list(args)
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                check=check,
                cwd=cwd,
                input=input,
                timeout=30
            )
            return result
        except subprocess.TimeoutExpired:
            pytest.fail(f"Command timed out: {' '.join(cmd)}")
        except subprocess.CalledProcessError as e:
            if check:
                pytest.fail(f"Command failed: {' '.join(cmd)}\nSTDOUT: {e.stdout}\nSTDERR: {e.stderr}")
            return e
    
    return run_cli


@pytest.fixture
def sample_project_name():
    """Provide a consistent test project name."""
    return "test_fire_project"


@pytest.fixture(scope="session")
def ofire_available():
    """Check if ofire CLI is available and properly installed."""
    try:
        result = subprocess.run(
            [sys.executable, "-m", "ofire", "--help"],
            capture_output=True,
            text=True,
            check=True,
            timeout=10
        )
        return True
    except (subprocess.CalledProcessError, subprocess.TimeoutExpired, FileNotFoundError):
        pytest.skip("ofire CLI not available - package not installed")