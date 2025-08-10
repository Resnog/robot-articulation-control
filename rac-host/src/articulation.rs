use nalgebra::{self as na, RealField};
use rac_core::articulation::CoreArticulation;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ArticulationStatus {
    Unidentified,
    Unresponsive,
    Active,
    Idle,
    Failure,
    Lost,
}

struct Articulation<T> {
    id: usize,
    pose: na::Isometry3<T>,
    status: ArticulationStatus,
    core: Option<CoreArticulation<T>>
}

impl<T: RealField> Articulation<T> {
    pub fn new(new_id: usize) -> Self {
        Self {
            id: new_id,
            pose: na::Isometry3::identity(),
            status: ArticulationStatus::Unidentified,
            core: None,
        }
    }

    pub fn get_pose(&self) -> &na::Isometry3<T>{
        &self.pose
    }

    pub fn get_status(&self) -> ArticulationStatus{
        self.status
    }

    pub fn get_core(&self) -> Option<&CoreArticulation<T>>{
        self.core.as_ref()
    }
}
#[cfg(test)]
mod test {
    use core::f32::consts::PI;
    use rac_core::articulation::{CoreArticulation, JointLimits, JointType, Mechanism, Role};

    use crate::articulation::{self, *};

    #[test]
    fn core_articulation_new() {
        let id = 1;
        let role = Role::Base;
        let joint_limits = JointLimits::new((180.0 / PI) * 30.0, (180.0 / PI) * 120.0);
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

    #[test]
    fn host_articulation_new() {
        let art = Articulation::<f64>::new(1);

        assert_eq!(art.get_status(), articulation::ArticulationStatus::Unidentified);
        assert_eq!(art.get_core(), None);
        assert_eq!(art.get_pose(), &na::Isometry3::identity());
    }
}
