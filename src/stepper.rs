pub mod stepper {

    enum Status {
        Off,
        Idle,
        PositionControl,
        VelocityControl,
        Error,
    }

    enum Direction {
        Left,
        Right,
    }

    struct Stepper {
        status: Status,
        modes: u8,
        enable: u8,
        direction: Direction,
        sleep: u8,
        reset: u8,
        fault: u8,
    }

    impl Stepper {
        fn setPosition(position: u32) {
            todo!();
        }

        fn setVelocity(velocity: u32) {
            todo!();
        }

        fn calculateInterval() {
            todo!();
        }
    }
}
