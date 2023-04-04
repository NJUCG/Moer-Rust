use std::f64::consts::PI;
use std::rc::Rc;
use cgmath::InnerSpace;
use nalgebra::{Point3, Vector2};
use serde_json::Value;
use crate::core_layer::function::solve_quadratic;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Bounds3, Intersection, Ray, Shape, V3f};
use super::shape::ShapeBase;

#[derive(Clone)]
pub struct Cylinder {
    pub shape: ShapeBase,
    height: f32,
    radius: f32,
    phi_max: f32,
}

impl Cylinder {
    pub fn from_json(json: &Value) -> Self {
        let radius = json["radius"].as_f64().unwrap_or(1.0) as f32;
        let height = json["height"].as_f64().unwrap_or(1.0) as f32;
        let phi_max = json["phi_max"].as_f64().unwrap_or(2.0 * PI) as f32;

        let mut shape = ShapeBase::from_json(json);
        let bounds3 = Bounds3::new(V3f::new(-radius, -radius, 0.0), V3f::new(radius, radius, height));
        shape.bounds3 = shape.transform.to_world_bounds3(bounds3);

        Self {
            shape,
            height,
            radius,
            phi_max,
        }
    }
}

impl Transformable for Cylinder {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Cylinder {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)> {
        let trans = self.transform();
        let local_ray = trans.local_ray(ray);
        let l_dir = &local_ray.direction;
        let l_origin = &local_ray.origin;
        let a = l_dir.x * l_dir.x + l_dir.y * l_dir.y;
        let b = 2.0 * (l_origin.x * l_dir.x + l_origin.y * l_dir.y);
        let c = l_origin.x * l_origin.x + l_origin.y * l_origin.y - self.radius * self.radius;
        let roots = solve_quadratic(a, b, c);

        if roots.is_none() { return None; }

        let (t0, t1) = roots.unwrap(); // t0 <= t1

        // check t0 first, if success, then skip t1
        for tt in [t0, t1] {
            if tt <= local_ray.t_min || tt >= local_ray.t_max { continue; }
            let p = local_ray.at(tt);
            if p.z < 0.0 || p.z > self.height { continue; }

            let mut its_phi = (p.y / p.x).atan();
            if its_phi < 0.0 { its_phi += PI as f32; }
            if p.y < 0.0 { its_phi += PI as f32; }
            // if its_phi > 5.0 { print!("{its_phi}:{}\t", self.phi_max); }
            if its_phi <= self.phi_max {
                ray.t_max = tt;
                let u = its_phi / self.phi_max;
                let v = p.z / self.height;
                return Some((self.geometry_id(), u, v));
            }
        }
        None
    }

    fn fill_intersection(&self, distance: f32, _prim_id: u64, u: f32, v: f32, intersection: &mut Intersection) {
        let trans = self.transform();
        let phi = u * self.phi_max;
        let normal = V3f::new(phi.cos(), phi.sin(), 0.0);
        intersection.normal = trans.to_world_vec(&normal);

        let position = Point3::new(self.radius * phi.cos(), self.radius * phi.sin(), v * self.height);
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

    fn uniform_sample_on_surface(&self, _sample: Vector2<f32>) -> (Intersection, f32) {
        todo!()
    }
}