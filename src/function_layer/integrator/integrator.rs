use std::rc::Rc;
use cgmath::InnerSpace;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::light::light::{LightSampleResult, LightType};
use crate::function_layer::{Intersection, Scene, Sampler, RR, Ray};
use super::{normal_integrator::NormalIntegrator, whitted_integrator::WhittedIntegrator,
            direct_integrator::{DirectIntegratorSampleBSDF, DirectIntegratorSampleLight}};


pub trait Integrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB;
}

pub fn convert_pdf(result: &LightSampleResult, _intersection: &Intersection) -> f32 {
    let mut pdf = result.pdf;
    let distance = result.distance;
    match result.light_type {
        LightType::SpotLight => { pdf *= distance * distance }
        LightType::AreaLight => {
            pdf *= distance * distance;
            pdf /= result.normal.dot(result.direction).abs();
        }
        // 环境光的pdf转换在采样时已经完成
        LightType::EnvironmentLight => ()
    };
    pdf
}

pub fn construct_integrator(json: &Value) -> Rc<dyn Integrator> {
    match json["type"].as_str().unwrap() {
        "directSampleLight" => Rc::new(DirectIntegratorSampleLight {}),
        "directSampleBSDF" => Rc::new(DirectIntegratorSampleBSDF {}),
        "normal" => Rc::new(NormalIntegrator {}),
        "whitted" => Rc::new(WhittedIntegrator {}),
        tp => panic!("Invalid integrator type: {}!", tp),
    }
}