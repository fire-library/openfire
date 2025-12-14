"""OpenFire CLI for scaffolding fire engineering projects."""

import argparse
from .project import scaffold_new_project, open_documentation, run_fire_app


def open_docs(args):
    """Open the OpenFire documentation in the default browser."""
    open_documentation()


def scaffold_project(args):
    """Scaffold a new OpenFire project."""
    scaffold_new_project(
        project_name=args.name,
        target_dir=args.directory,
        include_notebook=args.notebook
    )


def run_app(args):
    """Run a fire engineering application."""
    run_fire_app(target=args.target)


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
    
    # Run command
    run_parser = subparsers.add_parser(
        'run',
        help='Run a fire engineering application'
    )
    run_parser.add_argument(
        'target',
        nargs='?',
        default=None,
        help='File path or URL to run (default: main.py)'
    )
    run_parser.set_defaults(func=run_app)
    
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