#![allow(dead_code)]

use std::ops::Add;
use nalgebra::{Point3, Vector3};

type V3f = Vector3<f32>;

#[derive(Debug, Clone)]
pub struct RayDifferential {
    pub origin_x: Point3<f32>,
    pub origin_y: Point3<f32>,
    pub direction_x: Vector3<f32>,
    pub direction_y: Vector3<f32>,
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: V3f,
    // pub direction_inv: Vector3f,
    pub t: f32,
    pub t_min: f32,
    pub t_max: f32,

    // Ray differential
    pub differential: Option<RayDifferential>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: V3f) -> Self {
        let t = 0.0;
        let t_min = 1e-4;
        let t_max = f32::MAX;
        Self { origin, direction, t, t_min, t_max, differential: None }
    }

    pub fn from_o2d(origin: Point3<f32>, destination: Point3<f32>) -> Self {
        let o2d = destination - origin;
        let t = 0.0;
        let t_min = 1e-4;
        let t_max = o2d.norm() - 1e-4;
        let direction = o2d.normalize();
        Self { origin, direction, t, t_min, t_max, differential: None }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        let delta = t * self.direction;
        let o = self.origin;
        Point3::from([o[0] + delta[0], o[1] + delta[1], o[2] + delta[2]])
    }

    pub fn change_dir(&mut self, dir: V3f) {
        // self.direction_inv.x = 1.0 / dir.x;
        // self.direction_inv.y = 1.0 / dir.y;
        // self.direction_inv.z = 1.0 / dir.z;
        self.direction = dir;
    }
}