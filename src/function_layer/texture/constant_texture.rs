use std::rc::Rc;
use std::thread::sleep;
use nalgebra::Vector2;
use crate::function_layer::shape::intersection::Intersection;
use crate::function_layer::texture::texture::{TextureCoord, TextureMapping};
use super::texture::Texture;

pub struct ConstantTexture<TReturn> {
    size: Vector2<usize>,
    mapping: Rc<dyn TextureMapping>,
    data: TReturn,
}

impl<TReturn: Copy> Texture<TReturn> for ConstantTexture<TReturn> {
    fn size(&self) -> Vector2<usize> {
        self.size
    }

    fn mapping(&self) -> Rc<dyn TextureMapping> {
        self.mapping.clone()
    }

    fn evaluate(&self, intersection: &Intersection) -> TReturn {
        self.data
    }

    fn evaluate_coord(&self, tex_coord: &TextureCoord) -> TReturn {
        self.data
    }
}