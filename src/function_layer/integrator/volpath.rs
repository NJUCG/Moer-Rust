use super::integrator::sample_interaction_illumination;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::material::bxdf::BSDFType;
use crate::function_layer::{
    compute_ray_differentials, InfiniteLight, Integrator, MediumInteraction, Ray, Sampler, Scene,
    V3f, RR,
};
use cgmath::{InnerSpace, Zero};
use serde_json::Value;

pub struct VolPathIntegrator {
    max_depth: u32,
}

impl VolPathIntegrator {
    pub fn from_json(json: &Value) -> Self {
        let max_depth = json["maxDepth"].as_u64().unwrap() as u32;
        Self { max_depth }
    }
}

impl Integrator for VolPathIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB {
        let mut spectrum = SpectrumRGB::same(0.0);
        let mut throughput = SpectrumRGB::same(1.0);
        let mut specular_bounce = false;
        let mut depth = 0;

        loop {
            let inter_opt = scene.ray_intersect(ray);
            let mut mi = MediumInteraction::default();
            if let Some(medium) = &ray.medium {
                throughput *= &medium.sample(ray, sampler.clone(), &mut mi);
            }
            if throughput.rgb().is_zero() {
                break;
            }

            if mi.is_valid() {
                // has phase function
                if depth >= self.max_depth {
                    break;
                }
                // sample medium illumination
                spectrum = sample_interaction_illumination(
                    scene,
                    -ray.direction,
                    &mi,
                    spectrum,
                    sampler.clone(),
                    throughput,
                );
                // sample out ray through the medium
                let mut wi = V3f::zero();
                mi.phase.as_ref().unwrap().sample_p(
                    -ray.direction,
                    &mut wi,
                    sampler.borrow_mut().next_2d(),
                );
                ray.origin = mi.position;
                ray.change_dir(wi);
                ray.reset();
                specular_bounce = false;
            } else {
                if inter_opt.is_none() {
                    for light in &scene.infinite_lights {
                        spectrum += throughput * light.evaluate_emission_ray(ray);
                    }
                    return spectrum;
                }

                let mut inter = inter_opt.unwrap();
                compute_ray_differentials(&mut inter, ray);

                if specular_bounce || depth == 0 {
                    if let Some(light) = inter.shape.as_ref().unwrap().get_light() {
                        spectrum += light.borrow().evaluate_emission(&inter, -ray.direction);
                    }
                }

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

                // sample bsdf
                let bsdf = inter
                    .shape
                    .as_ref()
                    .unwrap()
                    .material()
                    .as_ref()
                    .unwrap()
                    .compute_bsdf(&inter);
                let bsdf_sample_result =
                    bsdf.sample(-ray.direction, sampler.borrow_mut().next_2d());
                if bsdf_sample_result.weight.rgb().is_zero() {
                    break;
                }
                throughput *= &bsdf_sample_result.weight;

                ray.origin = inter.position;
                ray.change_dir(bsdf_sample_result.wi);
                // change the medium
                ray.medium = if inter.normal.dot(ray.direction) > 0.0 {
                    inter.medium_interface.outside()
                } else {
                    inter.medium_interface.inside()
                };
                ray.reset();
                specular_bounce = bsdf_sample_result.tp == BSDFType::Specular;
            }
            // Russian roulette
            if depth > 2 && sampler.borrow_mut().next_1d() > 0.95 {
                break;
            }
            throughput /= 0.95;
            depth += 1;
        }
        spectrum
    }
}
