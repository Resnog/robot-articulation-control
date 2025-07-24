use nalgebra::{self as na, RealField};
use rac_core::articulation::{Mechanism, Role};

#[derive(Debug, Clone, Copy, PartialEq)]
enum JointParameters<T> {
    DoF1(na::SVector<T, 1>),
    DoF2(na::SVector<T, 2>),
    DoF3(na::SVector<T, 3>),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArticulationStatus {
    Idle,
    Moving,
    Fault,
    Processing,
    Online,
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

}

#[cfg(test)]
mod test {
    use core::f32::consts::PI;
    use rac_core::articulation::{JointLimits, CoreArticulation, Mechanism, JointType};

    use super::*;
    #[test]
    // TODO - FIX CoreArticuation testing
    fn CoreArticulation_new() {
        let id = 1;
        let role = Role::Base;
        let joint_limits = JointLimits::new(
                (180.0 / PI) * 30.0,
                (180.0 / PI) * 120.0,
            );
        let mechanism = Mechanism::Joint(JointType::Revolute, joint_limits);
        let weight = 2.5;
        let q = (180.0 / PI) * 55.0;
        let mut art = CoreArticulation::new(id, role, mechanism, weight);

        assert_eq!(art.get_id(), id);
        assert_eq!(art.get_role(), role);
        assert_eq!(art.get_mechanism(), mechanism);
        assert_eq!(art.get_weight(), weight);

        art.set_q(q);
        assert_eq!(art.get_q(), q);

        assert!(art.is_within_limits());
    }
}
