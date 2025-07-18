# Robot Articulation Control

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/badge/build-WIP-yellow.svg)]()

**Robot Articulation Control (RAC)** is a modular, low-level framework for
embedded robotics. It provides the core building blocks to model, control, and
synchronize robotic articulations across microcontrollers (MCUs) and embedded
Linux systems.

Originally started as a motor control experiment, RAC has evolved into a
flexible system for distributed motion control, designed to scale from
individual joint controllers to complex articulated mechanisms. It enables
precise coordination between nodes (MCUs) and higher-level controllers (hosts),
with clean abstractions for control logic, communication protocols, and system
roles.


---

## Goals and Features

- Suitable for `no_std` embedded targets.
- Modular design for motors, encoders, and articulation nodes.
- Planned support for SPI and UART communication layers.
- Use with real-time motion systems (on MCUs or SBCs).

Target development board: **STM32F767ZI (Nucleo-144)**  

## Current status

- Designing independent modules for the controller, articulations, and communication nodes
- Creating an interface between the nodes and the controller
- Implementing a protocol to allow a reactive motion control
- Beginning real-world testing on embedded hardware

## Future

- Planned support for CAN and friends communication layers.
- Ready for use with real-time motion systems (on MCUs or SBCs).


## References
- Planned support for CAN and friends communication layers.
- Ready for use with real-time motion systems (on MCUs or SBCs).
---

## 📦 Usage

> ⚠️ The project is under active development.

<-- TODO: 
To use in your Rust project:
```toml
[dependencies]
robot-articulation-control = { git = "https://github.com/Resnog/robot-articulation-control" }
-->
