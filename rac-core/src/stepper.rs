pub mod stepper {
    use crate::{ControlStatus, Motor, Rotation};

    #[derive(Debug, PartialEq, Eq)]
    pub enum StepModes {
        Full,
        Half,
        Quaeter,
        Micro8,
        Micro16,
        Micro32,
        Micro64,
        Micro128,
        Micro256,
    }

    pub struct Stepper {
        motor: Motor,
        mode: StepModes,
        steps_per_rev: u16,
        holding_torque: u32,
    }

    impl Stepper {
        pub fn new(p: u32, steps: u16, torque: u32) -> Self {
            Self {
                motor: Motor::new(p),
                mode: StepModes::Full,
                steps_per_rev: steps,
                holding_torque: torque,
            }
        }

        pub fn get_steps_per_rev(self) -> u16 {
            self.steps_per_rev
        }

        pub fn get_mode(self) -> StepModes {
            self.mode
        }

        pub fn get_holding_torque(self) -> u32 {
            self.holding_torque
        }
    }

    pub struct StepperControl {
        goal: u32,
        step: u16,
        rotation: Rotation,
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
    use crate::stepper::stepper::*;

    const POWER: u32 = 500;
    const STEPS: u16 = 200;
    const TORQUE: u32 = 35;

    #[test]
    fn new_stepper() {
        let stepper = Stepper::new(POWER, STEPS, TORQUE);
        // assert_eq!(&stepper.get_steps_per_rev(), STEPS);
        // assert_eq!(&stepper.get_holding_torque(), TORQUE);
        // assert_eq!(&stepper.get_mode(), StepModes::Full);
    }
}
