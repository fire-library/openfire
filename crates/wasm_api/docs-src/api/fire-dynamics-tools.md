---
title: "Fire Dynamics Tools"
group: "API Reference"
category: "Modules"
---

# Fire Dynamics Tools

General fire dynamics calculations organized by document chapter.

## Overview

The Fire Dynamics Tools module provides calculations from various chapters of fire dynamics engineering documents. Functions are organized hierarchically by chapter for easy navigation and reference.

## Accessing the Module

```javascript
import init, { get_fire_dynamics_tools } from 'openfire-wasm';

await init();
const fdt = get_fire_dynamics_tools();
```

## Chapter Organization

### Chapter 2 - Compartment Fire Dynamics
Functions related to fire behavior in compartments, including hot gas temperatures and layer properties.

**Available Functions:**
- [`hot_gas_temperature_increase`](#hot_gas_temperature_increase) - Calculate temperature increase of hot gases
- [`density_hot_gas_layer`](#density_hot_gas_layer) - Calculate density of hot gas layer

### Chapter 4 - Heat Transfer
Heat transfer calculations for compartment boundaries and structural elements.

**Available Functions:**
- [`heat_transfer_coefficient_short_times`](#heat_transfer_coefficient_short_times) - Heat transfer coefficient for short times or thick walls

### Chapter 5 - Smoke Movement
Calculations related to smoke movement and thermal penetration.

**Available Functions:**
- [`thermal_penetration_time`](#thermal_penetration_time) - Calculate thermal penetration time

---

## Function Reference

### Chapter 2 Functions

#### hot_gas_temperature_increase

```javascript
fdt.chapter_2.hot_gas_temperature_increase(heat_release_rate, compartment_area, compartment_height)
```

Calculates the temperature increase of hot gases in a compartment fire. This is a simplified version for demonstration of namespace organization.

**Parameters:**
- `heat_release_rate` (number): Heat release rate in kW
- `compartment_area` (number): Floor area of compartment in m²
- `compartment_height` (number): Height of compartment in m

**Returns:**
- (number): Temperature increase in Kelvin

**Example:**
```javascript
const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
console.log(`Temperature increase: ${tempIncrease} K`);
```

**Equation:**
This function implements a simplified version of the equation:
```
ΔTg = 6.85 × (Q²/(A×h))^(1/3)
```
Where:
- ΔTg = Hot gas temperature increase (K)
- Q = Heat release rate (kW) 
- A = Compartment floor area (m²)
- h = Compartment height (m)

---

#### density_hot_gas_layer

```javascript
fdt.chapter_2.density_hot_gas_layer(hot_gas_temperature)
```

Calculates the density of the hot gas layer based on temperature using the ideal gas law.

**Parameters:**
- `hot_gas_temperature` (number): Temperature of hot gas layer in Kelvin

**Returns:**
- (number): Density in kg/m³

**Example:**
```javascript
const density = fdt.chapter_2.density_hot_gas_layer(573); // 300°C
console.log(`Hot gas density: ${density.toFixed(3)} kg/m³`);
```

**Equation:**
```
ρ = P/(R×T)
```
Where:
- ρ = Density (kg/m³)
- P = Pressure (Pa) - assumes standard atmospheric pressure
- R = Specific gas constant for air (287 J/(kg·K))
- T = Temperature (K)

---

### Chapter 4 Functions

#### heat_transfer_coefficient_short_times

```javascript
fdt.chapter_4.heat_transfer_coefficient_short_times(thermal_conductivity, thermal_penetration_time)
```

Calculate heat transfer coefficient for short times or thick walls.

**Parameters:**
- `thermal_conductivity` (number): Thermal conductivity in W/(m·K)
- `thermal_penetration_time` (number): Thermal penetration time in seconds

**Returns:**
- (number): Heat transfer coefficient in W/(m²·K)

**Example:**
```javascript
const heatCoeff = fdt.chapter_4.heat_transfer_coefficient_short_times(0.5, 100);
console.log(`Heat transfer coefficient: ${heatCoeff} W/(m²·K)`);
```

---

### Chapter 5 Functions

#### thermal_penetration_time

```javascript
fdt.chapter_5.thermal_penetration_time(thermal_diffusivity, thickness)
```

Calculate thermal penetration time through a material.

**Parameters:**
- `thermal_diffusivity` (number): Thermal diffusivity in m²/s
- `thickness` (number): Wall thickness in meters

**Returns:**
- (number): Thermal penetration time in seconds

**Example:**
```javascript
const penetrationTime = fdt.chapter_5.thermal_penetration_time(1.2e-7, 0.1);
console.log(`Penetration time: ${penetrationTime} seconds`);
```

**Equation:**
```
tp = δ²/α
```
Where:
- tp = Thermal penetration time (s)
- δ = Material thickness (m)
- α = Thermal diffusivity (m²/s)

---

## Usage Patterns

### Basic Calculation

```javascript
import init, { get_fire_dynamics_tools } from 'openfire-wasm';

async function basicCalculation() {
  await init();
  const fdt = get_fire_dynamics_tools();
  
  const result = fdt.chapter_2.hot_gas_temperature_increase(1000, 50, 3);
  console.log(result);
}
```

### Chained Calculations

```javascript
async function chainedCalculations() {
  await init();
  const fdt = get_fire_dynamics_tools();
  
  // Calculate temperature increase
  const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(1500, 75, 3.5);
  
  // Calculate hot gas temperature
  const ambientTemp = 293; // K
  const hotGasTemp = ambientTemp + tempIncrease;
  
  // Calculate density at that temperature
  const density = fdt.chapter_2.density_hot_gas_layer(hotGasTemp);
  
  console.log(`Final density: ${density} kg/m³`);
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
    console.error('Calculation failed:', error);
    return null;
  }
}
```

## Related Documentation

- [Getting Started Guide](../user-guide/getting-started.html) - Basic usage examples
- [Fire Dynamics Examples](../user-guide/fire-dynamics-examples.html) - Practical calculation examples
- [Installation](../installation.html) - Setup instructions

## Future Modules

The following modules are planned for future releases:

- **PD 7974** - Fire Safety Engineering Principles
- **BR 187** - External Fire Spread
- **BS 9999** - Fire Safety in Buildings  
- **CIBSE Guide E** - Fire Safety Engineering
- **SFPE Handbook** - Fire Protection Engineering
- **TR 17** - Technical Report