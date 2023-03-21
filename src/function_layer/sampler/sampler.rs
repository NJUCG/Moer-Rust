use std::cell::RefCell;
use std::rc::Rc;
use nalgebra::Vector2;
use serde_json::Value;
use crate::function_layer::RR;
use super::independent_sampler::IndependentSampler;

pub trait Sampler {
    fn xsp(&self) -> usize;
    fn ysp(&self) -> usize;
    fn next_1d(&mut self) -> f32;
    fn next_2d(&mut self) -> Vector2<f32>;
}

pub fn construct_sampler(json: &Value) -> RR<dyn Sampler> {
    match json["type"].as_str().expect("No sampler type given!") {
        "independent" => Rc::new(RefCell::new(IndependentSampler::from_json(json))),
        _ => panic!("Invalid sampler type"),
    }
}