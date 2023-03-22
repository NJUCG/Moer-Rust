use std::rc::Rc;
use crate::function_layer::{Acceleration, Bounds3, Ray, RR, Shape};
use crate::function_layer::bounds3::Axis;
use super::acceleration::{AccelerationBase, AccelerationType};

pub struct BVHBuildNode {
    bounds: Bounds3,
    left: Option<Rc<BVHBuildNode>>,
    right: Option<Rc<BVHBuildNode>>,
    first_shape_offset: usize,
    n_shapes: usize,
    split_axis: Axis,
}

impl Default for BVHBuildNode {
    fn default() -> Self {
        BVHBuildNode {
            bounds: Default::default(),
            left: None,
            right: None,
            first_shape_offset: 0,
            n_shapes: 0,
            split_axis: Axis::X,
        }
    }
}

const MAX_PRIMS_IN_NODE: usize = 1;

#[derive(Default)]
pub struct BVHAccel {
    root: Option<Rc<BVHBuildNode>>,
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
        BVHAccel::get_intersection(root.unwrap(), ray, &self.acc.shapes)
    }

    fn build(&mut self) {
        for shape in &self.acc.shapes {
            shape.borrow_mut().init_internal_acceleration();
        }
        let root = recursively_build(&mut self.acc.shapes[..], 0);
        self.acc.bounds = root.bounds.clone();
        self.root = Some(root);
    }

    fn atp(&self) -> AccelerationType {
        AccelerationType::BVH
    }
}

fn recursively_build(shapes: &mut [RR<dyn Shape>], b: usize) -> Rc<BVHBuildNode> {
    let mut res = BVHBuildNode::default();
    let bounds: Vec<_> = shapes.iter().map(|s: &RR<dyn Shape>| s.borrow().get_bounds().clone()).collect();
    res.bounds = Bounds3::arr_bounds(bounds);
    if shapes.len() <= MAX_PRIMS_IN_NODE {
        res.first_shape_offset = b;
        res.n_shapes = shapes.len();
        return Rc::new(res);
    }
    let mid = shapes.len() / 2;
    let axis = res.bounds.max_extent();
    let _ = shapes.select_nth_unstable_by(mid, |s1: &RR<dyn Shape>, s2: &RR<dyn Shape>| {
        s1.borrow().shape().bounds3.centroid_axis(axis).partial_cmp(
            &s2.borrow().shape().bounds3.centroid_axis(axis)
        ).unwrap()
    });
    for i in 0..shapes.len() {
        shapes[i].borrow_mut().set_geometry_id((i + b) as u64);
    }
    res.split_axis = axis;
    let l = recursively_build(&mut shapes[..mid], b);
    let r = recursively_build(&mut shapes[mid..], b + mid);
    res.left = Some(l);
    res.right = Some(r);
    Rc::new(res)
}

impl BVHAccel {
    pub fn get_intersection(node: Rc<BVHBuildNode>, ray: &mut Ray, shapes: &Vec<RR<dyn Shape>>) -> Option<(u64, u64, f32, f32)> {
        if !node.bounds.intersect_p(ray) { return None; }
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
            return Some((sp.geometry_id(), p_id, u, v));
        }
        let mut two_child = [node.left.as_ref().unwrap().clone(), node.right.as_ref().unwrap().clone()];
        let flip: bool = match node.split_axis {
            Axis::X => ray.direction.x < 0.0,
            Axis::Y => ray.direction.y < 0.0,
            Axis::Z => ray.direction.z < 0.0
        };
        if flip { two_child.reverse(); }
        let hit1 = BVHAccel::get_intersection(two_child[0].clone(), ray, shapes);
        let hit2 = BVHAccel::get_intersection(two_child[1].clone(), ray, shapes);
        if hit2.is_some() { hit2 } else { hit1 }
    }
}
