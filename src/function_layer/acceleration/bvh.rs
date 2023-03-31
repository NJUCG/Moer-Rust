use super::acceleration::{AccelerationBase, AccelerationType};
use crate::function_layer::{Acceleration, Bounds3, Ray, RR, Shape, bounds3::Axis};


pub enum BVHNode {
    Node {
        bounds: Bounds3,
        left: usize,
        right: usize,
        // father: usize,
        split_axis: Axis,
    },
    Leaf {
        bounds: Bounds3,
        shape_idx: usize,
        // father: usize,
    },
}

impl BVHNode {
    pub fn get_bounds(&self) -> &Bounds3 {
        match self {
            BVHNode::Node { bounds: b, .. } | BVHNode::Leaf { bounds: b, .. } => b
        }
    }
    #[allow(dead_code)]
    pub fn get_father_idx(&self) -> usize {
        match self {
            BVHNode::Node { .. } | BVHNode::Leaf { .. } => 0
        }
    }
}

impl Default for BVHNode {
    fn default() -> Self {
        BVHNode::Node {
            bounds: Default::default(),
            left: 0,
            right: 0,
            split_axis: Axis::X,
        }
    }
}

const USE_SAH: bool = true;

#[derive(Default)]
pub struct BVHAccel {
    nodes: Vec<BVHNode>,
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
        if self.nodes.is_empty() { return None; }
        BVHAccel::get_intersection(&self.nodes, 0, ray, &self.acc.shapes)
    }

    fn build(&mut self) {
        for shape in &self.acc.shapes {
            shape.borrow_mut().init_internal_acceleration();
        }
        recursively_build(&mut self.acc.shapes, 0, &mut self.nodes);
        // TODO: 单纯的SAH构建的树可能不平衡
        self.acc.bounds = self.nodes[0].get_bounds().clone();
    }

    fn atp(&self) -> AccelerationType {
        AccelerationType::BVH
    }
}

fn get_bounds_arr(shapes: &[RR<dyn Shape>]) -> Bounds3 {
    let bounds_v: Vec<_> = shapes.iter().
        map(|s: &RR<dyn Shape>| s.borrow().get_bounds().clone()).collect();
    Bounds3::arr_bounds(bounds_v)
}

fn recursively_build(shapes: &mut [RR<dyn Shape>], b: usize, nodes: &mut Vec<BVHNode>) -> usize {
    let bounds = get_bounds_arr(shapes);
    let idx = nodes.len();
    if shapes.len() == 1 {
        nodes.push(BVHNode::Leaf { bounds, shape_idx: b });
        return idx;
    }
    if shapes.len() == 2 {
        nodes.push(BVHNode::default());
        let l = recursively_build(&mut shapes[..1], b, nodes);
        let r = recursively_build(&mut shapes[1..], b + 1, nodes);
        nodes[idx] = BVHNode::Node { bounds, left: l, right: r, split_axis: Axis::X };
        return idx;
    }
    let mut mid = shapes.len() / 2;
    let axis = bounds.max_extent();
    let _ = shapes.sort_unstable_by(|s1: &RR<dyn Shape>, s2: &RR<dyn Shape>| {
        s1.borrow().shape().bounds3.centroid_axis(axis).partial_cmp(
            &s2.borrow().shape().bounds3.centroid_axis(axis)
        ).unwrap()
    });
    for i in 0..shapes.len() {
        shapes[i].borrow_mut().set_geometry_id((i + b) as u64);
    }
    if USE_SAH && shapes.len() > 4 {
        let len = shapes.len();
        let part = len.min(32);
        let mut scores = Vec::with_capacity(part);
        scores.push(f32::MAX);
        for idx in 1..part {
            let pp = len * idx / part;
            let ls = &shapes[..pp];
            let rs = &shapes[pp..];
            assert_eq!(ls.len() + rs.len(), shapes.len());
            let lbs = get_bounds_arr(ls);
            let rbs = get_bounds_arr(rs);
            let l_area = lbs.surface_area();
            let r_area = rbs.surface_area();
            // let area = l_area + r_area;
            scores.push(l_area * ls.len() as f32 + r_area * rs.len() as f32);
        }
        let cut = scores
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(i, _)| i).unwrap();
        mid = len * cut / part;
    }
    nodes.push(BVHNode::Node {
        bounds: Default::default(),
        left: 0,
        right: 0,
        split_axis: Axis::X,
    });
    let l = recursively_build(&mut shapes[..mid], b, nodes);
    let r = recursively_build(&mut shapes[mid..], b + mid, nodes);
    nodes[idx] = BVHNode::Node {
        bounds,
        left: l,
        right: r,
        split_axis: axis,
    };
    idx
}

impl BVHAccel {
    pub fn get_intersection(nodes: &Vec<BVHNode>, root: usize, ray: &mut Ray, shapes: &Vec<RR<dyn Shape>>) -> Option<(u64, u64, f32, f32)> {
        if !nodes[root].get_bounds().intersect_p(ray) { return None; }
        let node = &nodes[root];
        match node {
            BVHNode::Node { left: l, right: r, split_axis: axis, .. } => {
                let mut two_children = [*l, *r];
                let flip: bool = match axis {
                    Axis::X => ray.direction.x < 0.0,
                    Axis::Y => ray.direction.y < 0.0,
                    Axis::Z => ray.direction.z < 0.0
                };
                if flip { two_children.reverse(); }
                let hit1 = BVHAccel::get_intersection(nodes, two_children[0], ray, shapes);
                let hit2 = BVHAccel::get_intersection(nodes, two_children[1], ray, shapes);
                if hit2.is_some() { hit2 } else { hit1 }
            }
            BVHNode::Leaf { shape_idx: idx, .. } => {
                let shape = shapes[*idx].borrow();
                let its = shape.ray_intersect_shape(ray);
                if let Some((p_id, u, v)) = its {
                    Some((shape.geometry_id(), p_id, u, v))
                } else { None }
            }
        }
    }
}
