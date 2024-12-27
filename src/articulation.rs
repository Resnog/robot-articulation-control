use nalgebra::{self as na};

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
enum JointParameters {
    DoF1(na::SVector<f32, 1>),
    DoF2(na::SVector<f32, 2>),
    DoF3(na::SVector<f32, 3>),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct JointLimits<const N: usize> {
    min: na::SVector<f32, N>,
    max: na::SVector<f32, N>,
}

impl<const N: usize> JointLimits<N> {
    pub fn new(min: na::SVector<f32, N>, max: na::SVector<f32, N>) -> Self {
        Self { min, max }
    }

    pub fn is_within_limits(&self, art: &Articulation) -> bool {
        let q = art.get_q();
        q > self.min[0] && q < self.max[0]
    }
}

type JointLimits1D = JointLimits<1>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mechanism {
    Fixed,
    Joint(JointType, JointLimits1D),
    Gimbal { dof: usize },
    Wheel(WheelType),
    OmniDrive { dof: usize },
    None,
}

/// An articulation represents a 1DOF actuator that will impact the
/// overall position of a robot.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Articulation {
    id: u8,
    pose: na::Isometry3<f32>,
    role: Role,
    mechanism: Mechanism,
    weight: f32,
    q: f32,
}

impl Articulation {
    /// Creates a new Articulation based on the 3D point in space and the Euler angles
    pub fn new(
        id: u8,
        role: Role,
        mechanism: Mechanism,
        weight: f32,
        location: na::Vector3<f32>,
        rotation: na::UnitQuaternion<f32>,
    ) -> Self {
        let translation = na::Translation3::from(location);
        Articulation {
            id,
            role,
            mechanism,
            weight,
            pose: na::Isometry3::<f32>::from_parts(translation, rotation),
            q: 0.0,
        }
    }

    pub fn get_q(&self) -> f32 {
        return self.q;
    }

    pub fn set_q(&mut self, new_q: f32) {
        self.q = new_q;
    }

    pub fn is_within_limits(&self) -> bool {
        let q = self.get_q();

        match &self.mechanism {
            Mechanism::Joint(_, limits) => q < limits.max[0] && q > limits.min[0],
            Mechanism::Wheel(_) => true,
            Mechanism::OmniDrive { .. } => true,
            Mechanism::Fixed => q == 0.0,
            Mechanism::None => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

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
