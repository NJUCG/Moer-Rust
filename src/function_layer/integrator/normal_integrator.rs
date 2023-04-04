use super::integrator::Integrator;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{Ray, Sampler, Scene, V3f, RR};

pub struct NormalIntegrator;

impl Integrator for NormalIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, _sampler: RR<dyn Sampler>) -> SpectrumRGB {
        let intersection_opt = scene.ray_intersect(ray);
        match intersection_opt {
            None => SpectrumRGB::same(0.0),
            Some(val) => SpectrumRGB::from_rgb((val.normal + V3f::from([1.0; 3])) * 0.5),
        }
    }
}
