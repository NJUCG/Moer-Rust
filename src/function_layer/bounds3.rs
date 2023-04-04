use std::mem::swap;
use cgmath::ElementWise;
use crate::function_layer::V3f;
use super::ray::Ray;


#[derive(Clone, Debug)]
pub struct Bounds3 {
    pub p_min: V3f,
    pub p_max: V3f,
}

#[derive(Clone, Copy)]
pub enum Axis { X, Y, Z }

impl Bounds3 {
    pub fn new(p1: V3f, p2: V3f) -> Self {
        let p_min = ele_wise_min(p1, p2);
        let p_max = ele_wise_max(p1, p2);
        Self { p_min, p_max }
    }

    pub fn empty() -> Self {
        Self { p_min: V3f::from([f32::INFINITY; 3]), p_max: V3f::from([f32::NEG_INFINITY; 3]) }
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.p_min.x > self.p_max.x || self.p_min.y > self.p_max.y || self.p_min.z > self.p_max.z
    }
    pub fn diagonal(&self) -> V3f { &self.p_max - &self.p_min }
    pub fn max_extent(&self) -> Axis {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z { Axis::X } else if d.y > d.z { Axis::Y } else { Axis::Z }
    }
    pub fn surface_area(&self) -> f32 {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.y * d.z + d.z * d.x)
    }
    pub fn centroid(&self) -> V3f { 0.5 * &self.p_min + 0.5 * &self.p_max }
    pub fn expand(&mut self, p: V3f) {
        self.p_min = ele_wise_min(self.p_min, p);
        self.p_max = ele_wise_max(self.p_max, p);
    }
    #[allow(dead_code)]
    pub fn intersect(&self, b: &Bounds3) -> Bounds3 {
        Bounds3 {
            p_min: ele_wise_max(self.p_min, b.p_min),
            p_max: ele_wise_min(self.p_max, b.p_max),
        }
    }
    #[allow(dead_code)]
    pub fn offset(&self, p: &V3f) -> V3f {
        let mut o = p - &self.p_min;
        if self.p_max.x > self.p_min.x { o.x /= self.p_max.x - self.p_min.x; }
        if self.p_max.y > self.p_min.y { o.y /= self.p_max.y - self.p_min.y; }
        if self.p_max.z > self.p_min.z { o.z /= self.p_max.z - self.p_min.z; }
        o
    }

    pub fn overlaps(b1: &Bounds3, b2: &Bounds3) -> bool {
        let x = b1.p_max.x >= b2.p_min.x && b1.p_min.x <= b2.p_max.x;
        let y = b1.p_max.y >= b2.p_min.y && b1.p_min.y <= b2.p_max.y;
        let z = b1.p_max.z >= b2.p_min.z && b1.p_min.z <= b2.p_max.z;
        x && y && z
    }
    #[allow(dead_code)]
    pub fn inside(p: &V3f, b: &Bounds3) -> bool {
        p.x >= b.p_min.x && p.x <= b.p_max.x &&
            p.y >= b.p_min.y && p.y <= b.p_max.y &&
            p.z >= b.p_min.z && p.z <= b.p_max.z
    }
    pub fn intersect_p(&self, ray: &Ray) -> bool {
        let (t_near, t_far) = self.intersect_t(ray);
        t_near <= t_far
    }
    pub fn intersect_t(&self, ray: &Ray) -> (f32, f32) {
        let inv_dir = ray.inv_dir;
        let neg_dir = ray.neg_dir;
        let mut t_min = (self.p_min - V3f::from(ray.origin.coords.data.0[0])).mul_element_wise(inv_dir);
        let mut t_max = (self.p_max - V3f::from(ray.origin.coords.data.0[0])).mul_element_wise(inv_dir);
        for i in 0..3 {
            if neg_dir[i] { swap(&mut t_min[i], &mut t_max[i]); }
        }
        let t_near = ray.t_min.max(t_min.x.max(t_min.y.max(t_min.z)));
        let t_far = ray.t_max.min(t_max.x.min(t_max.y.min(t_max.z)));
        (t_near, t_far)
    }
    pub fn union_bounds(b1: &Bounds3, b2: &Bounds3) -> Bounds3 {
        Bounds3 {
            p_min: ele_wise_min(b1.p_min, b2.p_min),
            p_max: ele_wise_max(b1.p_max, b2.p_max),
        }
    }
    #[allow(dead_code)]
    pub fn union_point(b: &Bounds3, p: V3f) -> Bounds3 {
        Bounds3 {
            p_min: ele_wise_min(b.p_min, p),
            p_max: ele_wise_max(b.p_max, p),
        }
    }
    pub fn centroid_axis(&self, a: Axis) -> f32 {
        match a {
            Axis::X => { self.centroid().x }
            Axis::Y => { self.centroid().y }
            Axis::Z => { self.centroid().z }
        }
    }

    pub fn arr_bounds(v: Vec<Bounds3>) -> Bounds3 {
        v.into_iter().fold(Bounds3::default(), |b1, b2| { Bounds3::union_bounds(&b1, &b2) })
    }

    pub fn sub_bounds(b: &Bounds3) -> [Bounds3; 8] {
        let diff = (b.p_max - b.p_min) / 2.0;
        let mut arr = Vec::with_capacity(8);
        for i in [0.0, 1.0] {
            for j in [0.0, 1.0] {
                for k in [0.0, 1.0] {
                    let p_min = b.p_min + V3f::new(i * diff.x, j * diff.y, k * diff.z);
                    let p_max = p_min + diff;
                    arr.push(Bounds3 {
                        p_min,
                        p_max,
                    })
                }
            }
        }
        arr.try_into().unwrap()
    }
}

fn ele_wise_min(v1: V3f, v2: V3f) -> V3f {
    V3f::new(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z))
}

fn ele_wise_max(v1: V3f, v2: V3f) -> V3f {
    V3f::new(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z))
}


impl Default for Bounds3 {
    fn default() -> Self {
        Bounds3::empty()
    }
}