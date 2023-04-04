use std::any::Any;
use std::rc::Rc;
use cgmath::Zero;
use cgmath::Vector2;
use serde_json::Value;
use crate::core_layer::{colorspace::SpectrumRGB, constants::EPSILON};
use crate::function_layer::{Intersection, RR, Shape, V3f, construct_shape, fetch_v3f};
use super::light::{Light, LightSampleResult, LightType};


pub struct AreaLight {
    pub shape: Option<RR<dyn Shape>>,
    energy: SpectrumRGB,
}

impl AreaLight {
    pub fn from_json(json: &Value) -> Self {
        let shape = construct_shape(&json["shape"]);
        let energy = fetch_v3f(json, "energy", V3f::zero());
        let energy = SpectrumRGB::from_rgb(match energy { Ok(i) | Err(i) => i });
        Self {
            shape: Some(shape),
            energy,
        }
    }
}

impl Light for AreaLight {
    fn evaluate_emission(&self, _intersection: &Intersection, _wo: &V3f) -> SpectrumRGB {
        self.energy
    }

    fn sample(&self, shading_point: &Intersection, sample: Vector2<f32>) -> LightSampleResult {
        let (sample_result, pdf) = self.shape.as_ref().unwrap().borrow().uniform_sample_on_surface(sample);
        let shading_point2sample = sample_result.position - shading_point.position;
        LightSampleResult {
            energy: self.energy,
            direction: V3f::from(shading_point2sample.normalize().data.0[0]),
            distance: shading_point2sample.norm() - EPSILON,
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
        if rhs.light_type() != LightType::AreaLight { return false; }
        let other = rhs.as_any().downcast_ref::<Self>().unwrap();
        self.energy == other.energy && match (&self.shape, &other.shape) {
            (Some(l), Some(r)) => Rc::ptr_eq(l, r),
            (None, None) => true,
            (_, _) => false
        }
    }
}