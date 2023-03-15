use std::rc::Rc;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::integrator::integrator::Integrator;
use crate::function_layer::ray::Ray;
use crate::function_layer::sampler::sampler::Sampler;
use crate::function_layer::scene::Scene;

pub struct DirectIntegratorSampleLight;

impl Integrator for DirectIntegratorSampleLight {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: Rc<dyn Sampler>) -> SpectrumRGB {
        todo!()
    }
}

pub struct DirectIntegratorSampleBSDF;

impl Integrator for DirectIntegratorSampleBSDF {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: Rc<dyn Sampler>) -> SpectrumRGB {
        todo!()
    }
}


