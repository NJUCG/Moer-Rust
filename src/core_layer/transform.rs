#![allow(dead_code)]

use nalgebra::{Matrix4, Point3, Vector4};
use crate::function_layer::V3f;

type M4f = Matrix4<f32>;

#[derive(Clone, Debug)]
pub struct Transform {
    pub translate: M4f,
    pub inv_translate: M4f,
    pub rotate: M4f,
    pub inv_rotate: M4f,
    pub scale: M4f,
    pub inv_scale: M4f,
}

impl Default for Transform {
    fn default() -> Self {
        Transform::identity()
    }
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            translate: M4f::identity(),
            inv_translate: M4f::identity(),
            rotate: M4f::identity(),
            inv_rotate: M4f::identity(),
            scale: M4f::identity(),
            inv_scale: M4f::identity(),
        }
    }
    pub fn new(translate: M4f, rotate: M4f, scale: M4f) -> Self {
        let inv_rotate = rotate.transpose();
        let mut inv_translate = M4f::identity();
        let mut inv_scale = M4f::identity();
        for i in 0..3 {
            inv_translate[(i, 3)] = -translate[(i, 3)];
            inv_scale[(i, i)] = 1.0 / scale[(i, i)];
        }
        Self { translate, inv_translate, rotate, inv_rotate, scale, inv_scale }
    }

    pub fn translation(offset: &V3f) -> M4f {
        let mut mat = M4f::identity();
        for i in 0..3 {
            mat[(i, 3)] = offset[i];
        }
        mat
    }

    pub fn rotation(axis: &V3f, radian: f32) -> M4f {
        let mut mat = M4f::identity();
        let a = axis.normalize();
        let sin_theta = radian.sin();
        let cos_theta = radian.cos();
        mat[(0, 0)] = a[0] * a[0] + (1.0 - a[0] * a[0]) * cos_theta;
        mat[(0, 1)] = a[0] * a[1] * (1.0 - cos_theta) - a[2] * sin_theta;
        mat[(0, 2)] = a[0] * a[2] * (1.0 - cos_theta) + a[1] * sin_theta;

        mat[(1, 0)] = a[0] * a[1] * (1.0 - cos_theta) + a[2] * sin_theta;
        mat[(1, 1)] = a[1] * a[1] + (1.0 - a[1] * a[1]) * cos_theta;
        mat[(1, 2)] = a[1] * a[2] * (1.0 - cos_theta) - a[0] * sin_theta;

        mat[(2, 0)] = a[0] * a[2] * (1.0 - cos_theta) - a[1] * sin_theta;
        mat[(2, 1)] = a[1] * a[2] * (1.0 - cos_theta) + a[0] * sin_theta;
        mat[(2, 2)] = a[2] * a[2] + (1.0 - a[2] * a[2]) * cos_theta;

        mat
    }

    pub fn scalation(scale: &V3f) -> M4f {
        let mut mat = M4f::identity();
        for i in 0..3 {
            mat[(i, i)] = scale[i];
        }
        mat
    }

    pub fn to_world_vec(&self, v: &V3f) -> V3f {
        let v4 = Vector4::new(v[0], v[1], v[2], 0.0);
        let v4 = self.translate * self.rotate * self.scale * v4;
        V3f::new(v4[0], v4[1], v4[2])
    }

    pub fn to_world_point(&self, v: &Point3<f32>) -> Point3<f32> {
        let v4 = Vector4::new(v[0], v[1], v[2], 1.0);
        let mut v4 = self.translate * self.rotate * self.scale * v4;
        v4 /= v4[3];
        let p = Point3::from(v4.xyz());
        p
    }
}

pub trait Transformable {
    fn transform(&self) -> &Transform;
}