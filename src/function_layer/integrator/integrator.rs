use std::rc::Rc;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use super::direct_integrator::{DirectIntegratorSampleBSDF, DirectIntegratorSampleLight};
use super::normal_integrator::NormalIntegrator;
use crate::function_layer::ray::Ray;
use crate::function_layer::sampler::sampler::Sampler;
use crate::function_layer::scene::Scene;

pub trait Integrator {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: Rc<dyn Sampler>) -> SpectrumRGB;
}

pub fn construct_integrator(json: &Value) -> Rc<dyn Integrator> {
    match json["type"].as_str().unwrap() {
        "directSampleLight" => Rc::new(DirectIntegratorSampleLight {}),
        "directSampleBSDF" => Rc::new(DirectIntegratorSampleBSDF {}),
        "normal" => Rc::new(NormalIntegrator {}),
        _ => panic!("Invalid integrator type!"),
    }
}