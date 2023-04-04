use cgmath::InnerSpace;
use nalgebra::{Matrix4, Point3, Vector4};
use crate::function_layer::{Bounds3, Ray, V3f};

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
        V3f::new(v4.x, v4.y, v4.z)
    }

    pub fn to_world_point(&self, v: &Point3<f32>) -> Point3<f32> {
        let v4 = v.to_homogeneous();
        let v4 = self.translate * self.rotate * self.scale * v4;
        Point3::from_homogeneous(v4).unwrap()
    }

    pub fn to_world_bounds3(&self, b: Bounds3) -> Bounds3 {
        let mut res = Bounds3::default();
        let ps = [&b.p_min, &b.p_max];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let p = Point3::new(ps[i].x, ps[j].y, ps[k].z);
                    res.expand(V3f::from(self.to_world_point(&p).coords.data.0[0]));
                }
            }
        }
        res
    }
    pub fn local_ray(&self, ray: &Ray) -> Ray {
        let origin = &ray.origin;
        let dir = ray.direction;
        let o = origin.to_homogeneous();
        let d =  nalgebra::Vector4::new(dir.x, dir.y, dir.z, 0.0);
        let o = self.inv_rotate * self.inv_translate * o;
        let d = self.inv_rotate * self.inv_translate * d;

        let origin = Point3::from_homogeneous(o).unwrap();
        let direction = V3f::from(d.xyz().data.0[0]);
        let mut res = Ray::new(origin, direction);
        res.t_min = ray.t_min;
        res.t_max = ray.t_max;
        res
    }
}

pub trait Transformable {
    fn transform(&self) -> &Transform;
}