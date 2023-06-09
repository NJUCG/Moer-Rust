use crate::function_layer::{Medium, V3f};
use cgmath::{InnerSpace, Point3};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RayDifferential {
    pub origin_x: Point3<f32>,
    pub origin_y: Point3<f32>,
    pub direction_x: V3f,
    pub direction_y: V3f,
}

#[derive(Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: V3f,
    pub inv_dir: V3f,
    pub neg_dir: [bool; 3],
    // pub direction_inv: Vector3f,
    pub t: f32,
    pub t_min: f32,
    pub t_max: f32,
    pub medium: Option<Rc<dyn Medium>>,
    // Ray differential
    pub differential: Option<RayDifferential>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: V3f) -> Self {
        let t = 0.0;
        let t_min = 1e-4;
        let t_max = f32::INFINITY;
        let inv_dir = V3f::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        let neg_dir = [inv_dir.x < 0.0, inv_dir.y < 0.0, inv_dir.z < 0.0];
        Self {
            origin,
            direction,
            inv_dir,
            neg_dir,
            t,
            t_min,
            t_max,
            medium: None,
            differential: None,
        }
    }

    #[allow(dead_code)]
    pub fn from_o2d(origin: Point3<f32>, destination: Point3<f32>) -> Self {
        let o2d = destination - origin;
        let t = 0.0;
        let t_min = 1e-4;
        let t_max = o2d.magnitude() - 1e-4;
        let direction = o2d.normalize();

        let inv_dir = V3f::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        let neg_dir = [inv_dir.x < 0.0, inv_dir.y < 0.0, inv_dir.z < 0.0];
        Self {
            origin,
            inv_dir,
            neg_dir,
            direction,
            t,
            t_min,
            t_max,
            medium: None,
            differential: None,
        }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        let delta = t * self.direction;
        let o = self.origin;
        Point3::from([o.x + delta.x, o.y + delta.y, o.z + delta.z])
    }

    pub fn change_dir(&mut self, dir: V3f) {
        let inv_dir = V3f::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);
        self.neg_dir = [inv_dir.x < 0.0, inv_dir.y < 0.0, inv_dir.z < 0.0];
        self.inv_dir = inv_dir;
        self.direction = dir;
    }

    pub fn reset(&mut self) {
        self.t_min = 1e-4;
        self.t_max = f32::INFINITY;
    }
}
