use super::texture::{TextureCoord, TextureMapping, UVMapping};
use super::Texture;
use crate::function_layer::SurfaceInteraction;
use cgmath::{Vector2, Zero};
use std::rc::Rc;

pub struct ConstantTexture<TReturn> {
    size: Vector2<usize>,
    mapping: Rc<dyn TextureMapping>,
    data: TReturn,
}

impl<TReturn: Copy> ConstantTexture<TReturn> {
    pub fn new(data: &TReturn) -> Self {
        Self {
            size: Vector2::zero(),
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

    fn evaluate(&self, _intersection: &SurfaceInteraction) -> TReturn {
        self.data
    }

    fn evaluate_coord(&self, _tex_coord: &TextureCoord) -> TReturn {
        self.data
    }
}
