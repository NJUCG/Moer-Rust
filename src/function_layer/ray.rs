use nalgebra::Point3;
use crate::function_layer::V3f;

#[derive(Debug, Clone)]
pub struct RayDifferential {
    pub origin_x: Point3<f32>,
    pub origin_y: Point3<f32>,
    pub direction_x: V3f,
    pub direction_y: V3f,
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: V3f,
    pub inv_dir: V3f,
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
        let t_max = f32::INFINITY;
        let inv_dir = V3f::new(
            1.0 / direction.x,
            1.0 / direction.y,
            1.0 / direction.z,
        );
        Self { origin, direction, inv_dir, t, t_min, t_max, differential: None }
    }

    #[allow(dead_code)]
    pub fn from_o2d(origin: Point3<f32>, destination: Point3<f32>) -> Self {
        let o2d = destination - origin;
        let t = 0.0;
        let t_min = 1e-4;
        let t_max = o2d.norm() - 1e-4;
        let direction = o2d.normalize();
        let inv_dir = V3f::new(
            1.0 / direction.x,
            1.0 / direction.y,
            1.0 / direction.z,
        );
        Self { origin, inv_dir, direction, t, t_min, t_max, differential: None }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        let delta = t * self.direction;
        let o = self.origin;
        o + delta
    }
    #[allow(dead_code)]
    pub fn change_dir(&mut self, dir: V3f) {
        self.direction = dir;
    }
}