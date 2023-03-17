mod bxdf;

use std::rc::Rc;
use crate::function_layer::texture::normal_texture::NormalTexture;

pub trait Material {
    fn normal_map(&self) -> Rc<NormalTexture>;
}