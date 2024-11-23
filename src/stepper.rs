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

    struct Stepper {
        status: Status,
        modes: u8,
        steps_per_rev: u16,
        reset: u8,
        fault: u8,
        holding_torque: u32,
    }

    struct StepperControl {
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
