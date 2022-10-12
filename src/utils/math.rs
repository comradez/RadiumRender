use vecmat::traits::Dot;

use super::{Vector3, Vector4, Matrix4};

pub fn extend(vector: &Vector3, extend_value: f64) -> Vector4 {
    Vector4::from([
        vector[0],
        vector[1],
        vector[2],
        extend_value,
    ])
}

pub fn xyz(vector: &Vector4) -> Vector3 {
    Vector3::from([
        vector[0],
        vector[1],
        vector[2],
    ])
}

pub fn transform_direction(matrix: &Matrix4, direction: &Vector3) -> Vector3 {
    let direction_extended = extend(direction, 1.);
    xyz(&matrix.dot(direction_extended)).normalize()
}

pub fn transform_point(matrix: &Matrix4, point: &Vector3) -> Vector3 {
    let point_extended = extend(point, 0.);
    xyz(&matrix.dot(point_extended)).normalize()
}