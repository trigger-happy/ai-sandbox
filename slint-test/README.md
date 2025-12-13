# Slint Test Project - Browser Application

An experimental Rust project using Slint UI framework compiled to WebAssembly for testing various UI components **in the browser**.

## Overview

This project demonstrates:
- **Slint UI Framework** - Running in the browser via WebAssembly
- **Axum Web Server** - Serving the application
- **Various UI Components** - Comprehensive test suite of Slint widgets
- **WebAssembly** - Modern web technology for high-performance applications

## Architecture

The application consists of two parts:
1. **Slint UI (WebAssembly)** - Runs in the browser, renders on HTML5 Canvas
2. **Web Server (Axum)** - Serves HTML and wasm files

## UI Components

The Slint interface includes three tabs showcasing different component types:

### Tab 1: Buttons & Text
- Multiple button controls with click handlers
- Counter functionality (increment/reset)
- Text input field (LineEdit) with real-time display
- CheckBox with state binding and visual feedback

### Tab 2: Sliders & Inputs
- Slider control (0-100 range) with value binding
- Visual progress bar linked to slider value
- SpinBox for numeric input
- ComboBox with multiple selectable options

### Tab 3: Visual Elements
- Color palette (5 colored rectangles)
- Text style demonstrations (normal, bold, large, colored)
- Progress indicator dynamically linked to slider value

## Features

- **Property Binding** - Two-way data binding between UI elements
- **Callbacks** - Event handlers for button clicks
- **State Management** - Reactive properties that update the UI
- **Status Messages** - Real-time feedback for user interactions
- **Browser-Based** - No installation required, runs in any modern browser

## Prerequisites

- Rust (latest stable)
- `wasm-pack` - WebAssembly build tool
- `wasm32-unknown-unknown` target

### Installing Prerequisites

```bash
# Install wasm-pack
cargo install wasm-pack

# Add wasm target
rustup target add wasm32-unknown-unknown
```

## Building

### 1. Build the WebAssembly Library

```bash
wasm-pack build --target web --out-dir pkg
```

This generates the wasm files in the `pkg/` directory:
- `slint_test_bg.wasm` - WebAssembly binary (7.8MB)
- `slint_test.js` - JavaScript wrapper (82KB)

### 2. Build the Web Server

```bash
cargo build --bin server --release
```

## Running

Start the web server:

```bash
cargo run --bin server --release
```

Or run the pre-built binary:

```bash
./target/release/server
```

When running, you'll see:
```
🚀 Starting Slint Test Web Server...
✓ Web server started at http://127.0.0.1:3000
📊 Health check available at http://127.0.0.1:3000/health
🌐 Open http://127.0.0.1:3000 in your browser to see the Slint UI

Press Ctrl+C to stop
```

## Accessing the Application

Open your web browser and navigate to:
```
http://127.0.0.1:3000
```

The Slint UI will render in the browser with all interactive components!

## Endpoints

- `http://127.0.0.1:3000/` - Main application page
- `http://127.0.0.1:3000/health` - Health check endpoint
- `http://127.0.0.1:3000/pkg/` - WebAssembly files

## Dependencies

### Runtime
- `slint` (1.9) - UI framework
- `wasm-bindgen` (0.2) - Rust/Wasm interop
- `console_error_panic_hook` (0.1) - Better error messages in browser

### Server (non-wasm)
- `axum` (0.7) - Web server framework
- `tokio` (1.x) - Async runtime
- `tower-http` (0.6) - Static file serving

### Build
- `slint-build` (1.9) - Build-time UI compilation
- `wasm-pack` - WebAssembly packaging

## Project Structure

```
slint-test/
├── src/
│   ├── lib.rs           # Wasm library (UI logic)
│   └── server.rs        # Web server
├── ui/
│   └── appwindow.slint  # UI definition
├── pkg/                 # Generated wasm files (after build)
│   ├── slint_test.js
│   └── slint_test_bg.wasm
├── build.rs             # Build script
├── Cargo.toml           # Dependencies
└── README.md            # This file
```

## Browser Compatibility

Works in all modern browsers that support:
- WebAssembly
- ES6 modules
- Canvas API

Tested on:
- Chrome 61+
- Firefox 60+
- Safari 11+
- Edge 79+

## Development Notes

- The wasm build is configured without `wasm-opt` to avoid external dependencies
- The application uses Slint's femtovg renderer for canvas rendering
- State management is handled entirely in wasm, with no backend communication required
- All UI interactions happen client-side in the browser

## Notes

This is an experimental project for testing Slint UI components in a browser environment using WebAssembly. It demonstrates that Slint applications can run anywhere modern web browsers are available, without platform-specific installations.

## Troubleshooting

### wasm-pack fails to build
- Ensure you have the wasm32-unknown-unknown target installed
- Check that wasm-pack version is compatible (0.13+)

### Server doesn't start
- Check if port 3000 is already in use
- Ensure the `pkg/` directory exists with wasm files

### Blank page in browser
- Open browser console (F12) to check for errors
- Verify wasm files are accessible at `/pkg/slint_test.js` and `/pkg/slint_test_bg.wasm`
- Check that the server is serving files correctly
