#![allow(dead_code)]

use serde_json::Value;
use crate::function_layer::{Shape, Intersection, Ray, Bounds3, RR};

pub enum AccelerationType {
    Embree,
    Linear,
    Octree,
    BVH,
}

pub trait Acceleration {
    fn acceleration(&self) -> &AccelerationBase;
    fn acceleration_mut(&mut self) -> &mut AccelerationBase;
    fn get_intersect(&self, ray: &Ray) -> Option<Intersection> {
        let hit = self.ray_intersect(ray);
        if hit.is_none() { return None; }
        let (prime_id, geom_id, u, v) = hit.unwrap();
        let mut its = Intersection::default();
        self.acceleration().shapes[geom_id as usize]
            .borrow().fill_intersection(ray.t_max, prime_id, u, v, &mut its);
        Some(its)
    }
    fn ray_intersect(&self, ray: &Ray) -> Option<(u64, u64, f32, f32)>;
    fn build(&mut self);
    fn attach_shape(&mut self, shape: RR<dyn Shape>) {
        self.acceleration_mut().shapes.push(shape)
    }
    fn atp(&self) -> AccelerationType;
}

#[derive(Default)]
pub struct AccelerationBase {
    pub bounds: Bounds3,
    shapes: Vec<RR<dyn Shape>>,
}

pub fn construct_acceleration(json: &Value) -> RR<dyn Acceleration> {
    match json["acceleration"].as_str().unwrap() {
        "bvh" => {}
        _ => ()
    }
    todo!()
}