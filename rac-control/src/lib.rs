#![cfg_attr(not(feature = "std"), no_std)]

pub mod encoder;
pub mod stepper;

enum Rotation {
    Clk,  // Clockwise
    Cclk, // Counter clockwise
}

enum ControlStatus {
    PositionControl,
    VelocityControl,
}

struct Motor {
    power: u32,
    status: Status,
}

impl Motor {
    pub fn new(p: u32) -> Self {
        Self {
            power: p,
            status: Status::Uninitialized,
        }
    }
}
