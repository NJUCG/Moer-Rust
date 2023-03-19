#![allow(dead_code)]
use std::rc::Rc;
use nalgebra::Vector2;
use crate::function_layer::Intersection;
use super::texture::{TextureCoord, TextureMapping, UVMapping};
use super::Texture;

pub struct ConstantTexture<TReturn> {
    size: Vector2<usize>,
    mapping: Rc<dyn TextureMapping>,
    data: TReturn,
}

impl<TReturn: Copy> ConstantTexture<TReturn> {
    pub fn new(data: &TReturn) -> Self {
        Self {
            size: Vector2::zeros(),
            mapping: Rc::new(UVMapping {}),
            data: data.clone(),
        }
    }
}

impl<TReturn: Copy> Texture<TReturn> for ConstantTexture<TReturn> {
    fn size(&self) -> Vector2<usize> {
        self.size
    }

    fn mapping(&self) -> Rc<dyn TextureMapping> {
        self.mapping.clone()
    }

    fn evaluate(&self, _intersection: &Intersection) -> TReturn { self.data }

    fn evaluate_coord(&self, _tex_coord: &TextureCoord) -> TReturn {
        self.data
    }
}