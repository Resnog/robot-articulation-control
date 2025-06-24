# Robot Articulation Control

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/badge/build-WIP-yellow.svg)]()

**Robot Articulation Control** is a modular toolkit for controlling robot articulations at
a low level, designed to run both on MCUs and more powerful CPUs. Initially started as a motor
control experiment, this project has evolved to support more complete motion primitives and
system-level integration.

This is not a motion controller per se — it is the **foundation** for building motion controllers through
standardized articulations, graph representations, and communication protocols.

---

## ✨ Goals and Features

- Architecture-agnostic: portable across ARM, RISC-V, and x86.
- Suitable for `no_std` embedded targets.
- Modular design for motors, encoders, and articulation nodes.
- Planned support for CAN, SPI, and UART communication layers.
- Ready for use with real-time motion systems (on MCUs or SBCs).

Target development board: **STM32F767ZI (Nucleo-144)**  
Eventually aiming for multi-platform deployment.

## Current status

- Creating an interface between robot articulations and the planner
- Designing independent modules for the controller, articulations, and communication nodes
- Beginning real-world testing on embedded hardware

---

## 📦 Usage

> ⚠️ The project is under active development. Interfaces may change frequently.

To use in your Rust project:

```toml
[dependencies]
robot-articulation-control = { git = "https://github.com/Resnog/robot-articulation-control" }
