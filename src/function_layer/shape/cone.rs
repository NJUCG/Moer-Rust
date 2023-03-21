use std::f64::consts::PI;
use std::rc::Rc;
use nalgebra::{Point3, Vector2};
use serde_json::Value;
use crate::core_layer::function::solve_quadratic;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Bounds3, Intersection, Ray, Shape, V3f};
use super::shape::ShapeBase;

#[derive(Clone)]
pub struct Cone {
    pub shape: ShapeBase,
    phi_max: f32,
    radius: f32,
    height: f32,
    cos_theta: f32,
}

impl Cone {
    pub fn from_json(json: &Value) -> Self {
        let radius = json["radius"].as_f64().unwrap_or(1.0) as f32;
        let height = json["height"].as_f64().unwrap_or(1.0) as f32;
        let phi_max = json["phi_max"].as_f64().unwrap_or(2.0 * PI) as f32;
        let tan_theta = radius / height;
        let cos_theta = (1.0 / (1.0 + tan_theta * tan_theta)).sqrt();
        let mut shape = ShapeBase::from_json(json);
        let bounds3 = Bounds3::new(V3f::new(-radius, -radius, 0.0), V3f::new(radius, radius, height));
        shape.bounds3 = shape.transform.to_world_bounds3(bounds3);
        Self {
            shape,
            height,
            radius,
            phi_max,
            cos_theta,
        }
    }
}

impl Transformable for Cone {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Cone {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)> {
        let trans = self.transform();
        let local_ray = trans.local_ray(ray);
        let d = &local_ray.direction;
        let o = &local_ray.origin;

        let cc = Point3::new(0.0, 0.0, self.height);
        let co: V3f = o - cc;
        let pw2_cos = self.cos_theta * self.cos_theta;
        let a = -d.z * -d.z - pw2_cos; // (d . v)^2 - cos^2 theta.
        let b = 2.0 * (-d.z * -co.z - d.dot(&co) * pw2_cos);
        let c = -co.z * -co.z - co.dot(&co) * pw2_cos;
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

            if its_phi > self.phi_max { continue; }

            ray.t_max = tt;
            let u = its_phi / self.phi_max;
            let v = p.z / self.height;
            return Some((self.geometry_id(), u, v));
        }
        None
    }

    fn fill_intersection(&self, distance: f32, _prim_id: u64, u: f32, v: f32, intersection: &mut Intersection) {
        let trans = self.transform();
        let phi = u * self.phi_max;
        let z = v * self.height;
        let ck_norm = (self.height - z) / (self.cos_theta * self.cos_theta);
        let k = Point3::from([0.0, 0.0, self.height - ck_norm]);

        let position = Point3::new(self.radius * (1.0 - v) * phi.cos(), self.radius * (1.0 - v) * phi.sin(), z);
        intersection.position = trans.to_world_point(&position);

        let normal: V3f = (position - k).normalize();
        intersection.normal = trans.to_world_vec(&normal);

        intersection.shape = Some(Rc::new(self.clone()));
        intersection.distance = distance;
        intersection.tex_coord = Vector2::new(u, v);
        // 计算交点的切线和副切线
        let mut tangent = V3f::new(1.0, 0.0, 0.0);
        if tangent.dot(&intersection.normal).abs() > 0.9 {
            tangent = V3f::new(0.0, 1.0, 0.0);
        }
        let bitangent = tangent.cross(&intersection.normal).normalize();
        tangent = intersection.normal.cross(&bitangent).normalize();
        intersection.tangent = tangent;
        intersection.bitangent = bitangent;
    }

    fn uniform_sample_on_surface(&self, _sample: Vector2<f32>) -> (Intersection, f32) {
        todo!()
    }
}