# Slint Test Project

An experimental Rust project combining Slint UI framework with a web server for testing various UI components.

## Overview

This project demonstrates:
- **Slint UI Framework** - A native desktop UI toolkit for Rust
- **Axum Web Server** - Running concurrently with the desktop application
- **Various UI Components** - Comprehensive test suite of Slint widgets

## Architecture

The application runs two concurrent components:
1. **Desktop UI (Slint)** - Main window with interactive components
2. **Web Server (Axum)** - HTTP server on port 3000

## UI Components Tested

The Slint interface includes three tabs showcasing different component types:

### Tab 1: Buttons & Text
- Multiple button controls with click handlers
- Counter functionality (increment/reset)
- Text input field (LineEdit) with real-time display
- CheckBox with state binding and visual feedback

### Tab 2: Sliders
- Slider control (0-100 range) with value binding
- Visual progress bar linked to slider value
- SpinBox for numeric input
- ComboBox with multiple selectable options

### Tab 3: Visual
- Color palette (5 colored rectangles)
- Text style demonstrations (normal, bold, large, colored)
- Progress indicator dynamically linked to slider value

## Features

- **Property Binding** - Two-way data binding between UI elements
- **Callbacks** - Event handlers for button clicks
- **State Management** - Reactive properties that update the UI
- **Status Messages** - Real-time feedback for user interactions
- **Concurrent Execution** - Desktop UI and web server running simultaneously

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Running

```bash
# Run the application
cargo run --release
```

When running:
- Desktop window will appear showing the Slint UI
- Web server starts at `http://127.0.0.1:3000`
- Visit the web server URL in your browser for application info
- Health check endpoint: `http://127.0.0.1:3000/health`

## Dependencies

- `slint` (1.9) - UI framework
- `slint-build` (1.9) - Build-time UI compilation
- `axum` (0.7) - Web server framework
- `tokio` (1.x) - Async runtime

## Project Structure

```
slint-test/
├── src/
│   └── main.rs           # Main application code
├── ui/
│   └── appwindow.slint   # UI definition
├── build.rs              # Build script for Slint compilation
├── Cargo.toml            # Dependencies
└── README.md             # This file
```

## Notes

This is an experimental project for testing Slint UI components and exploring the integration of native UI with web server functionality in Rust.
