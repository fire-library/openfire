# copilot-instructions.md

## Project Overview

This is a multi-crate Rust project using a Cargo workspace. The root directory contains a `Cargo.toml` that defines the workspace and members (individual crates). Each crate has its own `Cargo.toml`, `src/` directory, and may depend on other crates in the workspace.

## Project Structure

- `Cargo.toml` (workspace root)
- `crates/`
  - `crate_a/`
    - `Cargo.toml`
    - `src/`
  - `crate_b/`
    - `Cargo.toml`
    - `src/`
  - ...
- `target/` (build output)

This project provides implementations for a wide range of Fire Engineering calculations. These calculations are consumed by a Tauri application and presented via a React frontend.

At the core of the project is the framework crate, which serves as the foundation for defining and executing methods:

    The Method struct, located at crates/framework/src/method.rs, represents the structure of a fire engineering method. This struct is serialized and sent to the frontend for display.

    The MethodRunner trait, defined in crates/framework/src/method/runner.rs, provides an interface for executing the logic behind each method. It exposes standardized methods that individual implementations must provide.

Each of the other crates in the workspace corresponds to a specific document or domain in fire engineering. These crates implement multiple `MethodRunner` trait instances, each representing a different fire engineering method.

This structure allows the project to remain modular, extensible, and easy to integrate into the frontend application.


Documents that the equations are based on are provided in the `./Documents` folder. This folder contains a set of PDF files that can be searched. When asked to create a runner from a document, the appropriate document should be checked to find the specific method.

## Conventions

- Modules are defined in `mod.rs` or directly within `lib.rs` or `main.rs`.
- Public APIs are declared with `pub` and documented with `///`.
- Internal modules are kept private unless explicitly needed externally.
- Tests are colocated using `#[cfg(test)]` and/or in `tests/` directories.
- Common utilities are often factored into shared crates.

## Creating Method Runners

A runner is an implementation of the MethodRunner trait. The trait defines a consistent interface, but calculation logic must be in separate modules, allowing reuse across different runners.
Structure of a Runner

Each runner has:

A folder named after the equation or method it implements (e.g. equation_10_1/)

Inside this folder:

### Example Layout

- openfire_runner.rs – implements the MethodRunner trait
- openfire_runner/ – contains all integration tests:
  - integration_tests.rs – returns a list of available tests
  - integration_tests/ – contains:
    - test_1.rs, test_2.rs, etc. – each defining an individual test
    - description – describes the required test coverage
    - Optionally, test_1/description, test_2/description, etc.

The calculation logic for the method is not in the runner file, but in a separate .rs file, e.g. equation_10_1.rs.

```
crates/
  cibse_guide_e/
    Cargo.toml
    src/
      chapter_10.rs              # Collects runners from chapter 10
      chapter_10/
        equation_10_1.rs        # Calculation logic
        equation_10_1/
          openfire_runner.rs    # MethodRunner implementation
          openfire_runner/
            integration_tests.rs
            integration_tests/
              test_1.rs
              test_2.rs
              description
              test_1/
                description
              test_2/
                description
```

Whenever you are asked to create a runner, follow this folder structure and use the files quoted above as guides for what should be in the files.

### When Asked to Create a Runner

Whenever a runner needs to be created:

- Follow the example above closely.
- Refer to openfire_runner.rs, integration_tests.rs, and the description files as templates.
- Make sure all files are placed according to the described folder structure.
- Find the relevant equation or method in the appropriate PDF inside the Documents/ folder.

## Crate Interactions

- Crates within the workspace use path dependencies to refer to each other:

```toml
[dependencies]
crate_b = { path = "../crate_b" }
```
