use std::rc::Rc;
use image::DynamicImage;
use nalgebra::Vector2;
use serde_json::Value;

pub struct Film {
    pub size: [usize; 2],
    pub image: Option<Rc<DynamicImage>>,
}

impl Film {
    pub fn from_json(json: &Value) -> Self {
        let size = json["size"].as_array().unwrap();
        let x = size[0].as_u64().unwrap();
        let y = size[1].as_u64().unwrap();
        let size = [x as usize, y as usize];
        let image = Some(Rc::new(DynamicImage::new_rgb32f(size[0] as u32, size[1] as u32)));
        Self { size, image }
    }
}