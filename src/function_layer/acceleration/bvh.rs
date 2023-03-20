#![allow(dead_code)]

use std::rc::Rc;
use crate::function_layer::{Acceleration, Bounds3, Intersection, Ray, Shape, V3f};
use super::acceleration::{AccelerationBase, AccelerationType};

pub struct BVHBuildNode {
    bounds: Bounds3,
    shape: Option<Rc<dyn Shape>>,
    left: Option<Rc<BVHBuildNode>>,
    right: Option<Rc<BVHBuildNode>>,
    area: f32,
}

impl Default for BVHBuildNode {
    fn default() -> Self {
        BVHBuildNode {
            bounds: Default::default(),
            shape: None,
            left: None,
            right: None,
            area: 0.0,
        }
    }
}

const MAX_PRIMS_IN_NODE: u64 = 64;

#[derive(Default)]
pub struct BVHAccel {
    pub root: Option<Rc<BVHBuildNode>>,
    pub acc: AccelerationBase,
}

impl Acceleration for BVHAccel {
    fn acceleration(&self) -> &AccelerationBase {
        &self.acc
    }

    fn acceleration_mut(&mut self) -> &mut AccelerationBase {
        &mut self.acc
    }

    fn ray_intersect(&self, ray: &Ray) -> Option<(u64, u64, f32, f32)> {
        let root = self.root.clone();
        if root.is_none() { return None; }
        let its = BVHAccel::get_intersection(root.unwrap(), ray);
        let shape = its.shape.as_ref().unwrap().shape();
        shape.geometry_id;
        todo!()
    }

    fn build(&mut self) {
        todo!()
    }

    fn atp(&self) -> AccelerationType {
        AccelerationType::BVH
    }

    fn bound3(&self) -> Bounds3 { self.acc.bounds.clone() }
}

impl BVHAccel {
    pub fn get_intersection(nodes: Rc<BVHBuildNode>, ray: &Ray) -> Intersection {
        let res = Intersection::default();
        let dir_inv = V3f::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        if !nodes.bounds.intersect_p(ray, &dir_inv,
                                     [ray.direction.x < 0.0, ray.direction.y < 0.0, ray.direction.z < 0.]) {
            return res;
        }
        if nodes.left.is_none() && nodes.right.is_none() {
            return match &nodes.shape {
                None => res,
                Some(_) => { res }
            };
        }
        let hit1 = BVHAccel::get_intersection(nodes.left.as_ref().unwrap().clone(), ray);
        let hit2 = BVHAccel::get_intersection(nodes.right.as_ref().unwrap().clone(), ray);
        if hit1.distance < hit2.distance { hit1 } else { hit2 }
    }
}