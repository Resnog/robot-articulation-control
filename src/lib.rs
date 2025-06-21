mod articulation;
mod encoder;
mod stepper;

enum Status {
    Off,
    Active,
    Inactive,
    Error,
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
            status: Status::Off,
        }
    }
}
