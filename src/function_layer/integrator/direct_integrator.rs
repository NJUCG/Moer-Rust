use std::rc::Rc;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::integrator::integrator::{convert_pdf, Integrator};
use crate::function_layer::light::light::{InfiniteLight, Light};
use crate::function_layer::ray::Ray;
use crate::function_layer::sampler::sampler::Sampler;
use crate::function_layer::scene::Scene;
use crate::function_layer::shape::intersection::{compute_ray_differentials, Intersection};

pub struct DirectIntegratorSampleLight;

// TODO 目前由于环境光采样还有些bug，先不要在场景中配置环境光
impl Integrator for DirectIntegratorSampleLight {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: Rc<dyn Sampler>) -> SpectrumRGB {
        let mut spectrum = SpectrumRGB::same(0.0);
        let intersection_opt = scene.ray_intersect(ray);
        if intersection_opt.is_none() {
            return match &scene.infinite_lights {
                Some(light) => light.evaluate_emission_ray(ray),
                None => spectrum,
            };
        }
        let mut intersection = intersection_opt.unwrap();
        compute_ray_differentials(&mut intersection, ray);
        if let Some(light) = intersection.shape.get_light() {
            spectrum += light.borrow().evaluate_emission(&intersection, &(-ray.direction));
        }
        if scene.infinite_lights.is_some() {
            let res = scene.infinite_lights.as_ref().unwrap().sample(&intersection, sampler.next_2d());
            let mut shadow_ray = Ray::new(intersection.position + res.direction * 1e-4, res.direction);
            shadow_ray.t_max = res.distance;
            let occlude = scene.ray_intersect(&shadow_ray);
            if occlude.is_none() {
                let material = intersection.shape.material();
                let bsdf = material.compute_bsdf(&intersection);
                let f = bsdf.f(&-ray.direction, &shadow_ray.direction);
                let pdf = convert_pdf(&res, &intersection);
                spectrum += res.energy * f / pdf;
            }
            return spectrum;
        }
        let mut pdf_light = 0.0;
        let light = scene.sample_light(sampler.next_1d(), &mut pdf_light);
        let mut light_sample_result = light.borrow().sample(&intersection, sampler.next_2d());
        let mut shadow_ray = Ray::new(intersection.position, light_sample_result.direction);
        shadow_ray.t_max = light_sample_result.distance;
        let occlude = scene.ray_intersect(&shadow_ray);
        if occlude.is_none() {
            let material = intersection.shape.material();
            let bsdf = material.compute_bsdf(&intersection);
            let f = bsdf.f(&-ray.direction, &shadow_ray.direction);
            light_sample_result.pdf *= pdf_light;
            let pdf = convert_pdf(&light_sample_result, &intersection);
            spectrum += light_sample_result.energy * f / pdf;
        }

        spectrum
    }
}

pub struct DirectIntegratorSampleBSDF;

impl Integrator for DirectIntegratorSampleBSDF {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: Rc<dyn Sampler>) -> SpectrumRGB {
        let mut spectrum = SpectrumRGB::same(0.0);
        let intersection_opt = scene.ray_intersect(ray);
        if intersection_opt.is_none() {
            return scene.infinite_lights.as_ref().unwrap().evaluate_emission_ray(ray);
        }
        let intersection = intersection_opt.unwrap();
        if let Some(light) = intersection.shape.get_light() {
            spectrum += light.borrow().evaluate_emission(&intersection, &-ray.direction);
        }
        let material = intersection.shape.material();
        let bsdf = material.compute_bsdf(&intersection);
        let bsdf_sample_result = bsdf.sample(&-ray.direction, &sampler.next_2d());
        let shadow_ray = Ray::new(intersection.position, bsdf_sample_result.wi);
        let find_light = scene.ray_intersect(&shadow_ray);
        match find_light {
            None => {
                let env_s = scene.infinite_lights.as_ref().unwrap().evaluate_emission_ray(&shadow_ray);
                spectrum += bsdf_sample_result.weight * env_s;
            }
            Some(fl) => {
                let shape = fl.shape.clone();
                if let Some(light) = shape.get_light() {
                    spectrum += bsdf_sample_result.weight *
                        light.borrow().evaluate_emission(&fl, &-shadow_ray.direction);
                }
            }
        }
        spectrum
    }
}


