use std::rc::Rc;
use nalgebra::Vector2;
use serde_json::Value;

pub trait Sampler {
    fn xsp(&self) -> i64;
    fn ysp(&self) -> i64;
    fn next_2d(&self) -> Vector2<f32>;
}

pub fn construct_sampler(json: &Value) -> Rc<dyn Sampler> {
    todo!()
}