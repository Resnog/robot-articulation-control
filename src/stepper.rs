
pub mod stepper {
 
    enum Status {
        Off,
        Idle,
        Position_control,
        Velocity_control,
        Error,
    }

    enum Direction {
        Left,
        Right,
    }

    struct Stepper{
        status: Status,
        modes : u8,
        enable : u8,
        direction : Direction,
        sleep : u8,
        reset : u8,
        fault : u8,
    }

    impl Stepper {
        fn setMode(modes:u8) {
            Self.modes = modes;
        }

        fn setDirection(direction:Direction) {
            Self.direction = direction;
        }
    }
}

