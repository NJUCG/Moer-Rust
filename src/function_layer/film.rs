use std::cell::RefCell;
use std::rc::Rc;
use image::{ImageFormat, Rgb};
use nalgebra::Vector2;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::Image;

pub struct Film {
    pub size: [usize; 2],
    pub image: Option<Rc<RefCell<Image>>>,
}

impl Film {
    pub fn from_json(json: &Value) -> Self {
        let size = json["size"].as_array().unwrap();
        let x = size[0].as_u64().unwrap();
        let y = size[1].as_u64().unwrap();
        let size = [x as usize, y as usize];
        let image = Some(Rc::new(RefCell::new(Image::new(size[0] as u32, size[1] as u32))));
        Self { size, image }
    }

    pub fn deposit(&mut self, xy: Vector2<usize>, spectrum: &SpectrumRGB) {
        self.image.as_ref().unwrap().borrow_mut().put_pixel(xy.x as u32, xy.y as u32, Rgb(spectrum.to_slice()));
    }

    pub fn save_hdr(&self, filename: &str) {
        self.image.as_ref().unwrap().borrow().
            save_with_format(filename, ImageFormat::Hdr).expect("saving error");
    }
}