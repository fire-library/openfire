---
title: "Fire Dynamics Examples"
group: "User Guide"
category: "Examples"
---

# Fire Dynamics Examples

Practical examples for common fire safety engineering calculations using OpenFire WASM.

## Compartment Fire Analysis

### Example 1: Office Fire

Calculate key parameters for a fire in an office compartment:

```javascript
import init, { get_fire_dynamics_tools } from 'openfire-wasm';

async function officeFireAnalysis() {
  await init();
  const fdt = get_fire_dynamics_tools();
  
  // Office compartment properties
  const officeArea = 25;        // m² (5m x 5m office)
  const officeHeight = 2.7;     // m (typical ceiling height)
  const designFireSize = 1500;  // kW (furniture fire)
  
  console.log('=== Office Fire Analysis ===');
  console.log(`Compartment: ${officeArea} m² × ${officeHeight} m`);
  console.log(`Design fire: ${designFireSize} kW`);
  
  // Calculate hot gas temperature increase
  const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
    designFireSize, officeArea, officeHeight
  );
  
  const ambientTemp = 293; // K (20°C)
  const hotGasTemp = ambientTemp + tempIncrease;
  
  console.log(`Hot gas temperature increase: ${tempIncrease.toFixed(1)} K`);
  console.log(`Hot gas temperature: ${(hotGasTemp - 273.15).toFixed(1)} °C`);
  
  // Calculate hot gas density
  const hotGasDensity = fdt.chapter_2.density_hot_gas_layer(hotGasTemp);
  const ambientDensity = fdt.chapter_2.density_hot_gas_layer(ambientTemp);
  
  console.log(`Hot gas density: ${hotGasDensity.toFixed(3)} kg/m³`);
  console.log(`Density reduction: ${((1 - hotGasDensity/ambientDensity) * 100).toFixed(1)}%`);
  
  return {
    temperatureIncrease: tempIncrease,
    hotGasTemperature: hotGasTemp,
    density: hotGasDensity
  };
}
```

### Example 2: Parametric Study

Compare different fire sizes in the same compartment:

```javascript
async function parametricStudy() {
  await init();
  const fdt = get_fire_dynamics_tools();
  
  const compartmentArea = 50; // m²
  const compartmentHeight = 3; // m
  const fireSizes = [500, 1000, 2000, 5000]; // kW
  
  console.log('=== Parametric Fire Study ===');
  console.log('Fire Size (kW) | Temp Increase (K) | Hot Gas Temp (°C)');
  console.log('----------------------------------------------------');
  
  for (const fireSize of fireSizes) {
    const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
      fireSize, compartmentArea, compartmentHeight
    );
    const hotGasTemp = 293 + tempIncrease; // K
    
    console.log(
      `${fireSize.toString().padEnd(11)} | ` +
      `${tempIncrease.toFixed(1).padEnd(16)} | ` +
      `${(hotGasTemp - 273.15).toFixed(1)}`
    );
  }
}
```

## Heat Transfer Analysis

### Example 3: Wall Heating Analysis

Calculate heat transfer to compartment boundaries:

```javascript
async function wallHeatingAnalysis() {
  await init();
  const fdt = get_fire_dynamics_tools();
  
  console.log('=== Wall Heating Analysis ===');
  
  // Material properties
  const materials = [
    { name: 'Concrete', k: 1.4, rho: 2300, cp: 880 },
    { name: 'Gypsum Board', k: 0.17, rho: 640, cp: 1170 },
    { name: 'Steel', k: 45, rho: 7850, cp: 460 }
  ];
  
  const thickness = 0.1; // m
  
  for (const material of materials) {
    // Calculate thermal diffusivity
    const thermalDiffusivity = material.k / (material.rho * material.cp);
    
    // Thermal penetration time
    const penetrationTime = fdt.chapter_5.thermal_penetration_time(
      thermalDiffusivity, thickness
    );
    
    // Heat transfer coefficient for short times
    const heatTransferCoeff = fdt.chapter_4.heat_transfer_coefficient_short_times(
      material.k, penetrationTime
    );
    
    console.log(`${material.name}:`);
    console.log(`  Thermal diffusivity: ${(thermalDiffusivity * 1e6).toFixed(2)} mm²/s`);
    console.log(`  Penetration time: ${(penetrationTime / 60).toFixed(1)} min`);
    console.log(`  Heat transfer coeff: ${heatTransferCoeff.toFixed(1)} W/(m²·K)`);
    console.log('');
  }
}
```

