#![cfg_attr(not(feature = "std"), no_std)]

pub mod articulation;
mod encoder;
pub mod knode;
pub mod knode_protocol;
mod stepper;

enum Status {
    Uninitialized,
    Active,
    Inactive,
    Error(u32),
}
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
