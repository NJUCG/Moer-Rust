use crate::function_layer::{Acceleration, Bounds3, Ray, RR, Shape};
use super::acceleration::{AccelerationBase, AccelerationType};

const MAX_DEPTH: usize = 4;
const MAX_LEAF_SIZE: usize = 32;

#[derive(Default)]
struct OctreeNode {
    bounds: Bounds3,
    index_buf: Option<Vec<usize>>,
    sub_nodes: Option<[Option<Box<OctreeNode>>; 8]>,
}

#[derive(Default)]
pub struct Octree {
    root: Option<Box<OctreeNode>>,
    pub acc: AccelerationBase,
}

impl Acceleration for Octree {
    fn acceleration(&self) -> &AccelerationBase {
        &self.acc
    }

    fn acceleration_mut(&mut self) -> &mut AccelerationBase {
        &mut self.acc
    }

    fn ray_intersect(&self, ray: &mut Ray) -> Option<(u64, u64, f32, f32)> {
        let root = self.root.as_ref();
        if root.is_none() { return None; }
        return self.get_intersection(root.unwrap(), ray);
    }

    fn build(&mut self) {
        let bounds: Vec<Bounds3> = self.acc.shapes.iter().map(|s: &RR<dyn Shape>| s.borrow().get_bounds().clone()).collect();
        let bounds = Bounds3::arr_bounds(bounds);
        for shape in &self.acc.shapes {
            shape.borrow_mut().init_internal_acceleration();
        }
        let index_buf: Vec<_> = (0..self.acc.shapes.len()).collect();
        self.root = self.recursively_build(bounds, index_buf, 0);
        self.acc.bounds = self.root.as_ref().unwrap().bounds.clone();
    }

    fn atp(&self) -> AccelerationType {
        AccelerationType::Octree
    }
}

impl Octree {
    fn get_intersection(&self, node: &Box<OctreeNode>, ray: &mut Ray) -> Option<(u64, u64, f32, f32)> {
        if !node.bounds.intersect_p(ray) { return None; }
        if node.sub_nodes.is_none() {
            let (mut dist, mut p_id, mut u, mut v) = (f32::INFINITY, 0u64, 0.0, 0.0);
            let mut sp = self.acc.shapes[0].borrow();
            for idx in node.index_buf.as_ref().unwrap() {
                let shape = self.acc.shapes[*idx].borrow();
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
        let mut sub_res = vec![];
        for i in 0..8 {
            let sub_node = node.sub_nodes.as_ref().unwrap()[i].as_ref();
            if sub_node.is_none() { continue; }
            sub_res.push(self.get_intersection(sub_node.unwrap(), ray));
        }
        for res in sub_res.into_iter().rev() {
            if res.is_some() { return res; }
        }
        None
    }
    fn recursively_build(&self, b: Bounds3, index_buffer: Vec<usize>, depth: usize) -> Option<Box<OctreeNode>> {
        if index_buffer.is_empty() { return None; }
        let bounds: Vec<Bounds3> = index_buffer.iter().map(|idx: &usize| {
            self.acc.shapes[*idx].borrow().get_bounds().clone()
        }).collect();
        let bounds = Bounds3::arr_bounds(bounds);
        if index_buffer.len() <= MAX_LEAF_SIZE || depth > MAX_DEPTH {
            return Some(Box::new(OctreeNode {
                bounds,
                index_buf: Some(index_buffer),
                sub_nodes: None,
            }));
        }
        let sub_bounds = Bounds3::sub_bounds(&b);
        let mut sub_buffers: [Vec<usize>; 8] = Default::default();
        let mut node = OctreeNode {
            bounds,
            index_buf: None,
            sub_nodes: Some(Default::default()),
        };
        for i in 0..8 {
            for index in &index_buffer {
                if Bounds3::overlaps(self.acc.shapes[*index].borrow().get_bounds(),
                                     &sub_bounds[i]) {
                    sub_buffers[i].push(*index);
                }
            }
        }
        for (i, (sbs, sbf)) in
        sub_bounds.into_iter().zip(sub_buffers.into_iter()).enumerate() {
            node.sub_nodes.as_mut().unwrap()[i] = self.recursively_build(sbs, sbf, depth + 1);
        }

        Some(Box::new(node))
    }
}
