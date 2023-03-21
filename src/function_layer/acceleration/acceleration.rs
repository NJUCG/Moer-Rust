#![allow(dead_code)]

use std::cell::RefCell;
use crate::function_layer::{Shape, Intersection, Ray, Bounds3, RR};
use super::linear::LinearAccel;
use super::bvh::BVHAccel;

#[derive(Copy, Clone)]
pub enum AccelerationType {
    Embree,
    Linear,
    Octree,
    BVH,
}

static mut ACC_TYPE: AccelerationType = AccelerationType::BVH;

pub trait Acceleration {
    fn acceleration(&self) -> &AccelerationBase;
    fn acceleration_mut(&mut self) -> &mut AccelerationBase;
    fn get_intersect(&self, ray: &mut Ray) -> Option<Intersection> {
        let hit = self.ray_intersect(ray);
        if hit.is_none() { return None; }
        let (geom_id, prime_id, u, v) = hit.unwrap();
        let mut its = Intersection::default();
        self.acceleration().shapes[geom_id as usize]
            .borrow().fill_intersection(ray.t_max, prime_id, u, v, &mut its);
        Some(its)
    }
    fn ray_intersect(&self, ray: &mut Ray) -> Option<(u64, u64, f32, f32)>;
    fn build(&mut self);
    fn attach_shape(&mut self, shape: RR<dyn Shape>) {
        self.acceleration_mut().shapes.push(shape)
    }
    fn atp(&self) -> AccelerationType;
    fn bound3(&self) -> &Bounds3 { &self.acceleration().bounds }
}

#[derive(Default)]
pub struct AccelerationBase {
    pub bounds: Bounds3,
    pub shapes: Vec<RR<dyn Shape>>,
}

pub fn set_acc_type(tp: &str) {
    unsafe {
        ACC_TYPE = match tp {
            "embree" => AccelerationType::Embree,
            "linear" => AccelerationType::Linear,
            "octree" => AccelerationType::Octree,
            "bvh" => AccelerationType::BVH,
            _ => panic!("Unknown acc type!"),
        }
    }
}

pub fn create_acceleration() -> RR<dyn Acceleration> {
    match unsafe { ACC_TYPE } {
        AccelerationType::BVH => { RR::new(RefCell::new(BVHAccel::default())) }
        AccelerationType::Linear => { RR::new(RefCell::new(LinearAccel::default())) }
        _ => panic!("Not implemented yet!")
    }
}