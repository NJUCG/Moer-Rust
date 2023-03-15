use std::rc::Rc;
use crate::core_layer::colorspace::SpectrumRGB;
use super::integrator::Integrator;
use crate::function_layer::ray::Ray;
use crate::function_layer::sampler::sampler::Sampler;
use crate::function_layer::scene::Scene;

pub struct NormalIntegrator;

impl Integrator for NormalIntegrator {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: Rc<dyn Sampler>) -> SpectrumRGB {
        todo!()
    }
}

