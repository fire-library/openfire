import argparse
from importlib.metadata import version, PackageNotFoundError
from .project import scaffold_new_project, open_documentation, run_fire_app


def open_docs(args):
    open_documentation()


def scaffold_project(args):
    scaffold_new_project(
        project_name=args.name,
        target_dir=args.directory
    )


def run_app(args):
    run_fire_app(target=args.target)


def show_version(args):
    """Show the OpenFire CLI version."""
    try:
        pkg_version = version('ofire')
        print(f"OpenFire CLI v{pkg_version}")
    except PackageNotFoundError:
        print("OpenFire CLI version unknown (package not found)")


def main():
    parser = argparse.ArgumentParser(
        description="OpenFire CLI - Tools for fire engineering projects",
        prog="ofire"
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
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
    new_parser.set_defaults(func=scaffold_project)
    
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
    
    docs_parser = subparsers.add_parser(
        'docs',
        help='Open OpenFire documentation in browser'
    )
    docs_parser.set_defaults(func=open_docs)
    
    version_parser = subparsers.add_parser(
        'version',
        help='Show OpenFire version'
    )
    version_parser.set_defaults(func=show_version)
    
    args = parser.parse_args()
    
    if args.command is None:
        parser.print_help()
        return
    
    args.func(args)


if __name__ == '__main__':
    main()