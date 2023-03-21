use std::rc::Rc;
use nalgebra::Vector3;
use crate::core_layer::colorspace::SpectrumRGB;
use super::integrator::Integrator;
use crate::function_layer::ray::Ray;
use crate::function_layer::RR;
use crate::function_layer::sampler::sampler::Sampler;
use crate::function_layer::scene::Scene;

pub struct NormalIntegrator;

impl Integrator for NormalIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, _sampler: RR<dyn Sampler>) -> SpectrumRGB {
        let intersection_opt = scene.ray_intersect(ray);
        match intersection_opt {
            None => SpectrumRGB::same(0.0),
            Some(val) => SpectrumRGB::from_rgb((val.normal + Vector3::from([1.0; 3])) * 0.5)
        }
    }
}
