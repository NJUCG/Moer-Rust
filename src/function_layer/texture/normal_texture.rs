use std::rc::Rc;
use image::DynamicImage;
use nalgebra::{Vector2, Vector3};
use crate::function_layer::shape::intersection::Intersection;
use crate::function_layer::texture::texture::{Texture, TextureCoord, TextureMapping};

pub struct NormalTexture {
    normal_map: Rc<DynamicImage>,
}

impl Texture<Vector3<f32>> for NormalTexture {
    fn size(&self) -> Vector2<i64> {
        todo!()
    }

    fn mapping(&self) -> Rc<dyn TextureMapping> {
        todo!()
    }

    fn evaluate(&self, intersection: &Intersection) -> Vector3<f32> {
        todo!()
    }

    fn evaluate_coord(&self, tex_coord: &TextureCoord) -> Vector3<f32> {
        todo!()
    }
}