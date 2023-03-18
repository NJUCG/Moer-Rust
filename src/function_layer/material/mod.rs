mod bxdf;

use std::rc::Rc;
use crate::function_layer::material::bxdf::bsdf::BSDF;
use crate::function_layer::shape::intersection::Intersection;
use crate::function_layer::texture::normal_texture::NormalTexture;

pub trait Material {
    fn normal_map(&self) -> Rc<NormalTexture>;
    fn compute_bsdf(&self, intersection: &Intersection) -> Rc<dyn BSDF>;
}