## Practical Design Scenarios

### Example 4: Sprinkler Activation Analysis

Estimate time-related fire development parameters:

```javascript
async function sprinklerActivationAnalysis() {
  await init();
  const fdt = get_fire_dynamics_tools();
  
  console.log('=== Sprinkler Activation Analysis ===');
  
  // Fire growth parameters
  const growthRates = [
    { type: 'Slow', alpha: 0.00293 },      // kW/s²
    { type: 'Medium', alpha: 0.01172 },    // kW/s²
    { type: 'Fast', alpha: 0.04687 },      // kW/s²
    { type: 'Ultra-fast', alpha: 0.1876 }  // kW/s²
  ];
  
  const activationTemp = 68; // °C (sprinkler activation)
  const compartmentArea = 100; // m²
  const compartmentHeight = 3; // m
  
  console.log('Growth Rate | Time to Activation | Fire Size at Activation');
  console.log('--------------------------------------------------------');
  
  for (const growth of growthRates) {
    // Iterative approach to find activation time
    let time = 60; // start at 60 seconds
    let converged = false;
    
    while (!converged && time < 600) { // max 10 minutes
      const fireSize = growth.alpha * time * time; // Q = αt²
      
      const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
        fireSize, compartmentArea, compartmentHeight
      );
      
      const hotGasTemp = 20 + tempIncrease; // °C
      
      if (hotGasTemp >= activationTemp) {
        console.log(
          `${growth.type.padEnd(11)} | ` +
          `${time.toFixed(0).padEnd(17)} s | ` +
          `${fireSize.toFixed(0)} kW`
        );
        converged = true;
      }
      
      time += 1; // increment by 1 second
    }
    
    if (!converged) {
      console.log(`${growth.type.padEnd(11)} | No activation within 10 min`);
    }
  }
}
```

### Example 5: Ventilation Impact Study

Compare natural vs. forced ventilation scenarios:

```javascript
async function ventilationImpactStudy() {
  await init();
  const fdt = get_fire_dynamics_tools();
  
  console.log('=== Ventilation Impact Study ===');
  
  const fireSize = 2000; // kW
  const compartmentArea = 75; // m²
  const compartmentHeight = 3.5; // m
  
  // Base case - limited ventilation
  const baseTempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
    fireSize, compartmentArea, compartmentHeight
  );
  
  console.log('Scenario Analysis:');
  console.log(`Base case temperature increase: ${baseTempIncrease.toFixed(1)} K`);
  
  // Reduced fire sizes due to ventilation limitation
  const ventilationLimitedSizes = [1000, 1500, fireSize];
  
  console.log('\nVentilation-limited scenarios:');
  for (const limitedSize of ventilationLimitedSizes) {
    const tempIncrease = fdt.chapter_2.hot_gas_temperature_increase(
      limitedSize, compartmentArea, compartmentHeight
    );
    
    const reduction = ((baseTempIncrease - tempIncrease) / baseTempIncrease * 100);
    
    console.log(
      `Fire size: ${limitedSize} kW, ` +
      `Temp increase: ${tempIncrease.toFixed(1)} K, ` +
      `Reduction: ${reduction.toFixed(1)}%`
    );
  }
}
```

## Running the Examples

To run all examples:

```javascript
async function runAllExamples() {
  console.log('OpenFire WASM Fire Dynamics Examples\n');
  
  await officeFireAnalysis();
  console.log('\n');
  
  await parametricStudy();
  console.log('\n');
  
  await wallHeatingAnalysis();
  console.log('\n');
  
  await sprinklerActivationAnalysis();
  console.log('\n');
  
  await ventilationImpactStudy();
}

// Execute examples
runAllExamples().catch(console.error);
```

## Key Takeaways

1. **Temperature increases** are strongly dependent on fire size and compartment geometry
2. **Material properties** significantly affect heat transfer rates and thermal response
3. **Fire growth rates** determine the timeline for safety system activation
4. **Ventilation conditions** can dramatically impact fire development

## Next Steps

- [API Reference](../api/fire-dynamics-tools.html) - Detailed function documentation
- [Getting Started](getting-started.html) - Basic usage guide
- [Installation](../installation.html) - Setup instructions