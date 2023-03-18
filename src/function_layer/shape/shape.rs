use std::cell::RefCell;
use std::rc::Rc;
use serde_json::Value;
use crate::core_layer::transform::Transformable;
use crate::function_layer::bounds3::Bounds3;
use crate::function_layer::light::light::Light;
use crate::function_layer::material::Material;

pub trait Shape: Transformable {
    fn set_light(&mut self, l: Rc<RefCell<dyn Light>>);
    fn get_light(&self) -> Option<Rc< RefCell<dyn Light>>>;
    fn material(&self) -> Rc<dyn Material>;
    fn get_bounds(&self) -> Bounds3;
    fn geometry_id(&self) -> i64;
}

pub fn construct_shape(json: &Value) -> Rc<RefCell<dyn Shape>> {
    todo!()
}