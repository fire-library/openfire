# OpenFire WASM API

Fire safety engineering tools for WebAssembly and JavaScript applications.

## Overview

OpenFire provides high-performance fire safety engineering calculations through WebAssembly bindings. Built in Rust for maximum speed and memory safety, with JavaScript/TypeScript integration for web applications.

## Key Features

- **High Performance**: Implemented in Rust and compiled to WebAssembly for maximum speed
- **JavaScript/TypeScript Integration**: Easy-to-use JavaScript API with full TypeScript support
- **Fire Dynamics**: Tools for fire dynamics calculations organized by document and chapter
- **Safety Engineering**: Comprehensive fire safety analysis tools

## Installation

```bash
npm install openfire-wasm
```

## Quick Start

```javascript
import init, { get_fire_dynamics_tools } from 'openfire-wasm';

// Initialize the WASM module
await init();

// Get the Fire Dynamics Tools API
const fdt = get_fire_dynamics_tools();

// Chapter 2 - Compartment Fire Dynamics
const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
console.log(`Hot gas temperature increase: ${tempIncrease} K`);

const density = fdt.chapter_2.density_hot_gas_layer(573);
console.log(`Hot gas layer density: ${density} kg/m³`);
```

## Documentation Structure

This documentation is organized similarly to the [OpenFire Python API](https://fire-library.github.io/openfire/):

### Installation
- **Installation Guide** - How to install and set up OpenFire WASM

### User Guide  
- **Getting Started** - Basic usage and examples
- **Fire Dynamics Examples** - Practical examples

### API Reference
- **Fire Dynamics Tools** - General fire dynamics calculations
- **Complete API Reference** - All available functions and classes

### In Node.js

```javascript
const { 
  hot_gas_temperature_increase,
  compartment_interior_surface_area,
  thermal_penetration_time
} = require('openfire-wasm');

// Use the functions directly (initialization is handled automatically in Node.js)
const tempIncrease = hot_gas_temperature_increase(1000.0, [2.5, 1.5], [2.0, 1.0], 75.0, 0.035);
console.log(`Hot gas temperature increase: ${tempIncrease.toFixed(2)} K`);
```

## Available Functions

### Fire Dynamics Tools - Chapter 2

All functions are from the fire dynamics tools module, chapter 2:

- `hot_gas_temperature_increase(q, a_v, h_v, a_t, h_k)` - Equation 2.1
- `compartment_interior_surface_area(w_c, l_c, h_c, a_v)` - Equation 2.2  
- `heat_transfer_coefficient_longtimes_or_thinwalls(k, delta)` - Equation 2.3
- `thermal_penetration_time(rho, c_p, k, delta)` - Equation 2.4
- `heat_transfer_coefficient_shorttimes_or_thickwalls(k, rho, c, t)` - Equation 2.5
- `hot_gas_temperature_increase_beyler_closed_compartment(k, rho, c, t, m, c_p, q)` - Equation 2.6
- `nondimensional_hot_gas_temperature_increase(q, m, t_a, h_k, a_t, c_p)` - Equation 2.7
- `hot_gas_temperature_increase_forced_ventilation(q, m, c_p, h_k, a_t)` - Equation 2.8
- `convective_heat_transfer_coefficient(k, rho, c, t, delta)` - Equation 2.9
- `height_smoke_layer_interface_natural_ventilation(k, q, t, a_c, h_c)` - Equation 2.10
- `k_constant_smoke_layer_height(rho_g, rho_a, g, c_p, t_a)` - Equation 2.11
- `k_constant_smoke_layer_height_post_substitution(rho_g)` - Equation 2.12
- `density_hot_gas_layer(t_g)` - Equation 2.13

## Building from Source

Requirements:
- Rust toolchain
- `wasm-pack` installed (`cargo install wasm-pack`)

```bash
# Build for web
npm run build

# Build for Node.js
npm run build-node

# Build for bundlers (webpack, rollup, etc.)
npm run build-bundler

# Development build (faster compilation, larger file size)
npm run dev
```

## TypeScript Support

Type definitions are automatically generated and included in the package. Import functions with full type safety:

```typescript
import { hot_gas_temperature_increase } from 'openfire-wasm';

const result: number = hot_gas_temperature_increase(
  1000.0,    // q: number
  [2.5, 1.5], // a_v: number[]
  [2.0, 1.0], // h_v: number[]
  75.0,      // a_t: number
  0.035      // h_k: number
);
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.