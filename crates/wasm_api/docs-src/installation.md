---
title: "Installation"
group: "Getting Started"
category: "Setup"
---

# Installation

Learn how to install and set up OpenFire WASM for your JavaScript/TypeScript projects.

## Prerequisites

- Node.js 14 or higher
- npm or yarn package manager
- Modern web browser with WebAssembly support

## Installing from npm

Install the OpenFire WASM package using npm:

```bash
npm install openfire-wasm
```

Or using yarn:

```bash
yarn add openfire-wasm
```

## Browser Setup

### ES Modules (Recommended)

```javascript
import init, { get_fire_dynamics_tools } from 'openfire-wasm';

async function main() {
  // Initialize the WASM module
  await init();
  
  // Now you can use the API
  const fdt = get_fire_dynamics_tools();
  console.log('OpenFire WASM initialized successfully!');
}

main();
```

### With a Bundler (Webpack, Vite, etc.)

Most modern bundlers support WebAssembly out of the box:

```javascript
import init, { get_fire_dynamics_tools } from 'openfire-wasm';

// Initialize once at app startup
init().then(() => {
  const fdt = get_fire_dynamics_tools();
  // Your application code here
});
```

## Node.js Setup

```javascript
const { get_fire_dynamics_tools } = require('openfire-wasm');

// In Node.js, initialization is automatic
const fdt = get_fire_dynamics_tools();
console.log('OpenFire WASM ready for Node.js!');
```

## Verification

Test your installation with this simple example:

```javascript
import init, { calculate_circle_area } from 'openfire-wasm';

await init();
const area = calculate_circle_area(5.0);
console.log(`Circle area: ${area} m²`); // Should output: Circle area: 78.54 m²
```

## Troubleshooting

### WASM Loading Issues

If you encounter WebAssembly loading issues:

1. **MIME Type**: Ensure your server serves `.wasm` files with the correct MIME type:
   ```
   application/wasm
   ```

2. **CORS**: For cross-origin requests, ensure proper CORS headers are set.

3. **Browser Support**: Verify WebAssembly support in your target browsers.

### Build Integration

For build tools that need explicit WASM handling, you may need to:

- Copy `.wasm` files to your output directory
- Configure your bundler to handle WASM imports
- Set up proper asset paths for production builds

## Next Steps

- [Getting Started Guide](user-guide/getting-started.html) - Basic usage and examples
- [API Reference](api/fire-dynamics-tools.html) - Complete function reference