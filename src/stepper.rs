pub mod stepper {

    enum Status {
        Off,
        Idle,
        Error,
    }

    enum ControlStatus {
        PositionControl,
        VelocityControl,
    }

    enum Direction {
        Left,
        Right,
    }

    pub struct Stepper {
        status: Status,
        modes: u8,
        pub steps_per_rev: u16,
        reset: u8,
        fault: u8,
        pub holding_torque: u32,
    }

    impl Stepper {
        pub fn new(steps: u16, torque: u32) -> Self {
            Self {
                status: Status::Off,
                modes: 0,
                steps_per_rev: steps,
                reset: 0,
                fault: 0,
                holding_torque: torque,
            }
        }
    }

    pub struct StepperControl {
        goal: u32,
        step: u16,
        direction: Direction,
        status: ControlStatus,
    }

    impl StepperControl {
        fn calculate_interval() {
            todo!();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::stepper::stepper::Stepper;
    #[test]
    fn new_stepper() {
        let stepper = Stepper::new(200, 35);
        assert!(stepper.steps_per_rev == 200);
        assert!(stepper.holding_torque == 35);
    }
}
