use super::{area_light::AreaLight, environment_light::EnvironmentLight, spot_light::SpotLight};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{Intersection, Ray, V3f, RR};
use cgmath::Vector2;
use serde_json::Value;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Light {
    fn evaluate_emission(&self, intersection: &Intersection, wo: &V3f) -> SpectrumRGB;
    fn sample(&self, shading_point: &Intersection, sample: Vector2<f32>) -> LightSampleResult;
    fn light_type(&self) -> LightType;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn do_equal(&self, rhs: &dyn Light) -> bool;
}

impl PartialEq for dyn Light + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.do_equal(other)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum LightType {
    SpotLight,
    AreaLight,
    EnvironmentLight,
}

pub struct LightSampleResult {
    pub energy: SpectrumRGB,
    pub direction: V3f,
    pub distance: f32,
    pub normal: V3f,
    pub pdf: f32,
    pub is_delta: bool,
    pub light_type: LightType,
}

pub trait InfiniteLight: Light {
    fn evaluate_emission_ray(&self, ray: &Ray) -> SpectrumRGB;
}

pub fn construct_light(json: &Value) -> RR<dyn Light> {
    match json["type"].as_str().expect("No light type given") {
        "environmentLight" => Rc::new(RefCell::new(EnvironmentLight::from_json(json))),
        "spotLight" => Rc::new(RefCell::new(SpotLight::from_json(json))),
        "areaLight" => Rc::new(RefCell::new(AreaLight::from_json(json))),
        _ => panic!("Invalid light type"),
    }
}
