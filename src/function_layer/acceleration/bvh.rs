#![allow(dead_code)]

use std::rc::Rc;
use crate::function_layer::{Acceleration, Bounds3, Intersection, Ray, RR, Shape, V3f};
use super::acceleration::{AccelerationBase, AccelerationType};

pub struct BVHBuildNode {
    bounds: Bounds3,
    left: Option<Rc<BVHBuildNode>>,
    right: Option<Rc<BVHBuildNode>>,
    first_shape_offset: usize,
    n_shapes: usize,
}

impl Default for BVHBuildNode {
    fn default() -> Self {
        BVHBuildNode {
            bounds: Default::default(),
            left: None,
            right: None,
            first_shape_offset: 0,
            n_shapes: 0,
        }
    }
}

const MAX_PRIMS_IN_NODE: usize = 8;

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

    fn ray_intersect(&self, ray: &mut Ray) -> Option<(u64, u64, f32, f32)> {
        let root = self.root.clone();
        if root.is_none() { return None; }
        let its = BVHAccel::get_intersection(root.unwrap(), ray, &self.acc.shapes);
        if let Some((_, g_id, p_id, u, v)) = its {
            Some((g_id, p_id, u, v))
        } else { None }
    }

    fn build(&mut self) {
        for shape in &self.acc.shapes {
            shape.borrow_mut().init_internal_acceleration();
        }
        let n_shapes = self.acc.shapes.len();
        let root = recursively_build(&mut self.acc.shapes, 0, n_shapes);
        self.acc.bounds = root.bounds.clone();
        self.root = Some(root);
    }

    fn atp(&self) -> AccelerationType {
        AccelerationType::BVH
    }
}

fn recursively_build(shapes: &mut Vec<RR<dyn Shape>>, b: usize, e: usize) -> Rc<BVHBuildNode> {
    let mut res = BVHBuildNode::default();
    let bounds: Vec<_> = shapes.iter().map(|s: &RR<dyn Shape>| s.borrow().shape().bounds3.clone()).collect();
    res.bounds = Bounds3::arr_bounds(bounds);
    if e - b <= MAX_PRIMS_IN_NODE {
        res.first_shape_offset = b;
        res.n_shapes = e - b;
        return Rc::new(res);
    }
    let mid = (e - b) / 2;
    let axis = res.bounds.max_extent();
    let _ = shapes[b..e].select_nth_unstable_by(mid, |s1: &RR<dyn Shape>, s2: &RR<dyn Shape>| {
        s1.borrow().shape().bounds3.centroid_axis(axis).partial_cmp(
            &s2.borrow().shape().bounds3.centroid_axis(axis)
        ).unwrap()
    });
    let l = recursively_build(shapes, b, b + mid + 1);
    let r = recursively_build(shapes, b + mid + 1, e);
    res.left = Some(l);
    res.right = Some(r);
    Rc::new(res)
}

impl BVHAccel {
    pub fn get_intersection(node: Rc<BVHBuildNode>, ray: &mut Ray, shapes: &Vec<RR<dyn Shape>>) -> Option<(f32, u64, u64, f32, f32)> {
        if !node.bounds.intersect_p(ray) {
            return None;
        }
        if node.left.is_none() && node.right.is_none() {
            let (mut dist, mut p_id, mut u, mut v) = (f32::INFINITY, 0u64, 0.0, 0.0);
            let mut sp = shapes[node.first_shape_offset].borrow();
            for shape_idx in node.first_shape_offset..node.first_shape_offset + node.n_shapes {
                let shape = shapes[shape_idx].borrow();
                let its = shape.ray_intersect_shape(ray);
                if let Some(r) = its {
                    dist = ray.t_max;
                    (p_id, u, v) = r;
                    sp = shape;
                }
            }
            if dist.is_infinite() { return None; }
            return Some((dist, sp.geometry_id(), p_id, u, v));
        }
        let hit1 = BVHAccel::get_intersection(node.left.as_ref().unwrap().clone(), ray, shapes);
        let hit2 = BVHAccel::get_intersection(node.right.as_ref().unwrap().clone(), ray, shapes);
        if hit2.is_some() { hit2 } else { hit1 }
    }
}