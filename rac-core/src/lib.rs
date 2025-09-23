#![no_std]

pub mod articulation;
pub mod encoder;
pub mod knode;
pub mod stepper;

type Vector<T, const N: usize> = [T; N];

#[derive(PartialEq)]
pub enum Status {
    Uninitialized,
    Initializing,
    Active,
    Inactive,
    MsgSendSuccessfully,
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
