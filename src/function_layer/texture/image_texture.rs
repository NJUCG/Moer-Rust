use super::mipmap::MipMap;
use super::texture::TextureMapping;
use super::texture::{Texture, TextureCoord, UVMapping};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::Intersection;
use crate::resource_layer::image_io::load_img;
use cgmath::Vector2;
use serde_json::Value;
use std::rc::Rc;

pub struct ImageTexture {
    size: Vector2<usize>,
    mapping: Rc<dyn TextureMapping>,
    mipmap: Rc<MipMap>,
}

impl ImageTexture {
    pub fn from_json(json: &Value) -> Self {
        let file_path = json["file"].as_str().unwrap();
        let img = load_img(file_path).expect("Read Image Error!");
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
        SpectrumRGB::from_rgb(self.mipmap.look_up(
            tex_coord.coord,
            tex_coord.duv_dx,
            tex_coord.duv_dy,
        ))
    }
}
