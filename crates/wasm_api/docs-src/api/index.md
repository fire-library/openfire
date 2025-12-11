---
title: "API Reference"
group: "API Reference"
category: "Overview"
---

# API Reference

Complete API reference for all OpenFire WASM modules, organized by source document.

## Overview

The OpenFire WASM API provides fire safety engineering calculations through a WebAssembly interface. Functions are organized hierarchically by source document and chapter to match fire engineering standards and guides.

## Module Organization

### Available Modules

#### [Fire Dynamics Tools](fire-dynamics-tools.html)
General fire dynamics calculations organized by chapter:
- **Chapter 2**: Compartment Fire Dynamics
- **Chapter 4**: Heat Transfer  
- **Chapter 5**: Smoke Movement

### Coming Soon

The following modules are planned for future releases:

#### PD 7974 - Fire Safety Engineering Principles
- Part 1: Application of fire safety engineering principles
- Additional parts as implemented

#### BR 187 - External Fire Spread
- Appendix A: External fire spread calculations
- Chapter 1: Building separation requirements

#### BS 9999 - Fire Safety in Buildings
- Chapter 15: Smoke control systems

#### CIBSE Guide E - Fire Safety Engineering
- Chapter 6: Compartment fires
- Chapter 7: Structural fire protection
- Chapter 10: Fire dynamics

#### SFPE Handbook
- Chapter 14: Heat transfer calculations

#### TR 17 - Technical Report
- Section 2: Fire safety calculations

## Core Module Functions

### Main API Access

```javascript
import init, { get_fire_dynamics_tools } from 'openfire-wasm';

// Initialize WASM module
await init();

// Access organized API
const fdt = get_fire_dynamics_tools();
```

### Utility Functions

#### Basic Calculations
```javascript
import { calculate_circle_area, calculate_fire_spread_risk } from 'openfire-wasm';

// Simple geometry
const area = calculate_circle_area(5.0);

// Fire spread risk assessment
const risk = calculate_fire_spread_risk(10.0, 15.0);
```

## API Design Principles

### Hierarchical Organization
Functions are organized to match fire engineering document structure:
```javascript
// Document -> Chapter -> Function
fdt.chapter_2.hot_gas_temperature_increase(...)
fdt.chapter_4.heat_transfer_coefficient_short_times(...)
fdt.chapter_5.thermal_penetration_time(...)
```

### Consistent Parameter Units
All functions use consistent SI units:
- **Length**: meters (m)
- **Area**: square meters (m²)
- **Temperature**: Kelvin (K) 
- **Heat Release Rate**: kilowatts (kW)
- **Time**: seconds (s)
- **Density**: kilograms per cubic meter (kg/m³)
- **Thermal Properties**: SI standard units

### Clear Function Names
Function names clearly indicate their purpose and source equation where applicable:
- `hot_gas_temperature_increase` - calculates temperature rise
- `density_hot_gas_layer` - calculates gas layer density
- `thermal_penetration_time` - calculates heat penetration timing

## Quick Reference

### Fire Dynamics Tools

| Function | Chapter | Description |
|----------|---------|-------------|
| `hot_gas_temperature_increase` | 2 | Temperature increase in compartment fire |
| `density_hot_gas_layer` | 2 | Hot gas layer density calculation |
| `heat_transfer_coefficient_short_times` | 4 | Heat transfer coefficient (short times) |
| `thermal_penetration_time` | 5 | Thermal penetration through materials |

### Utility Functions

| Function | Description |
|----------|-------------|
| `calculate_circle_area` | Calculate area of circle (demonstration) |
| `calculate_fire_spread_risk` | Fire spread risk assessment (example) |

## Usage Examples

### Basic Fire Analysis
```javascript
import init, { get_fire_dynamics_tools } from 'openfire-wasm';

async function fireAnalysis() {
  await init();
  const fdt = get_fire_dynamics_tools();
  
  // Input parameters
  const heatReleaseRate = 1000; // kW
  const compartmentArea = 50;   // m²
  const compartmentHeight = 3;  // m
  
  // Calculate temperature increase
  const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
    heatReleaseRate, compartmentArea, compartmentHeight
  );
  
  // Calculate hot gas temperature
  const hotGasTemp = 293 + tempIncrease; // K (20°C ambient + increase)
  
  // Calculate density
  const density = fdt.chapter_2.density_hot_gas_layer(hotGasTemp);
  
  console.log(`Temperature increase: ${tempIncrease.toFixed(1)} K`);
  console.log(`Hot gas density: ${density.toFixed(3)} kg/m³`);
}
```

### Error Handling
```javascript
async function safeCalculation() {
  try {
    await init();
    const fdt = get_fire_dynamics_tools();
    
    const result = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
    return result;
  } catch (error) {
    console.error('API call failed:', error);
    return null;
  }
}
```

## Migration from Individual Functions

If you're migrating from using individual exported functions, the new organized API provides better structure:

**Old approach:**
```javascript
import { hot_gas_temperature_increase } from 'openfire-wasm';
const result = hot_gas_temperature_increase(q, a_v, h_v, a_t, h_k);
```

**New organized approach:**
```javascript
import { get_fire_dynamics_tools } from 'openfire-wasm';
const fdt = get_fire_dynamics_tools();
const result = fdt.chapter_2.hot_gas_temperature_increase(q, a, h);
```

## Documentation Structure

This API reference follows the same organizational pattern as the [OpenFire Python API](https://fire-library.github.io/openfire/api/):

1. **Installation** - Setup and configuration
2. **User Guide** - Getting started and examples
3. **API Reference** - Complete function documentation

## Related Resources

- [Getting Started](../user-guide/getting-started.html) - Basic usage guide
- [Fire Dynamics Examples](../user-guide/fire-dynamics-examples.html) - Practical examples
- [Installation](../installation.html) - Setup instructions
- [Python API Documentation](https://fire-library.github.io/openfire/) - Equivalent Python interface