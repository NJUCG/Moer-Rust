use std::any::Any;
use std::f32::consts::PI;
use std::rc::Rc;
use nalgebra::{Vector2, Vector3};
use serde_json::Value;
use crate::core_layer::{colorspace::SpectrumRGB, constants::INV_PI, distribution::Distribution};
use super::light::{InfiniteLight, Light, LightSampleResult, LightType};
use crate::function_layer::{Intersection, Texture, Ray, construct_texture, V3f};
use crate::function_layer::texture::TextureCoord;

pub struct EnvironmentLight {
    environment_map: Rc<dyn Texture<SpectrumRGB>>,
    energy_distribution: Distribution<Vector2<usize>>,
}

fn direction2uv(direction: Vector3<f32>) -> Vector2<f32> {
    let (mut u, mut v): (f32, f32);
    let cos_theta = direction[1];
    v = cos_theta.acos();
    if direction[2].abs() < 1e-8 {
        u = if direction[0] > 0.0 { PI * 0.5 } else { PI * 1.5 }
    } else {
        let tan_phi = direction[0] / (direction[2] + 1e-8);
        u = tan_phi.atan();
        if direction[0] < 0.0 && direction[2] < 0.0 {
            u += PI;
        } else if direction[0] < 0.0 && direction[2] > 0.0 {
            u += 2.0 * PI;
        } else if direction[0] > 0.0 && direction[2] < 0.0 {
            u += PI; // TODO ????
        }
    }
    u *= 0.5 * INV_PI;
    v *= INV_PI;
    Vector2::new(u, v)
}

impl EnvironmentLight {
    pub fn from_json(json: &Value) -> Self {
        if json.get("texture").is_none() {
            panic!("EnvironmentLight must specify texture!\n")
        }
        let environment_map = construct_texture::<SpectrumRGB>(&json["texture"]);
        let [[width, height]] = environment_map.size().data.0;
        let mut indices: Vec<Vector2<usize>> = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                indices.push(Vector2::new(x, y));
            }
        }
        let inv_width = 1.0 / width as f32;
        let inv_height = 1.0 / height as f32;

        let weight_func = |index: Vector2<usize>| {
            let u = index.x as f32 * inv_width;
            let v = index.y as f32 * inv_height;
            let sin_theta = (PI * (index.y as f32 + 0.5) * inv_height).sin();
            let tex_coord = TextureCoord {
                coord: Vector2::new(u, v),
                duv_dx: Vector2::zeros(),
                duv_dy: Vector2::zeros(),
            };
            let s = environment_map.evaluate_coord(&tex_coord);
            s.rgb().dot(&Vector3::new(0.212671, 0.715160, 0.072169)) * sin_theta
        };
        let energy_distribution = Distribution::new(indices, weight_func);
        Self {
            environment_map,
            energy_distribution,
        }
    }
    pub fn copy_constr(env: &EnvironmentLight) -> Self {
        Self {
            environment_map: env.environment_map.clone(),
            energy_distribution: env.energy_distribution.clone(),
        }
    }
}

impl Light for EnvironmentLight {
    fn evaluate_emission(&self, _intersection: &Intersection, _wo: &V3f) -> SpectrumRGB {
        panic!("This shouldn't be invoked!\n");
    }

    fn sample(&self, _shading_point: &Intersection, sample: Vector2<f32>) -> LightSampleResult {
        let sz = self.environment_map.size();
        let inv_width = 1.0 / sz.x as f32;
        let inv_height = 1.0 / sz.y as f32;
        let mut pdf = 0.0;
        let index = self.energy_distribution.sample(sample.x, &mut pdf).unwrap();
        let u = index.x as f32 * inv_width;
        let v = index.y as f32 * inv_height;
        let phi = u * 2.0 * PI;
        let theta = v * PI;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        let x = sin_theta * sin_phi;
        let y = cos_theta;
        let z = sin_theta * cos_phi;
        let tex_coord = TextureCoord {
            coord: Vector2::new(u, v),
            duv_dx: Vector2::zeros(),
            duv_dy: Vector2::zeros(),
        };
        let energy = self.environment_map.evaluate_coord(&tex_coord);
        pdf *= (sz[0] * sz[1]) as f32 * INV_PI * INV_PI * 0.5 / sin_theta;
        LightSampleResult {
            energy,
            direction: Vector3::new(x, y, z),
            distance: f32::INFINITY,
            normal: Vector3::zeros(),
            pdf,
            is_delta: false,
            light_type: LightType::EnvironmentLight,
        }
    }

    fn light_type(&self) -> LightType {
        LightType::EnvironmentLight
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn do_equal(&self, rhs: &dyn Light) -> bool {
        if rhs.light_type() != LightType::EnvironmentLight { return false; }
        let other = rhs.as_any().downcast_ref::<Self>().unwrap();
        self.energy_distribution == other.energy_distribution &&
            Rc::ptr_eq(&self.environment_map, &other.environment_map)
    }
}

impl InfiniteLight for EnvironmentLight {
    fn evaluate_emission_ray(&self, ray: &Ray) -> SpectrumRGB {
        let uv = direction2uv(ray.direction);
        self.environment_map.evaluate_coord(&TextureCoord {
            coord: uv,
            duv_dx: Vector2::zeros(),
            duv_dy: Vector2::zeros(),
        })
    }
}