#![allow(dead_code)]
use std::rc::Rc;
use nalgebra::Vector2;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::shape::intersection::Intersection;
use super::texture::{Texture, TextureCoord, UVMapping};
use super::mipmap::MipMap;
use super::texture::TextureMapping;

pub struct ImageTexture {
    size: Vector2<usize>,
    mapping: Rc<dyn TextureMapping>,
    mipmap: Rc<MipMap>,
}

impl ImageTexture {
    pub fn from_json(json: &Value) -> Self {
        let file_path = json["file"].as_str().unwrap();
        let img = image::io::Reader::open(file_path).expect("Open image error!")
            .decode().expect("Decode error!").to_rgb32f();
        let img = Rc::new(img);
        let size = img.dimensions();
        let size = Vector2::new(size.0 as usize, size.1 as usize);
        Self {
            size,
            mapping: Rc::new(UVMapping {}),
            mipmap: Rc::new(MipMap::new(img)),
        }
    }
}

impl Texture<SpectrumRGB> for ImageTexture {
    fn size(&self) -> Vector2<usize> {
        self.size
    }

    fn mapping(&self) -> Rc<dyn TextureMapping> {
        self.mapping.clone()
    }

    fn evaluate(&self, intersection: &Intersection) -> SpectrumRGB {
        let tex_coord = self.mapping.map(intersection);
        self.evaluate_coord(&tex_coord)
    }

    fn evaluate_coord(&self, tex_coord: &TextureCoord) -> SpectrumRGB {
        SpectrumRGB::from_rgb(
            self.mipmap.look_up(tex_coord.coord, tex_coord.duv_dx, tex_coord.duv_dy))
    }
}
