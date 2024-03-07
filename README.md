# RustyChaosCraftor
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) [![Deployment](https://github.com/tomtuamnuq/rusty-chaos-craftor/actions/workflows/pages.yml/badge.svg)](https://github.com/tomtuamnuq/rusty-chaos-craftor/actions/workflows/pages.yml) 
[![Tests](https://github.com/tomtuamnuq/rusty-chaos-craftor/actions/workflows/test.yml/badge.svg)](https://github.com/tomtuamnuq/rusty-chaos-craftor/actions/workflows/test.yml) 
[Live App ![Live App](./assets/icon_ios_touch_192.png)](https://tomtuamnuq.github.io/rusty-chaos-craftor/) 
## Overview
RustyChaosCraftor is an interactive exploration tool for visualizing dynamics in chaotic systems. Driven by a fascination with the complex patterns emerging from simple mathematical constructs, this project aims to provide profound insights into chaos theory through highly customizable visualization techniques.

### Core Functionality
- Multidimensional real-time visualization of chaotic functions, fractals, and particle systems.

### Motivation
- A personal journey into chaos theory and mathematical aesthetics.
- The desire to learn Rust and explore WebAssembly (WASM).

### Goal
- To facilitate the exploration of chaos theory through detailed visualizations such as bifurcation diagrams and colormaps.
- To examine the impact of initial conditions and minor parameter variations on chaotic systems.
- To analyze various types of fractals, attractors, and number systems within chaos theory.


## Features
- **Efficient Chaos**: 1D, 2D, 3D, and 4D discrete chaotic maps implemented in Rust, generating a diverse array of patterns.
- **Dynamic Systems**: Live solutions of 2D, 3D, and 4D ordinary differential equations, creating visually stunning attractors.
- **Particle Simulation**: An interactive N-Body problem solver in 2D and 3D, exploring attractive and repulsive forces.
- **Fractal Generation**: Support for four number systems (algebraic rings) including Complex, Dual, Perplex numbers, and Quaternions.
- **Parameter Exploration**: Bifurcation analysis for all features, offering deep dives into system dynamics.
- **Initial Condition Analysis**: A wide range of initial distributions to study the effects on system behavior.
- **WASM Web Application**: Easily accessible web application with the option to compile natively across all platforms.


## Installation
This project was tested with stable version ˋrustc 1.76.0ˋ of the Rust toolchain. Install it with `rustup install 1.76.0`.

Sure, I can help you with that. Here's how you can extend the README with the information about how the app saves its state:

### Native
For local testing on native platforms, execute `cargo run --release`. Linux users must install necessary libraries for `eframe` with:

```bash
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

Fedora Rawhide users should run:

```bash
dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel
```

The app configuration is automatically saved in the user's data directory on shutdown and every 30 seconds during autosave. The exact location of the file depends on the operating system. For example, on Linux, the file is located at `~/.local/share/rustychaoscraftor/app.ron`.

### Web
For web deployment, compile to [WASM](https://en.wikipedia.org/wiki/WebAssembly) using [Trunk](https://trunkrs.dev/):
1. Add the WASM target: `rustup target add wasm32-unknown-unknown`.
2. Install Trunk: `cargo install --locked trunk`.
3. Serve locally with `trunk serve --port=8043`, automatically rebuilding on changes.
4. Access the app at `http://127.0.0.1:8043/index.html#dev`. The `#dev`deactivates caching to view a currently developed version.

The app configuration is saved in the browser's local storage every 30 seconds. Clearing website data may resolve serialization issues.

## Community and Support
To contribute or seek support, open an issue in this repository. Questions, feedback, and contributions are welcome. For any inquiries, please feel free to open an issue.

## Acknowledgments
Special thanks to the following Rust crates that made this project possible:

- `egui`: The intuitive frontend framework. [Repository](https://github.com/emilk/egui)
- `egui_plotter`: Integration of Plotters 3D chart into the egui main panel. [Repository](https://docs.rs/egui-plotter)
- `plotters`: The powerful backend for 3D plotting capabilities. [Repository](https://github.com/plotters-rs)
- `ode_solvers`: For simulating particle systems and chaotic differential equations. [Repository](https://github.com/srenevey/ode-solvers)
