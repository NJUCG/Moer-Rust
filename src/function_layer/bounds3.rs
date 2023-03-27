use std::mem::swap;
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
        let p_min = p1.inf(&p2);
        let p_max = p1.sup(&p2);
        Self { p_min, p_max }
    }

    pub fn empty() -> Self {
        Self { p_min: V3f::from([f32::INFINITY; 3]), p_max: V3f::from([f32::NEG_INFINITY; 3]) }
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
    pub fn expand(&mut self, p: &V3f) {
        self.p_min = self.p_min.inf(p);
        self.p_max = self.p_max.sup(p);
    }
    #[allow(dead_code)]
    pub fn intersect(&self, b: &Bounds3) -> Bounds3 {
        Bounds3 {
            p_min: self.p_min.sup(&b.p_min),
            p_max: self.p_max.inf(&b.p_max),
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
        let mut t_near = ray.t_min;
        let mut t_far = ray.t_max;
        let inv_dir = &ray.inv_dir;

        let mut t_min = (self.p_min - ray.origin.coords).component_mul(inv_dir);
        let mut t_max = (self.p_max - ray.origin.coords).component_mul(inv_dir);
        if inv_dir.x < 0.0 { swap(&mut t_min.x, &mut t_max.x); }
        if inv_dir.y < 0.0 { swap(&mut t_min.y, &mut t_max.y); }
        if inv_dir.z < 0.0 { swap(&mut t_min.z, &mut t_max.z); }
        t_near = t_near.max(t_min.max());
        t_far = t_far.min(t_max.min());
        (t_near, t_far)
    }
    pub fn union_bounds(b1: &Bounds3, b2: &Bounds3) -> Bounds3 {
        Bounds3 {
            p_min: b1.p_min.inf(&b2.p_min),
            p_max: b1.p_max.sup(&b2.p_max),
        }
    }
    #[allow(dead_code)]
    pub fn union_point(b: &Bounds3, p: &V3f) -> Bounds3 {
        Bounds3 {
            p_min: b.p_min.inf(p),
            p_max: b.p_max.sup(p),
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


impl Default for Bounds3 {
    fn default() -> Self {
        Bounds3::empty()
    }
}