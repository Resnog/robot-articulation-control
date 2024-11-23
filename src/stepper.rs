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
        stepsPerRev: u16,
        reset: u8,
        fault: u8,
        holdingTorque: u32,
    }

    struct StepperControl {
        goal: u32,
        step: u16,
        direction: Direction,
        status: ControlStatus,
    }

    impl StepperControl {
        fn calculateInterval() {
            todo!();
        }
    }
}
