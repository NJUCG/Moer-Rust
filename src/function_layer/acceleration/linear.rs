use std::process::exit;
use crate::function_layer::{Acceleration, Ray};
use crate::function_layer::acceleration::acceleration::AccelerationType;
use super::acceleration::AccelerationBase;

#[derive(Default)]
pub struct LinearAccel {
    pub acc: AccelerationBase,
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