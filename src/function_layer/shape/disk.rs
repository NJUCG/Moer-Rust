use super::shape::ShapeBase;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Bounds3, SurfaceInteraction, Ray, Shape, V3f};
use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector2};
use serde_json::Value;
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Clone)]
pub struct Disk {
    pub shape: ShapeBase,
    radius: f32,
    inner_radius: f32,
    phi_max: f32,
}

impl Disk {
    pub fn from_json(json: &Value) -> Self {
        let radius = json["radius"].as_f64().unwrap_or(1.0) as f32;
        let inner_radius = json["inner_radius"].as_f64().unwrap_or(0.0) as f32;
        let phi_max = json["phi_max"].as_f64().unwrap_or(2.0 * PI) as f32;
        let mut shape = ShapeBase::from_json(json);
        let bounds3 = Bounds3::new(
            V3f::new(-radius, -radius, 0.0),
            V3f::new(radius, radius, 0.0),
        );
        shape.bounds3 = shape.transform.to_world_bounds3(bounds3);

        Self {
            shape,
            radius,
            inner_radius,
            phi_max,
        }
    }
}

impl Transformable for Disk {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Disk {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)> {
        let trans = self.transform();
        let local_ray = trans.local_ray(ray);
        if local_ray.direction.z == 0.0 {
            return None;
        }
        let t0 = -local_ray.origin.z / local_ray.direction.z;
        if t0 <= local_ray.t_min || t0 >= local_ray.t_max {
            return None;
        }

        let p = local_ray.at(t0);
        let r = p.to_vec().magnitude();
        if r < self.inner_radius || r > self.radius {
            return None;
        }

        let mut its_phi = (p.y / p.x).atan();
        if its_phi < 0.0 {
            its_phi += PI as f32;
        }
        if p.y < 0.0 {
            its_phi += PI as f32;
        }

        if its_phi > self.phi_max {
            return None;
        }

        ray.t_max = t0;
        let u = its_phi / self.phi_max;
        let v = (r - self.inner_radius) / (self.radius - self.inner_radius);
        Some((self.geometry_id(), u, v))
    }

    fn fill_intersection(
        &self,
        distance: f32,
        _prim_id: u64,
        u: f32,
        v: f32,
        intersection: &mut SurfaceInteraction,
    ) {
        let trans = self.transform();
        let normal = V3f::new(0.0, 0.0, 1.0);
        intersection.normal = trans.to_world_vec(&normal);

        let r = v * (self.radius - self.inner_radius) + self.inner_radius;
        let phi = u * self.phi_max;
        let position = Point3::new(r * phi.cos(), r * phi.sin(), 0.0);
        intersection.position = trans.to_world_point(&position);

        intersection.shape = Some(Rc::new(self.clone()));
        intersection.distance = distance;
        intersection.tex_coord = Vector2::new(u, v);

        // 计算交点的切线和副切线
        let mut tangent = V3f::new(1.0, 0.0, 0.0);
        if tangent.dot(intersection.normal).abs() > 0.9 {
            tangent = V3f::new(0.0, 1.0, 0.0);
        }
        let bitangent = tangent.cross(intersection.normal).normalize();
        tangent = intersection.normal.cross(bitangent).normalize();
        intersection.tangent = tangent;
        intersection.bitangent = bitangent;
    }

    fn uniform_sample_on_surface(&self, _sample: Vector2<f32>) -> (SurfaceInteraction, f32) {
        todo!()
    }
}
