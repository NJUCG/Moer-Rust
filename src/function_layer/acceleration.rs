use std::cell::RefCell;
use std::rc::Rc;
use crate::function_layer::{Shape, Intersection, Ray, Bounds3};

pub trait Acceleration {
    fn ray_intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn build(&mut self);
    fn attach_shape(&mut self, shape: Rc<RefCell<dyn Shape>>);
}

pub struct BVHBuildNode {
    bounds: Bounds3,
    left: Option<Rc<BVHBuildNode>>,
    right: Option<Rc<BVHBuildNode>>,
    area: f32,
}

impl Default for BVHBuildNode {
    fn default() -> Self {
        BVHBuildNode {
            bounds: Default::default(),
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
        todo!()
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