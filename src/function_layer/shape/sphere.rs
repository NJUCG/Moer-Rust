use super::shape::ShapeBase;
use crate::core_layer::constants::INV_PI;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{fetch_v3f, Bounds3, Medium, Ray, Shape, SurfaceInteraction, V3f};
use cgmath::{InnerSpace, Point3, Vector2, Zero};
use serde_json::Value;
use std::f32::consts::PI;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub shape: ShapeBase,
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn from_json(json: &Value) -> Self {
        let mut shape = ShapeBase::from_json(json);
        let center = fetch_v3f(json, "center", V3f::zero());
        let radius = json["radius"].as_f64().unwrap() as f32;
        shape.bounds3 = Bounds3::new(
            center - V3f::from([radius; 3]),
            center + V3f::from([radius; 3]),
        );
        Sphere {
            shape,
            center: Point3::from([center.x, center.y, center.z]),
            radius,
        }
    }
}

impl Transformable for Sphere {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Sphere {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)> {
        let origin = ray.origin;
        let dir = ray.direction;
        let o2c = self.center - origin;
        let b = o2c.dot(dir);
        let c = o2c.dot(o2c) - self.radius * self.radius;
        let delta = b * b - c;
        if delta <= 0.0 {
            return None;
        }
        let sqrt_delta = delta.sqrt();
        let t1 = b - sqrt_delta;
        let t2 = b + sqrt_delta;

        let mut hit = false;
        if ray.t_min <= t2 && t2 <= ray.t_max {
            ray.t_max = t2;
            hit = true;
        }
        if ray.t_min <= t1 && t1 <= ray.t_max {
            ray.t_max = t1;
            hit = true;
        }
        if !hit {
            return None;
        }
        // TODO 计算 u, v考虑旋转
        let normal = (ray.at(ray.t_max) - self.center).normalize();
        let cos_theta = normal.y;
        let u = if normal.z.abs() < 1e-4 {
            if normal.x > 0.0 {
                PI * 0.5
            } else {
                PI * 1.5
            }
        } else {
            (normal.x / normal.z).atan() + if normal.z < 0.0 { PI } else { 0.0 }
        };
        Some((0, u, cos_theta.acos()))
    }

    fn fill_intersection(
        &self,
        distance: f32,
        _prim_id: u64,
        u: f32,
        v: f32,
        medium: Option<Rc<dyn Medium>>,
        intersection: &mut SurfaceInteraction,
    ) {
        let normal = V3f::new(v.sin() * u.sin(), v.cos(), v.sin() * u.cos());
        intersection.normal = normal;

        let position = self.center + self.radius * normal;
        intersection.position = position;
        intersection.tex_coord = Vector2::new(u * INV_PI * 0.5, v * INV_PI);

        // TODO 计算交点的切线和副切线
        intersection.shape = Some(Rc::new(self.clone()));
        self._fill_intersection(distance, medium, intersection);
    }

    fn uniform_sample_on_surface(&self, _sample: Vector2<f32>) -> (SurfaceInteraction, f32) {
        todo!()
    }
}
