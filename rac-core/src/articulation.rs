use nalgebra::{self as na, RealField};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
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
enum JointType {
    Revolute,
    Prismatic,
    Spherical,
    Planar,
    Fixed,
    Continuous,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum JointParameters<T> {
    DoF1(na::SVector<T, 1>),
    DoF2(na::SVector<T, 2>),
    DoF3(na::SVector<T, 3>),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct JointLimits<T, const N: usize> {
    min: na::SVector<T, N>,
    max: na::SVector<T, N>,
}

impl<T, const N: usize> JointLimits<T, N> {
    pub fn new(min: na::SVector<T, N>, max: na::SVector<T, N>) -> Self {
        Self { min, max }
    }
}

type JointLimits1D<T> = JointLimits<T, 1>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mechanism<T> {
    Fixed,
    Joint(JointType, JointLimits1D<T>),
    Gimbal { dof: usize },
    Wheel(WheelType),
    OmniDrive { dof: usize },
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArticulationStatus {
    Idle,
    Moving,
    Fault,
    LimitTriggered,
    Calibrating,
    Homed,
    Offline,
}

pub enum ArticulationVariant {
    F32(Articulation<f32>),
    F64(Articulation<f64>),
}

/// An articulation represents a 1DOF actuator that will impact the
/// overall position of a robot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Articulation<T: RealField> {
    id: usize,
    pose: na::Isometry3<T>,
    role: Role,
    mechanism: Mechanism<T>,
    weight: T,
    q: T,
    status: ArticulationStatus,
}

impl<T: RealField + PartialOrd + Copy> Articulation<T> {
    /// Creates a new Articulation based on the 3D point in space and the Euler angles
    pub fn new(
        id: usize,
        role: Role,
        mechanism: Mechanism<T>,
        weight: T,
        location: na::Vector3<T>,
        rotation: na::UnitQuaternion<T>,
    ) -> Self {
        let translation = na::Translation3::from(location);
        Articulation {
            id,
            role,
            mechanism,
            weight,
            pose: na::Isometry3::<T>::from_parts(translation, rotation),
            q: T::zero(),
            status: ArticulationStatus::Idle,
        }
    }

    pub fn get_q(&self) -> T {
        return self.q;
    }

    pub fn get_status(&self) -> ArticulationStatus {
        self.status
    }

    pub fn set_q(&mut self, new_q: T) {
        self.q = new_q;
    }

    pub fn is_within_limits(&self) -> bool {
        let q = self.get_q();

        match &self.mechanism {
            Mechanism::Joint(_, limits) => q < limits.max[0] && q > limits.min[0],
            Mechanism::Wheel(_) => true,
            Mechanism::OmniDrive { .. } => true,
            Mechanism::Fixed => q == T::zero(),
            Mechanism::None => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use core::f32::consts::PI;

    use super::*;
    #[test]
    fn articulation_new() {
        let id = 1;
        let role = Role::Base;
        let joint_limits = JointLimits::new(
            na::SVector::<f32, 1>::new(180.0 / PI * 30.0),
            na::SVector::<f32, 1>::new(180.0 / PI * 120.0),
        );
        let mechanism = Mechanism::Joint(JointType::Revolute, joint_limits);
        let weight = 2.5;
        let q = (180.0 / PI) * 33.0;
        let location = na::Vector3::<f32>::new(2.5, 1.5, 0.0);
        let rotation = na::UnitQuaternion::from_euler_angles(PI / 2.0, PI / 2.0, PI / 2.0);
        let mut art = Articulation::new(id, role, mechanism, weight, location, rotation);

        assert_eq!(art.id, id);
        assert_eq!(art.role, role);
        assert_eq!(art.mechanism, mechanism);
        assert_eq!(art.weight, weight);
        assert_eq!(art.id, id);

        art.set_q(q);
        assert_eq!(art.get_q(), q);

        assert!(art.is_within_limits());
    }
}
