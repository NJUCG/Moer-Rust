#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::function_layer::{Shape, Intersection, Ray, Bounds3, V3f};

pub trait Acceleration {
    fn ray_intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn build(&mut self);
    fn attach_shape(&mut self, shape: Rc<RefCell<dyn Shape>>);
}

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

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum SplitMethod {
    Naive,
    SAH, // SAH is not implemented
}

impl Default for SplitMethod {
    fn default() -> Self {
        SplitMethod::Naive
    }
}

#[derive(Default)]
pub struct BVHAccel {
    pub root: Option<Rc<BVHBuildNode>>,
    pub shapes: Vec<Rc<RefCell<dyn Shape>>>,
    pub max_prims_in_node: i32,
    pub split_method: SplitMethod,
}

impl Acceleration for BVHAccel {
    fn ray_intersect(&self, ray: &Ray) -> Option<Intersection> {
        match &self.root {
            None => None,
            Some(r) => Some(BVHAccel::get_intersection(r.clone(), ray)),
        }
    }

    fn build(&mut self) {
        todo!()
    }

    fn attach_shape(&mut self, shape: Rc<RefCell<dyn Shape>>) {
        let id = self.shapes.len();
        shape.borrow_mut().set_geometry_id(id as u64);
        self.shapes.push(shape);
    }
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