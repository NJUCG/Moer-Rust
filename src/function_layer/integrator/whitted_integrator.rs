use super::integrator::convert_pdf;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::material::bxdf::BSDFType;
use crate::function_layer::{
    compute_ray_differentials, InfiniteLight, Integrator, Light, Ray, Sampler, Scene, RR,
};

pub struct WhittedIntegrator;

impl Integrator for WhittedIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB {
        let mut ray = ray;
        let mut spectrum = SpectrumRGB::same(0.0);
        let mut beta = SpectrumRGB::same(1.0);
        loop {
            let its_opt = scene.ray_intersect(ray);
            if its_opt.is_none() {
                for light in &scene.infinite_lights {
                    spectrum += beta * light.evaluate_emission_ray(ray);
                }
                break;
            }
            let mut its = its_opt.unwrap();
            if let Some(l) = its.shape.as_ref().unwrap().get_light() {
                spectrum += beta * l.borrow().evaluate_emission(&its, -ray.direction);
            }
            compute_ray_differentials(&mut its, ray);
            let shape = its.shape.as_ref().unwrap();
            let bsdf = shape.material().unwrap().compute_bsdf(&its);
            let bsdf_sample_result = bsdf.sample(-ray.direction, sampler.borrow_mut().next_2d());
            match bsdf_sample_result.tp {
                BSDFType::Specular => {
                    ray.origin = its.position;
                    ray.change_dir(bsdf_sample_result.wi);
                    ray.reset();
                    beta *= &bsdf_sample_result.weight;
                    continue;
                }
                BSDFType::Diffuse => {
                    for light in &scene.infinite_lights {
                        let res = light.sample(&its, sampler.borrow_mut().next_2d());
                        let mut shadow_ray = Ray::new(its.position, res.direction);
                        if scene.ray_intersect(&mut shadow_ray).is_none() {
                            let f = bsdf.f(-ray.direction, shadow_ray.direction);
                            let pdf = convert_pdf(&res, &its);
                            spectrum += beta * res.energy * f / pdf;
                        }
                    }
                    let mut pdf_light = 0.0;
                    let light = scene.sample_light(sampler.borrow_mut().next_1d(), &mut pdf_light);
                    if light.is_none() || pdf_light == 0.0 {
                        break;
                    }
                    let light = light.as_ref().unwrap().borrow();
                    let mut res = light.sample(&its, sampler.borrow_mut().next_2d());
                    let mut shadow_ray = Ray::new(its.position, res.direction);
                    shadow_ray.t_max = res.distance;
                    if scene.ray_intersect(&mut shadow_ray).is_none() {
                        let f = bsdf.f(-ray.direction, shadow_ray.direction);
                        res.pdf *= pdf_light;
                        let pdf = convert_pdf(&res, &its);
                        spectrum += beta * res.energy * f / pdf;
                    }
                    break;
                }
            }
        }
        spectrum
    }
}
