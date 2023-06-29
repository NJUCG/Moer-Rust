use std::ops::{Add, Mul};
use crate::function_layer::V3f;

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 || a == 0.0 {
        return None;
    } else if discr == 0.0 {
        return Some((-0.5 * b / a, -0.5 * b / a));
    }
    let q = if b > 0.0 {
        -0.5 * (b + discr.sqrt())
    } else {
        -0.5 * (b - discr.sqrt())
    };
    let x0 = q / a;
    let x1 = c / q;
    if x0 < x1 {
        Some((x0, x1))
    } else {
        Some((x1, x0))
    }
}

pub fn lerp<T>(alpha: f32, p1: T, p2: T) -> T
    where T: Mul<f32, Output=T> + Add<T, Output=T> {
    p1 * alpha + p2 * (1.0 - alpha)
}

pub fn coordinate_system(v1: V3f, v2: &mut V3f, v3: &mut V3f) {
    *v2 = if v1.x.abs() > v1.y.abs() {
        V3f::new(-v1.z, 0.0, v1.x) / (v1.x * v1.x + v1.z * v1.z).sqrt()
    } else { V3f::new(0.0, v1.z, -v1.y) / (v1.y * v1.y + v1.z * v1.z).sqrt() };
    *v3 = v1.cross(*v2);
}

pub fn spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32,
                           x: V3f, y: V3f, z: V3f) -> V3f {
    sin_theta * phi.cos() * x + sin_theta * phi.sin() * y + cos_theta * z
}