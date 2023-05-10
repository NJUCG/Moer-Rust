#![allow(dead_code)]

use cgmath::Zero;
use serde_json::Value;

use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{compute_ray_differentials, InfiniteLight, Integrator, Light, Ray, RR, Sampler, Scene};
use crate::function_layer::material::bxdf::BSDFType;

use super::integrator::convert_pdf;

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
            if depth >= self.max_depth { break; }

            for light in &scene.infinite_lights {
                let res = light.sample(&inter, sampler.borrow_mut().next_2d());
                let mut shadow_ray =
                    Ray::new(inter.position + res.direction * 1e-4, res.direction);
                shadow_ray.t_max = res.distance;
                let occlude = scene.ray_intersect(&mut shadow_ray);
                if occlude.is_none() {
                    let material = inter.shape.as_ref().unwrap().material();
                    let bsdf = material.unwrap().compute_bsdf(&inter);
                    let f = bsdf.f(-ray.direction, shadow_ray.direction);
                    let pdf = convert_pdf(&res, &inter);
                    spectrum += res.energy * f / pdf;
                }
            }
            let mut pdf_light = 0.0;
            let light_opt = scene.sample_light(sampler.borrow_mut().next_1d(), &mut pdf_light);
            if light_opt.is_some() && pdf_light != 0.0 {
                let light = light_opt.unwrap();
                let mut light_sample_result = light
                    .borrow()
                    .sample(&inter, sampler.borrow_mut().next_2d());
                let mut shadow_ray = Ray::new(inter.position, light_sample_result.direction);
                shadow_ray.t_max = light_sample_result.distance;
                let occlude = scene.ray_intersect(&mut shadow_ray);
                if occlude.is_none() {
                    let material = inter.shape.as_ref().unwrap().material();
                    let bsdf = material.unwrap().compute_bsdf(&inter);
                    let f = bsdf.f(-ray.direction, shadow_ray.direction);
                    light_sample_result.pdf *= pdf_light;
                    let pdf = convert_pdf(&light_sample_result, &inter);
                    spectrum += light_sample_result.energy * f / pdf;
                }
            }
            if depth > 2 && sampler.borrow_mut().next_1d() > 0.95 {
                break;
            }
            throughput /= 0.95;
            let bsdf = inter.shape.as_ref().unwrap().material().as_ref().unwrap().compute_bsdf(&inter);
            let bsdf_sample_result = bsdf.sample(-ray.direction, sampler.borrow_mut().next_2d());
            if bsdf_sample_result.weight.rgb().is_zero() { break; }
            throughput *= &bsdf_sample_result.weight;

            ray.origin = inter.position;
            ray.change_dir(bsdf_sample_result.wi);
            ray.reset();
            specular_bounce = bsdf_sample_result.tp == BSDFType::Specular;
        }
        spectrum
    }
}

