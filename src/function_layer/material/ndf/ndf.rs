use cgmath::Vector2;
use crate::function_layer::V3f;

pub trait NDF {
    fn get_d(&self, wh_local: V3f, alpha: Vector2<f32>) -> f32;
    fn get_g(&self, wo_local: V3f, wi_local: V3f, alpha: Vector2<f32>) -> f32;
    fn pdf(&self, wo_local: V3f, wh_local: V3f, alpha: Vector2<f32>) -> f32;
    fn sample_wh(&self, wo_local: V3f, alpha: Vector2<f32>, sample: Vector2<f32>) -> V3f;
}