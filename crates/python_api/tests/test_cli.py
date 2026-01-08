import pytest
import subprocess
import sys
import os
from pathlib import Path


class TestCLIAvailability:
    """Test that the CLI is properly installed and accessible."""
    
    def test_cli_importable(self, ofire_available):
        """Test that ofire module can be imported."""
        import ofire.cli
        assert hasattr(ofire.cli, 'main')
    
    def test_cli_executable(self, cli_runner):
        """Test that ofire command is executable."""
        result = cli_runner("--help")
        assert result.returncode == 0
        assert "OpenFire CLI" in result.stdout


class TestVersionCommand:
    """Test the version command functionality."""
    
    def test_version_command(self, cli_runner, ofire_available):
        """Test version command shows version information."""
        result = cli_runner("version")
        assert result.returncode == 0
        assert "OpenFire CLI" in result.stdout
        # Should show either version number or "unknown" message
        assert ("v" in result.stdout) or ("unknown" in result.stdout)
    
    def test_version_command_output_format(self, cli_runner, ofire_available):
        """Test version command output format."""
        result = cli_runner("version")
        lines = result.stdout.strip().split('\n')
        assert len(lines) == 1  # Should be single line output
        assert result.stdout.startswith("OpenFire CLI")


class TestHelpCommand:
    """Test help command and general CLI structure."""
    
    def test_help_command(self, cli_runner, ofire_available):
        """Test that help command works."""
        result = cli_runner("--help")
        assert result.returncode == 0
        assert "OpenFire CLI" in result.stdout
        assert "Available commands" in result.stdout
    
    def test_subcommands_listed(self, cli_runner, ofire_available):
        """Test that all expected subcommands are listed in help."""
        result = cli_runner("--help")
        output = result.stdout
        
        expected_commands = ["new", "run", "docs", "version"]
        for cmd in expected_commands:
            assert cmd in output
    
    def test_subcommand_help(self, cli_runner, ofire_available):
        """Test that subcommands have their own help."""
        result = cli_runner("new", "--help")
        assert result.returncode == 0
        assert "Create a new OpenFire project" in result.stdout
        
        result = cli_runner("run", "--help")
        assert result.returncode == 0
        assert "Run a fire engineering application" in result.stdout


class TestNewCommand:
    """Test the new project creation command."""
    
    def test_new_command_creates_project(self, cli_runner, temp_dir, sample_project_name, ofire_available):
        """Test that new command creates a project structure."""
        result = cli_runner("new", sample_project_name, "-d", str(temp_dir))
        assert result.returncode == 0
        
        project_dir = temp_dir / sample_project_name
        assert project_dir.exists()
        assert project_dir.is_dir()
    
    def test_new_command_creates_main_file(self, cli_runner, temp_dir, sample_project_name, ofire_available):
        """Test that new command creates main.py file."""
        result = cli_runner("new", sample_project_name, "-d", str(temp_dir))
        assert result.returncode == 0
        
        main_file = temp_dir / sample_project_name / "main.py"
        assert main_file.exists()
        assert main_file.is_file()
    
    def test_new_command_without_directory_flag(self, cli_runner, temp_dir, ofire_available):
        """Test new command works in current directory."""
        project_name = "current_dir_test"
        result = cli_runner("new", project_name, cwd=str(temp_dir))
        assert result.returncode == 0
        
        project_dir = temp_dir / project_name
        assert project_dir.exists()
    
    def test_new_command_missing_name(self, cli_runner, ofire_available):
        """Test new command fails without project name."""
        result = cli_runner("new", check=False)
        assert result.returncode != 0
        assert "required" in result.stderr.lower() or "required" in result.stdout.lower()
    
    @pytest.mark.parametrize("project_name", [
        "simple_project",
        "project-with-hyphens", 
        "project_123",
        "CamelCaseProject"
    ])
    def test_new_command_various_names(self, cli_runner, temp_dir, project_name, ofire_available):
        """Test new command with various valid project names."""
        result = cli_runner("new", project_name, "-d", str(temp_dir))
        assert result.returncode == 0
        
        project_dir = temp_dir / project_name
        assert project_dir.exists()


class TestRunCommand:
    """Test the run command functionality."""
    
    def test_run_command_help(self, cli_runner, ofire_available):
        """Test run command shows help properly."""
        result = cli_runner("run", "--help")
        assert result.returncode == 0
        assert "Run a fire engineering application" in result.stdout
    
    def test_run_command_with_nonexistent_file(self, cli_runner, temp_dir, ofire_available):
        """Test run command with non-existent file."""
        nonexistent_file = temp_dir / "nonexistent.py"
        result = cli_runner("run", str(nonexistent_file), check=False, cwd=str(temp_dir))
        # Should fail gracefully (exact behavior depends on implementation)
        assert result.returncode != 0 or "not found" in result.stderr.lower()


class TestDocsCommand:
    """Test the docs command functionality."""
    
    def test_docs_command_help(self, cli_runner, ofire_available):
        """Test docs command shows help properly."""
        result = cli_runner("docs", "--help")
        assert result.returncode == 0
        assert "Open OpenFire documentation" in result.stdout
    
    def test_docs_command_execution(self, cli_runner, ofire_available):
        """Test docs command executes without error."""
        # Note: This test might need adjustment based on how docs command works
        # If it opens a browser, it might succeed silently or need mocking
        result = cli_runner("docs", check=False)
        # Should either succeed or fail gracefully
        assert result.returncode in [0, 1]  # Allow either success or controlled failure


class TestErrorHandling:
    """Test CLI error handling and edge cases."""
    
    def test_invalid_command(self, cli_runner, ofire_available):
        """Test that invalid commands are handled properly."""
        result = cli_runner("invalid_command", check=False)
        assert result.returncode != 0
    
    def test_no_command(self, cli_runner, ofire_available):
        """Test that running CLI with no command shows help."""
        result = cli_runner()
        # Should show help when no command is provided
        assert "OpenFire CLI" in result.stdout
        assert "Available commands" in result.stdout


class TestCrossPlatformCompatibility:
    """Test cross-platform specific functionality."""
    
    def test_path_handling(self, cli_runner, temp_dir, ofire_available):
        """Test that paths are handled correctly across platforms."""
        project_name = "path_test_project"
        
        # Test with different path separators
        target_dir = temp_dir / "nested" / "directory"
        target_dir.mkdir(parents=True, exist_ok=True)
        
        result = cli_runner("new", project_name, "-d", str(target_dir))
        assert result.returncode == 0
        
        project_dir = target_dir / project_name
        assert project_dir.exists()
    
    @pytest.mark.skipif(os.name == 'nt', reason="Unix-specific test")
    def test_unix_permissions(self, cli_runner, temp_dir, sample_project_name, ofire_available):
        """Test file permissions on Unix systems."""
        result = cli_runner("new", sample_project_name, "-d", str(temp_dir))
        assert result.returncode == 0
        
        main_file = temp_dir / sample_project_name / "main.py"
        assert main_file.exists()
        # Check that file is readable
        assert os.access(str(main_file), os.R_OK)
    
    @pytest.mark.skipif(os.name != 'nt', reason="Windows-specific test")
    def test_windows_path_handling(self, cli_runner, temp_dir, sample_project_name, ofire_available):
        """Test Windows-specific path handling."""
        # Test with Windows-style paths if on Windows
        result = cli_runner("new", sample_project_name, "-d", str(temp_dir))
        assert result.returncode == 0
        
        project_dir = temp_dir / sample_project_name
        assert project_dir.exists()
        # Verify Windows path normalization works
        assert str(project_dir).replace('/', os.sep) == str(project_dir)