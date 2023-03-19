use std::any::Any;
use nalgebra::{Point3, Vector2, Vector3};
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::core_layer::constants::EPSILON;
use crate::function_layer::{Intersection, Light, V3f};
use super::light::{LightType, LightSampleResult};

pub struct SpotLight {
    position: Point3<f32>,
    energy: SpectrumRGB,
}

impl SpotLight {
    pub fn from_json(json: &Value) -> Self {
        let position: Vec<f32> = serde_json::from_value(json["position"].clone()).unwrap();
        let energy: Vec<f32> = serde_json::from_value(json["energy"].clone()).unwrap();
        Self {
            position: Point3::from_slice(&position),
            energy: SpectrumRGB::new(energy[0], energy[1], energy[2]),
        }
    }
}

impl Light for SpotLight {
    //! 由于点光源不会与光线发生相交，故该函数实际上不会被调用
    fn evaluate_emission(&self, _intersection: &Intersection, _wo: &V3f) -> SpectrumRGB {
        SpectrumRGB::same(0.0)
    }

    fn sample(&self, shading_point: &Intersection, _sample: Vector2<f32>) -> LightSampleResult {
        let shading_point2sample = self.position - shading_point.position;
        LightSampleResult {
            energy: self.energy,
            direction: shading_point2sample.normalize(),
            distance: shading_point2sample.norm() - EPSILON,
            normal: Vector3::zeros(),
            pdf: 1.0,
            is_delta: true,
            light_type: LightType::SpotLight,
        }
    }

    fn light_type(&self) -> LightType {
        LightType::SpotLight
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn do_equal(&self, rhs: &dyn Light) -> bool {
        if rhs.light_type() != LightType::SpotLight { return false; }
        let other = rhs.as_any().downcast_ref::<Self>().unwrap();
        self.position == other.position && self.energy == other.energy
    }
}