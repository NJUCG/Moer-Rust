use cgmath::InnerSpace;
use serde_json::Value;

use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{compute_ray_differentials, InfiniteLight, Integrator, Ray, RR, Sampler, Scene, V3f};
use crate::function_layer::integrator::integrator::sample_interaction_illumination;
use crate::function_layer::material::MaterialType;

pub struct BlackHoleIntegrator {
    max_iter: u32,
}

impl BlackHoleIntegrator {
    pub fn from_json(json: &Value) -> Self {
        let max_iter = json["maxIter"].as_u64().unwrap() as u32;
        Self { max_iter }
    }
}

impl Integrator for BlackHoleIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB {
        // println!("{:?}", ray.direction);
        let mut spectrum = SpectrumRGB::same(0.0);
        let t_max = ray.t_max;
        let bh_centers = scene.black_hole_centers();

        for _ in 0..self.max_iter {
            let intersection_opt = scene.ray_intersect(ray);
            if let Some(inter) = intersection_opt {
                let shape = inter.shape.as_ref().unwrap();
                if let Some(mat) = shape.material().as_ref() {
                    if mat.mat_type() == MaterialType::BlackHole {
                        return spectrum;
                    }
                } else {
                    break;
                }
            }
            let acc = bh_centers.iter().map(|c| {
                let a = ray.origin - c;
                -a.normalize() / a.magnitude2().powi(2)
            }).sum::<V3f>() * 0.3;
            if acc.magnitude2() < 1e-8 { break; }
            let dir = ray.direction + 0.05 * acc;
            ray.change_dir(dir);
            ray.origin += 0.05 * &ray.direction;
            ray.t_max = 0.1;
        }
        ray.t_max = t_max;
        // println!("{:?}, {:?}", ray.direction, ray.origin);
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
