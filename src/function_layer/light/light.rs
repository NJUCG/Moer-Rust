use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use nalgebra::{Vector2, Vector3};
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::ray::Ray;
use crate::function_layer::shape::intersection::Intersection;

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
#[derive(Eq, PartialEq)]
pub enum LightType { SpotLight, AreaLight, EnvironmentLight }

type V3f = Vector3<f32>;

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
    fn evaluate_emission(ray: &Ray) -> SpectrumRGB;
}

pub fn construct_light(json: &Value) -> Rc<RefCell<dyn Light>> {
    match json["type"].as_str().unwrap() {
        "environmentLight" => {}
        "spotLight" => {}
        "areaLight" => {}
        _ => {}
    }
    todo!()
}