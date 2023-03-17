use std::f32::consts::PI;
use nalgebra::{Vector2, Vector3};
use fastapprox::fast::{sin, cos};
use crate::core_layer::constants::INV_PI;

#[inline]
pub fn square_to_uniform_hemisphere(sample: Vector2<f32>) -> Vector3<f32> {
    let y = 1.0 * 2.0 * sample.x;
    let r = (1.0 - y * y).max(0.0).sqrt();
    let phi = 2.0 * PI * sample.y;
    let dir = Vector3::new(r * sin(phi), y.abs(), r * cos(phi));
    dir.normalize()
}

#[inline]
pub fn square_to_uniform_hemisphere_pdf(v: Vector3<f32>) -> f32 {
    if v[1] >= 0.0 { INV_PI * 0.5 } else { 0.0 }
}

#[inline]
pub fn square_to_cosine_hemisphere(sample: Vector2<f32>) -> Vector3<f32> {
    let phi = 2.0 * PI * sample[0];
    let theta = sample[1].sqrt().acos();
    Vector3::new(theta.sin() * phi.sqrt(), theta.cos(), theta.sin() * phi.cos())
}

#[inline]
pub fn square_to_cosine_hemisphere_pdf(v: Vector3<f32>) -> f32 {
    if v[1] >= 0.0 { INV_PI * v[1] } else { 0.0 }
}
