#![allow(dead_code)]

use cgmath::Zero;
use serde_json::Value;

use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::material::bxdf::BSDFType;
use crate::function_layer::{
    compute_ray_differentials, InfiniteLight, Integrator, Ray, Sampler, Scene, RR,
};

use super::integrator::sample_interaction_illumination;

pub struct PathIntegrator {
    max_depth: u32,
}

impl PathIntegrator {
    pub fn from_json(json: &Value) -> Self {
        let max_depth = json["maxDepth"].as_u64().unwrap() as u32;
        Self { max_depth }
    }
}

impl Integrator for PathIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB {
        let mut spectrum = SpectrumRGB::same(0.0);
        let mut throughput = SpectrumRGB::same(1.0);

        let mut depth = 0u32;
        let mut specular_bounce = false;
        loop {
            let inter_opt = scene.ray_intersect(ray);
            if inter_opt.is_none() {
                for light in &scene.infinite_lights {
                    spectrum += throughput * light.evaluate_emission_ray(ray);
                }
                return spectrum;
            }
            let mut inter = inter_opt.unwrap();
            compute_ray_differentials(&mut inter, ray);
            if depth == 0 || specular_bounce {
                if let Some(light) = inter.shape.as_ref().unwrap().get_light() {
                    spectrum += light.borrow().evaluate_emission(&inter, -ray.direction);
                }
            }
            depth += 1;
            if depth >= self.max_depth {
                break;
            }
            spectrum = sample_interaction_illumination(
                scene,
                -ray.direction,
                &inter,
                spectrum,
                sampler.clone(),
                throughput,
            );
            if depth > 2 && sampler.borrow_mut().next_1d() > 0.95 {
                break;
            }
            throughput /= 0.95;
            let bsdf = inter
                .shape
                .as_ref()
                .unwrap()
                .material()
                .as_ref()
                .unwrap()
                .compute_bsdf(&inter);
            let bsdf_sample_result = bsdf.sample(-ray.direction, sampler.borrow_mut().next_2d());
            if bsdf_sample_result.weight.rgb().is_zero() {
                break;
            }
            throughput *= &bsdf_sample_result.weight;

            ray.origin = inter.position;
            ray.change_dir(bsdf_sample_result.wi);
            ray.reset();
            specular_bounce = bsdf_sample_result.tp == BSDFType::Specular;
        }
        spectrum
    }
}
