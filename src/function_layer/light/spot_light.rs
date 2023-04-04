use std::any::Any;
use cgmath::Zero;
use nalgebra::{Point3};
use cgmath::Vector2;
use serde_json::Value;
use crate::core_layer::{colorspace::SpectrumRGB, constants::EPSILON};
use crate::function_layer::{Intersection, Light, V3f, fetch_v3f};
use super::light::{LightType, LightSampleResult};

pub struct SpotLight {
    position: Point3<f32>,
    energy: SpectrumRGB,
}

impl SpotLight {
    pub fn from_json(json: &Value) -> Self {
        let position = fetch_v3f(json, "position", V3f::zero()).unwrap();
        let energy = fetch_v3f(json, "energy", V3f::zero()).unwrap();
        Self {
            position: Point3::from([position.x, position.y, position.z]),
            energy: SpectrumRGB::from_rgb(energy),
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
            direction: V3f::from(shading_point2sample.normalize().data.0[0]),
            distance: shading_point2sample.norm() - EPSILON,
            normal: V3f::zero(),
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