use super::{
    direct_integrator::{DirectIntegratorSampleBSDF, DirectIntegratorSampleLight},
    normal_integrator::NormalIntegrator,
    whitted_integrator::WhittedIntegrator,
    path_integrator::PathIntegrator,
    volpath::VolPathIntegrator,
};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::light::light::{LightSampleResult, LightType};
use crate::function_layer::{Intersection, Ray, Sampler, Scene, RR, Light};
use cgmath::InnerSpace;
use serde_json::Value;

pub trait Integrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB;
}

pub fn convert_pdf(result: &LightSampleResult, _intersection: &Intersection) -> f32 {
    let mut pdf = result.pdf;
    let distance = result.distance;
    match result.light_type {
        LightType::SpotLight => pdf *= distance * distance,
        LightType::AreaLight => {
            pdf *= distance * distance;
            pdf /= result.normal.dot(result.direction).abs();
        }
        // 环境光的pdf转换在采样时已经完成
        LightType::EnvironmentLight => (),
    };
    pdf
}

pub fn sample_surface_illumination(scene: &Scene, ray: &Ray,
                                   inter: &Intersection,
                                   mut spectrum: SpectrumRGB,
                                   sampler: RR<dyn Sampler>,
                                   throughput: SpectrumRGB) -> SpectrumRGB {
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
            spectrum += throughput * res.energy * f / pdf;
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
            spectrum += throughput * light_sample_result.energy * f / pdf;
        }
    }
    spectrum
}

pub fn construct_integrator(json: &Value) -> Box<dyn Integrator> {
    match json["type"].as_str().unwrap() {
        "directSampleLight" => Box::new(DirectIntegratorSampleLight {}),
        "directSampleBSDF" => Box::new(DirectIntegratorSampleBSDF {}),
        "normal" => Box::new(NormalIntegrator {}),
        "whitted" => Box::new(WhittedIntegrator {}),
        "path" => Box::new(PathIntegrator::from_json(json)),
        "volpath" => Box::new(VolPathIntegrator::from_json(json)),
        tp => panic!("Invalid integrator type: {}!", tp),
    }
}
