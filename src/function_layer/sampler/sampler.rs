use std::rc::Rc;
use serde_json::Value;

pub trait Sampler {
    fn xsp(&self) -> i64;
    fn ysp(&self) -> i64;

}

pub fn construct_sampler(json: &Value) -> Rc<dyn Sampler> {
    todo!()
}