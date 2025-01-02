// 3D space representation of a point
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(pos_x: f32, pos_y: f32, pos_z: f32) -> Self {
        Point {
            x: pos_x,
            y: pos_y,
            z: pos_z,
        }
    }
}

// TODO - Trigonometric functions for small MCUs
// TODO - Try CORDIC
// TODO - Use a LUP

// TODO - Matrix operations

// TODO - 3D transformations

const TF_SIZE: usize = 4;
type Tf = [[f32; TF_SIZE]; TF_SIZE];

struct Transform {
    tf: Tf,
}

impl Transform {
    pub fn new(alpha: f32, beta: f32, gamma: f32, point: Point) -> Self {
        let mut tf = Transform {
            tf: [[0.0; TF_SIZE]; TF_SIZE],
        };
        tf.update_tf_rot(alpha, beta, gamma);
        tf.update_tf_pose(point);
    }
    /// This method updates the tf values based on the Row-Yaw-Pitch angles using the
    /// std implementation of sin and cos.

    /// This function is a WIP, there is no optimization and its only used internally by the
    /// Transform.
    fn update_tf_rot(&mut self, alpha: f32, beta: f32, gamma: f32) {
        // 1st Row
        self.tf[0][0] = f32::cos(gamma) * f32::cos(beta);
        self.tf[0][1] =
            f32::cos(gamma) * f32::sin(beta) * f32::sin(alpha) - f32::sin(gamma) * f32::cos(alpha);
        self.tf[0][2] =
            f32::cos(gamma) * f32::sin(beta) * f32::cos(alpha) + f32::sin(gamma) * f32::sin(alpha);
        // 2nd Row
        self.tf[1][0] = f32::sin(gamma) * f32::cos(beta);
        self.tf[1][1] =
            f32::sin(alpha) * f32::sin(beta) * f32::sin(gamma) + f32::cos(alpha) * f32::cos(gamma);
        self.tf[1][2] =
            f32::sin(alpha) * f32::sin(beta) * f32::sin(gamma) - f32::cos(gamma) * f32::sin(alpha);
        // 3rd Row
        self.tf[2][0] = (-1.0) * f32::sin(beta);
        self.tf[2][1] = f32::sin(alpha) * f32::cos(beta);
        self.tf[2][2] = f32::cos(alpha) * f32::cos(beta);
    }

    fn update_tf_pose(&mut self, point: Point) {
        self.tf[0][3] = point.x;
        self.tf[1][3] = point.y;
        self.tf[2][3] = point.z;
    }
}

#[cfg(test)]
mod test {

    use core::f32;

    const ANGLE45DEG: f32 = f32::consts::PI / 2.0;

    use crate::math::*;
    #[test]
    fn new_point() {
        let p = Point::new(42.42, 42.42, 42.42);
        assert_eq!(p.x, 42.42);
        assert_eq!(p.y, 42.42);
        assert_eq!(p.z, 42.42);
    }

    #[test]
    fn new_tf() {
        let alpha = ANGLE45DEG;
        let beta = ANGLE45DEG;
        let gamma = ANGLE45DEG;
        let p = Point::new(42.42, 42.42, 42.42);
        let tf = Transform::new(ANGLE45DEG, ANGLE45DEG, ANGLE45DEG, p);
        // TODO check the rot mat
        // TODO check the translation mat
    }
}
