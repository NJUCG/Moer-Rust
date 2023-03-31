use crate::function_layer::{Acceleration, Ray};
use super::acceleration::{AccelerationBase, AccelerationType};

pub struct LinearAccel {
    pub acc: AccelerationBase,
}

impl Default for LinearAccel {
    fn default() -> Self {
        eprintln!("Warning: Linear method is slow!");
        Self { acc: Default::default() }
    }
}

impl Acceleration for LinearAccel {
    fn acceleration(&self) -> &AccelerationBase {
        &self.acc
    }

    fn acceleration_mut(&mut self) -> &mut AccelerationBase {
        &mut self.acc
    }

    fn ray_intersect(&self, ray: &mut Ray) -> Option<(u64, u64, f32, f32)> {
        let mut r = None;
        for shape in &self.acc.shapes {
            let res = shape.borrow().ray_intersect_shape(ray);
            if let Some((prim_id, u, v)) = res {
                r = Some((shape.borrow().geometry_id(), prim_id, u, v));
            }
        }
        r
    }

    fn build(&mut self) {
        for shape in &self.acc.shapes {
            shape.borrow_mut().init_internal_acceleration();
        }
    }

    fn atp(&self) -> AccelerationType {
        AccelerationType::Linear
    }
}