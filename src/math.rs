// 3D space representation of a point
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

// TODO - Trigonometric functions for small MCUs
// TODO - Try CORDIC
// TODO - Use a LUP

// TODO - Matrix operations
// TODO - 3D transformations
struct Transform([[f32; 4]; 4]);

#[cfg(test)]
mod test {

    use crate::math::*;
    #[test]
    fn new_point() {
        let p = Point {
            x: 42.42,
            y: 42.42,
            z: 42.42,
        };
        assert_eq!(p.x, 42.42);
        assert_eq!(p.y, 42.42);
        assert_eq!(p.z, 42.42);
    }
}
