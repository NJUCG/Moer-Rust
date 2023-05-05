use super::integrator::{convert_pdf, Integrator};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{
    compute_ray_differentials, InfiniteLight, Light, Ray, Sampler, Scene, RR,
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
        let mut intersection = intersection_opt.unwrap();
        compute_ray_differentials(&mut intersection, ray);
        let shape = intersection.shape.as_ref().unwrap();
        if let Some(light) = shape.get_light() {
            spectrum += light
                .borrow()
                .evaluate_emission(&intersection, &(-ray.direction));
        }
        for inf_lights in &scene.infinite_lights {
            let res = inf_lights.sample(&intersection, sampler.borrow_mut().next_2d());
            let mut shadow_ray =
                Ray::new(intersection.position + res.direction * 1e-4, res.direction);
            shadow_ray.t_max = res.distance;
            let occlude = scene.ray_intersect(&mut shadow_ray);
            if occlude.is_none() {
                let material = shape.material();
                let bsdf = material.unwrap().compute_bsdf(&intersection);
                let f = bsdf.f(-ray.direction, shadow_ray.direction);
                let pdf = convert_pdf(&res, &intersection);
                spectrum += res.energy * f / pdf;
            }
        }

        let mut pdf_light = 0.0;
        let light_opt = scene.sample_light(sampler.borrow_mut().next_1d(), &mut pdf_light);
        if let Some(light) = light_opt {
            if pdf_light == 0.0 {
                return spectrum;
            }
            let mut light_sample_result = light
                .borrow()
                .sample(&intersection, sampler.borrow_mut().next_2d());
            let mut shadow_ray = Ray::new(intersection.position, light_sample_result.direction);
            shadow_ray.t_max = light_sample_result.distance;
            let occlude = scene.ray_intersect(&mut shadow_ray);
            if occlude.is_none() {
                let material = shape.material();
                let bsdf = material.unwrap().compute_bsdf(&intersection);
                let f = bsdf.f(-ray.direction, shadow_ray.direction);
                light_sample_result.pdf *= pdf_light;
                let pdf = convert_pdf(&light_sample_result, &intersection);
                spectrum += light_sample_result.energy * f / pdf;
            }
        }

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
            spectrum += light.borrow()
                .evaluate_emission(&intersection, &-ray.direction);
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
                        * light.borrow()
                        .evaluate_emission(&fl, &-shadow_ray.direction);
                }
            }
        }
        spectrum
    }
}
