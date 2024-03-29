use super::light::{LightSampleResult, LightType};
use crate::core_layer::{colorspace::SpectrumRGB, constants::EPSILON};
use crate::function_layer::{fetch_v3f, Interaction, Light, SurfaceInteraction, V3f};
use cgmath::Point3;
use cgmath::Vector2;
use cgmath::{InnerSpace, Zero};
use serde_json::Value;
use std::any::Any;
use std::f64::consts::PI;

pub struct SpotLight {
    position: Point3<f32>,
    energy: SpectrumRGB,
    cos_theta: f32,
    direction: V3f,
}

impl SpotLight {
    pub fn from_json(json: &Value) -> Self {
        let position = fetch_v3f(json, "position", V3f::zero());
        let energy = fetch_v3f(json, "energy", V3f::zero());
        let angle = json["angle"].as_f64().unwrap();
        let cos_theta = (angle * PI / 180.0).cos() as f32;
        let direction = fetch_v3f(json, "direction", V3f::zero()).normalize();
        Self {
            position: Point3::from([position.x, position.y, position.z]),
            energy: SpectrumRGB::from_rgb(energy),
            cos_theta,
            direction,
        }
    }
}

impl Light for SpotLight {
    //! 由于点光源不会与光线发生相交，故该函数实际上不会被调用
    fn evaluate_emission(&self, _intersection: &SurfaceInteraction, _wo: V3f) -> SpectrumRGB {
        SpectrumRGB::same(0.0)
    }

    fn sample(&self, shading_point: &dyn Interaction, _sample: Vector2<f32>) -> LightSampleResult {
        let shading_point2sample = self.position - shading_point.p();
        let direction = shading_point2sample.normalize();
        let energy = if -direction.dot(self.direction) >= self.cos_theta {
            self.energy
        } else {
            SpectrumRGB::same(0.0)
        };
        LightSampleResult {
            energy,
            direction,
            distance: shading_point2sample.magnitude() - EPSILON,
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
        if rhs.light_type() != LightType::SpotLight {
            return false;
        }
        let other = rhs.as_any().downcast_ref::<Self>().unwrap();
        self.position == other.position && self.energy == other.energy
    }
}
