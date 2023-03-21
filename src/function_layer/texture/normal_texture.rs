use std::rc::Rc;
use image::Rgb32FImage;
use nalgebra::{clamp, Vector2, Vector3};
use serde_json::Value;
use crate::function_layer::Image;
use crate::function_layer::shape::intersection::Intersection;
use super::texture::{Texture, TextureCoord, TextureMapping, UVMapping};

pub struct NormalTexture {
    size: Vector2<usize>,
    mapping: Rc<dyn TextureMapping>,
    normal_map: Rc<Rgb32FImage>,
}

impl NormalTexture {
    pub fn from_json(json: &Value) -> Self {
        let relative_path = json["file"].as_str().unwrap();
        let normal_map = image::io::Reader::open(relative_path).expect("Open image error!")
            .decode().expect("Decode error!").to_rgb32f();
        let size = normal_map.dimensions();
        let mapping = Rc::new(UVMapping {});
        Self {
            size: Vector2::new(size.0 as usize, size.1 as usize),
            mapping,
            normal_map: Rc::new(normal_map),
        }
    }
}

impl Texture<Vector3<f32>> for NormalTexture {
    fn size(&self) -> Vector2<usize> {
        self.size
    }

    fn mapping(&self) -> Rc<dyn TextureMapping> {
        self.mapping.clone()
    }

    fn evaluate(&self, intersection: &Intersection) -> Vector3<f32> {
        let tex_coord = self.mapping.map(intersection);
        self.evaluate_coord(&tex_coord)
    }

    fn evaluate_coord(&self, tex_coord: &TextureCoord) -> Vector3<f32> {
        let x = tex_coord.coord.x * self.size.x as f32;
        let y = tex_coord.coord.y * self.size.y as f32;
        let x = clamp(x, 0.0, self.size.x as f32 - 1.0) as u32;
        let y = clamp(y, 0.0, self.size.y as f32 - 1.0) as u32;
        let xyz = self.normal_map.get_pixel(x, y);
        Vector3::from(xyz.0) * 2.0 - Vector3::from([1.0; 3])
    }
}