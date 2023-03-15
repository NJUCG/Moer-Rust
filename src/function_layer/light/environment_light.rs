use std::any::Any;
use std::rc::Rc;
use nalgebra::{Vector2, Vector3};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::core_layer::distribution::Distribution;
use super::light::{InfiniteLight, Light, LightSampleResult, LightType};
use crate::function_layer::ray::Ray;
use crate::function_layer::shape::intersection::Intersection;
use crate::function_layer::texture::texture::Texture;

type V3f = Vector3<f32>;

pub struct EnvironmentLight {
    environment_map: Rc<dyn Texture<SpectrumRGB>>,
    energy_distribution: Distribution<Vector2<i64>>,
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
    fn evaluate_emission(ray: &Ray) -> SpectrumRGB {
        todo!()
    }
}