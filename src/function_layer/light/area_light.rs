use super::light::{Light, LightSampleResult, LightType};
use crate::core_layer::{colorspace::SpectrumRGB, constants::EPSILON};
use crate::function_layer::{construct_shape, fetch_v3f, SurfaceInteraction, Shape, V3f, RR, Interaction};
use cgmath::Vector2;
use cgmath::{InnerSpace, Zero};
use serde_json::Value;
use std::any::Any;
use std::rc::Rc;

pub struct AreaLight {
    pub shape: Option<RR<dyn Shape>>,
    energy: SpectrumRGB,
}

impl AreaLight {
    pub fn from_json(json: &Value) -> Self {
        let shape = construct_shape(&json["shape"]);
        let energy = fetch_v3f(json, "energy", V3f::zero());
        let energy = SpectrumRGB::from_rgb(match energy {
            Ok(i) | Err(i) => i,
        });
        Self {
            shape: Some(shape),
            energy,
        }
    }
}

impl Light for AreaLight {
    fn evaluate_emission(&self, _intersection: &SurfaceInteraction, _wo: V3f) -> SpectrumRGB {
        self.energy
    }

    fn sample(&self, shading_point: &dyn Interaction, sample: Vector2<f32>) -> LightSampleResult {
        let (sample_result, pdf) = self
            .shape
            .as_ref()
            .unwrap()
            .borrow()
            .uniform_sample_on_surface(sample);
        let shading_point2sample = sample_result.position - shading_point.p();
        LightSampleResult {
            energy: self.energy,
            direction: shading_point2sample.normalize(),
            distance: shading_point2sample.magnitude() - EPSILON,
            normal: sample_result.normal,
            pdf,
            is_delta: false,
            light_type: LightType::AreaLight,
        }
    }

    fn light_type(&self) -> LightType {
        LightType::AreaLight
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn do_equal(&self, rhs: &dyn Light) -> bool {
        if rhs.light_type() != LightType::AreaLight {
            return false;
        }
        let other = rhs.as_any().downcast_ref::<Self>().unwrap();
        self.energy == other.energy
            && match (&self.shape, &other.shape) {
            (Some(l), Some(r)) => Rc::ptr_eq(l, r),
            (None, None) => true,
            (_, _) => false,
        }
    }
}
