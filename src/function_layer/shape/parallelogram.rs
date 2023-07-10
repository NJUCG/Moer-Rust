use super::shape::ShapeBase;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{fetch_v3f, SurfaceInteraction, Ray, Shape, V3f};
use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector2, Zero};
use serde_json::Value;
use std::rc::Rc;

#[derive(Clone)]
pub struct Parallelogram {
    shape: ShapeBase,
    pub base: Point3<f32>,
    pub edge0: V3f,
    pub edge1: V3f,
    pdf: f32,
}

impl Parallelogram {
    pub fn from_json(json: &Value) -> Self {
        let base = fetch_v3f(json, "base", V3f::zero()).unwrap();
        let edge0 = fetch_v3f(json, "edge0", V3f::zero()).unwrap();
        let edge1 = fetch_v3f(json, "edge1", V3f::zero()).unwrap();
        let base = Point3::from([base.x, base.y, base.z]);

        let mut shape = ShapeBase::from_json(json);
        let trans = &shape.transform;
        let base = trans.to_world_point(&base);
        let b = base.to_vec();
        let edge0 = trans.to_world_vec(&edge0);
        let edge1 = trans.to_world_vec(&edge1);

        shape.bounds3.expand(b);
        shape.bounds3.expand(b + edge0);
        shape.bounds3.expand(b + edge1);
        shape.bounds3.expand(b + edge0 + edge1);

        let area = edge0.cross(edge1).magnitude();
        Self {
            shape,
            base,
            edge0,
            edge1,
            pdf: 1.0 / area,
        }
    }
}

impl Transformable for Parallelogram {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Parallelogram {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)> {
        let origin = ray.origin;
        let dir = ray.direction;
        let normal = self.edge0.cross(self.edge1).normalize();
        let d = -normal.dot(self.base.to_vec());
        let a = normal.dot(origin.to_vec()) + d;
        let b = normal.dot(dir);
        if b == 0.0 {
            return None;
        }
        let t = -a / b;
        if t < ray.t_min || t > ray.t_max {
            return None;
        }
        let (edge0, edge1) = (self.edge0, self.edge1);
        let hit = origin + t * dir;
        let v1 = (hit - self.base).cross(edge1);
        let v2 = edge0.cross(edge1);
        let mut u = v1.magnitude() / v2.magnitude();
        if v1.dot(v2) < 0.0 {
            u *= -1.0;
        }

        let v1 = (hit - self.base).cross(edge0);
        let v2 = -v2; //cross(edge1, edge0)
        let mut v = v1.magnitude() / v2.magnitude();
        if v1.dot(v2) < 0.0 {
            v *= -1.0;
        }

        if 0.0 <= u && u <= 1.0 && 0.0 <= v && v <= 1.0 {
            ray.t_max = t;
            Some((0, u, v))
        } else {
            None
        }
    }

    fn fill_intersection(
        &self,
        distance: f32,
        _prim_id: u64,
        u: f32,
        v: f32,
        intersection: &mut SurfaceInteraction,
    ) {
        intersection.shape = Some(Rc::new(self.clone()));
        intersection.distance = distance;
        intersection.shape = Some(Rc::new(self.clone()));
        intersection.normal = self.edge0.cross(self.edge1).normalize();
        intersection.tex_coord = Vector2::new(u, v);
        intersection.position = self.base + u * self.edge0 + v * self.edge1;
        intersection.dp_du = self.edge0;
        intersection.dp_dv = self.edge1;
        intersection.tangent = self.edge0.normalize();
        intersection.bitangent = intersection.tangent.cross(intersection.normal).normalize();
        if let Some(mi) = &self.shape.medium_interface {
            intersection.medium_interface = mi.clone();
        }
    }

    fn uniform_sample_on_surface(&self, sample: Vector2<f32>) -> (SurfaceInteraction, f32) {
        let mut its = SurfaceInteraction::default();
        self.fill_intersection(0.0, 0, sample.x, sample.y, &mut its);
        (its, self.pdf)
    }

    fn shape_type(&self) -> String {
        "Parallelogram".to_owned()
    }
}
