use crate::Vector;
use num_traits::Zero;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Base,
    Link,
    Effector,
    Locomotor,
    Sensor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WheelType {
    Differential,
    Steering,
    Mechanum,
    Omni,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointType {
    Revolute,
    Prismatic,
    Spherical,
    Planar,
    Fixed,
    Continuous,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JointLimits<T> {
    min: T,
    max: T,
}

impl<T: Copy> JointLimits<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub fn get_min(&self) -> T {
        self.min
    }

    pub fn get_max(&self) -> T {
        self.max
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mechanism<T> {
    Fixed,
    Joint(JointType, JointLimits<T>),
    Gimbal { dof: usize }, // Future implementation
    Wheel(WheelType),
    OmniDrive { dof: usize }, // Future implementation
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreArticulationStatus {
    Idle,
    Moving,
    Fault,
    LimitTriggered,
    Calibrating,
    Homed,
    Offline,
}

pub enum CoreArticulationVariant {
    F32(CoreArticulation<f32>),
    F64(CoreArticulation<f64>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoreArticulation<T> {
    id: usize,
    role: Role,
    mechanism: Mechanism<T>,
    weight: T,
    q: T,
    status: CoreArticulationStatus,
}

impl<T: PartialOrd + Copy + Zero> CoreArticulation<T> {
    pub fn new(id: usize, role: Role, mechanism: Mechanism<T>, weight: T) -> Self {
        CoreArticulation {
            id,
            role,
            mechanism,
            weight,
            q: T::zero(),
            status: CoreArticulationStatus::Idle,
        }
    }

    pub fn get_id(&self) -> usize {
        return self.id;
    }

    pub fn get_role(&self) -> Role {
        return self.role;
    }

    pub fn get_mechanism(&self) -> Mechanism<T> {
        return self.mechanism;
    }

    pub fn get_weight(&self) -> T {
        return self.weight;
    }

    pub fn get_q(&self) -> T {
        return self.q;
    }

    pub fn get_status(&self) -> CoreArticulationStatus {
        self.status
    }

    pub fn set_q(&mut self, new_q: T) {
        self.q = new_q;
    }

    pub fn is_within_limits(&self) -> bool {
        let q = self.get_q();

        match &self.mechanism {
            Mechanism::Joint(_, limits) => q < limits.get_max() && q > limits.get_min(),
            Mechanism::Wheel(_) => true,
            //Mechanism::OmniDrive { .. } => true,
            Mechanism::Fixed => q == T::zero(),
            Mechanism::None => true,
            _ => false,
        }
    }
}
