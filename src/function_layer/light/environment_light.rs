use std::any::Any;
use std::f32::consts::PI;
use std::rc::Rc;
use nalgebra::{Vector2, Vector3};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::core_layer::constants::INV_PI;
use crate::core_layer::distribution::Distribution;
use super::light::{InfiniteLight, Light, LightSampleResult, LightType};
use crate::function_layer::ray::Ray;
use crate::function_layer::shape::intersection::Intersection;
use crate::function_layer::texture::texture::{Texture, TextureCoord};

type V3f = Vector3<f32>;

pub struct EnvironmentLight {
    environment_map: Rc<dyn Texture<SpectrumRGB>>,
    energy_distribution: Distribution<Vector2<i64>>,
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
    pub fn copy_constr(env: &EnvironmentLight) -> Self {
        Self {
            environment_map: env.environment_map.clone(),
            energy_distribution: env.energy_distribution.clone(),
        }
    }
}

impl Light for EnvironmentLight {
    fn evaluate_emission(&self, intersection: &Intersection, wo: &V3f) -> SpectrumRGB {
        todo!()
    }

    fn sample(&self, shading_point: &Intersection, sample: Vector2<f32>) -> LightSampleResult {
        todo!()
    }

    fn light_type(&self) -> LightType {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn do_equal(&self, rhs: &dyn Light) -> bool {
        if let Some(other) = rhs.as_any().downcast_ref::<Self>() {
            self.energy_distribution == other.energy_distribution &&
                Rc::ptr_eq(&self.environment_map, &other.environment_map)
        } else { false }
    }
}

impl InfiniteLight for EnvironmentLight {
    fn evaluate_emission(&self, ray: &Ray) -> SpectrumRGB {
        let uv = direction2uv(ray.direction);
        self.environment_map.evaluate_coord(&TextureCoord {
            coord: uv,
            duv_dx: Vector2::zeros(),
            duv_dy: Vector2::zeros(),
        })
    }
}