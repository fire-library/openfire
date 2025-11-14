# OpenFire

OpenFire is a multi-crate Rust project designed to provide implementations for a wide range of fire engineering calculations.

## Project Structure

The project is organized as a Cargo workspace with the following structure:

```
openfire/
├── Cargo.toml (workspace root)
├── crates/
│   ├── br_187/                # Crate for BR 187 document
│   ├── bs9999/                # Crate for BS 9999 document
│   ├── cibse_guide_e/         # Crate for CIBSE Guide E document
│   ├── framework/             # Core framework crate
│   ├── introduction_to_fire_dynamics/ # Crate for Introduction to Fire Dynamics
│   ├── openfire_cli/          # Command-line interface for OpenFire
│   ├── pd_7974/               # Crate for PD 7974 document
│   ├── sfpe_handbook/         # Crate for SFPE Handbook
│   └── tr17/                  # Crate for TR 17 document
└── src/                       # Main library source that only exposes the crates
```

### Domain-Specific Crates

Each crate in the `crates/` directory corresponds to a specific document or domain in fire engineering.
