use cgmath::{InnerSpace, Vector2, Zero};
use serde_json::Value;

use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{compute_ray_differentials, InfiniteLight, Integrator, Ray, RR, Sampler, Scene, Texture, V3f};
use crate::function_layer::integrator::integrator::sample_interaction_illumination;
use crate::function_layer::material::MaterialType;
use crate::function_layer::texture::TextureCoord;

pub struct BlackHoleIntegrator {
    max_iter: u32,
    step: f32,
}

impl BlackHoleIntegrator {
    pub fn from_json(json: &Value) -> Self {
        let max_iter = json["maxIter"].as_u64().unwrap() as u32;
        let step = json["step"].as_f64().unwrap() as f32;

        Self { max_iter, step }
    }
}

impl Integrator for BlackHoleIntegrator {
    fn li(&self, ray: &mut Ray, scene: &Scene, sampler: RR<dyn Sampler>) -> SpectrumRGB {
        // println!("{:?}", ray.direction.normalize());
        let mut spectrum = SpectrumRGB::same(0.0);
        let t_max = ray.t_max;
        let bh_centers = scene.black_hole_centers();
        ray.t_max = 0.05;

        for _ in 0..self.max_iter {
            let intersection_opt = scene.ray_intersect(ray);
            if let Some(inter) = intersection_opt {
                let shape = inter.shape.as_ref().unwrap();
                if let Some(mat) = shape.material().as_ref() {
                    if mat.mat_type() == MaterialType::BlackHole {
                        return spectrum;
                    }
                }
                // hit light source or other shapes
                break;
            }
            let accel = bh_centers.iter().map(|c| {
                let a = ray.origin - c;
                let h_sqr = a.cross(ray.direction).magnitude2();
                -1.5 * a * h_sqr / a.magnitude().powi(5)
            }).sum::<V3f>();
            // if accel.magnitude2() < 1e-8 { break; }
            let dir = ray.direction + self.step * accel;
            ray.change_dir(dir);
            ray.origin += self.step * &ray.direction;
            ray.t_max = 1.5 * self.step;
        }
        ray.t_max = t_max;
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
        if let Some(tex) = shape.texture() {
            let tex_coord = TextureCoord {
                coord: inter.tex_coord,
                duv_dx: Vector2::zero(),
                duv_dy: Vector2::zero(),
            };
            spectrum += tex.evaluate_coord(&tex_coord);
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
