use super::integrator::sample_interaction_illumination;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{
    compute_ray_differentials, InfiniteLight, Integrator, Ray, Sampler, Scene, RR,
};

pub struct DirectIntegratorSampleLight;

// TODO 目前由于环境光采样还有些bug，先不要在场景中配置环境光
impl Integrator for DirectIntegratorSampleLight {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB {
        let mut spectrum = SpectrumRGB::same(0.0);
        let intersection_opt = scene.ray_intersect(ray);
        if intersection_opt.is_none() {
            for inf_light in &scene.infinite_lights {
                spectrum += inf_light.evaluate_emission_ray(ray);
            }
            return spectrum;
        }
        let mut inter = intersection_opt.unwrap();
        compute_ray_differentials(&mut inter, ray);
        let shape = inter.shape.as_ref().unwrap();
        if let Some(light) = shape.get_light() {
            spectrum += light.borrow().evaluate_emission(&inter, -ray.direction);
        }
        spectrum = sample_interaction_illumination(
            scene,
            -ray.direction,
            &inter,
            spectrum,
            sampler.clone(),
            SpectrumRGB::same(1.0),
        );
        spectrum
    }
}

pub struct DirectIntegratorSampleBSDF;

impl Integrator for DirectIntegratorSampleBSDF {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB {
        let mut spectrum = SpectrumRGB::same(0.0);
        let intersection_opt = scene.ray_intersect(ray);
        println!("{:?}", ray.direction);
        if intersection_opt.is_none() {
            for inf_light in &scene.infinite_lights {
                spectrum += inf_light.evaluate_emission_ray(ray);
            }
            return spectrum;
        }
        let intersection = intersection_opt.unwrap();

        let shape = intersection.shape.as_ref().unwrap();
        if let Some(light) = shape.get_light() {
            spectrum += light
                .borrow()
                .evaluate_emission(&intersection, -ray.direction);
        }
        let material = shape.material();
        let bsdf = material.unwrap().compute_bsdf(&intersection);
        let bsdf_sample_result = bsdf.sample(-ray.direction, sampler.borrow_mut().next_2d());
        let mut shadow_ray = Ray::new(intersection.position, bsdf_sample_result.wi);
        let find_light = scene.ray_intersect(&mut shadow_ray);
        match find_light {
            None => {
                for inf_light in &scene.infinite_lights {
                    let env_s = inf_light.evaluate_emission_ray(&shadow_ray);
                    spectrum += bsdf_sample_result.weight * env_s;
                }
            }
            Some(fl) => {
                let shape = fl.shape.as_ref().unwrap();
                if let Some(light) = shape.get_light() {
                    spectrum += bsdf_sample_result.weight
                        * light.borrow().evaluate_emission(&fl, -shadow_ray.direction);
                }
            }
        }
        spectrum
    }
}
