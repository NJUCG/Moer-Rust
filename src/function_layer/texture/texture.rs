use super::image_texture::ImageTexture;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::shape::intersection::Intersection;
use cgmath::{Vector2, Zero};
use serde_json::Value;
use std::rc::Rc;

pub struct TextureCoord {
    pub coord: Vector2<f32>,
    pub duv_dx: Vector2<f32>,
    pub duv_dy: Vector2<f32>,
}

impl Default for TextureCoord {
    fn default() -> Self {
        Self {
            coord: Vector2::zero(),
            duv_dx: Vector2::zero(),
            duv_dy: Vector2::zero(),
        }
    }
}

pub trait TextureMapping {
    fn map(&self, intersection: &Intersection) -> TextureCoord;
}

pub struct UVMapping;

impl TextureMapping for UVMapping {
    fn map(&self, intersection: &Intersection) -> TextureCoord {
        TextureCoord {
            coord: intersection.tex_coord,
            duv_dx: Vector2::new(intersection.du_dx, intersection.dv_dx),
            duv_dy: Vector2::new(intersection.du_dy, intersection.dv_dy),
        }
    }
}

pub trait Texture<TReturn> {
    fn size(&self) -> Vector2<usize>;
    fn mapping(&self) -> Rc<dyn TextureMapping>;
    fn evaluate(&self, intersection: &Intersection) -> TReturn;
    fn evaluate_coord(&self, tex_coord: &TextureCoord) -> TReturn;
}

pub fn construct_texture<TReturn>(json: &Value) -> Rc<dyn Texture<SpectrumRGB>> {
    match json["type"].as_str().expect("No spectrum type given!") {
        "imageTex" => Rc::new(ImageTexture::from_json(json)),
        _ => panic!("Invalid spectrum type!"),
    }
}
