#![allow(dead_code)]

use crate::core_layer::constants::INV_PI;
use crate::function_layer::V3f;
use cgmath::{InnerSpace, Vector2};
use std::f32::consts::PI;

#[inline]
pub fn square_to_uniform_hemisphere(sample: Vector2<f32>) -> V3f {
    let y = 1.0 * 2.0 * sample.x;
    let r = (1.0 - y * y).max(0.0).sqrt();
    let phi = 2.0 * PI * sample.y;
    let dir = V3f::new(r * phi.sin(), y.abs(), r * phi.cos());
    dir.normalize()
}

#[inline]
pub fn square_to_uniform_hemisphere_pdf(v: V3f) -> f32 {
    if v[1] >= 0.0 {
        INV_PI * 0.5
    } else {
        0.0
    }
}

#[inline]
pub fn square_to_cosine_hemisphere(sample: Vector2<f32>) -> V3f {
    let phi = 2.0 * PI * sample[0];
    let theta = sample[1].sqrt().acos();
    V3f::new(
        theta.sin() * phi.sin(),
        theta.cos(),
        theta.sin() * phi.cos(),
    )
}

#[inline]
pub fn square_to_cosine_hemisphere_pdf(v: V3f) -> f32 {
    if v[1] >= 0.0 {
        INV_PI * v[1]
    } else {
        0.0
    }
}
