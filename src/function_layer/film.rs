use std::cell::RefCell;
use std::rc::Rc;
use image::{ImageFormat, Rgb};
use nalgebra::Vector2;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{Image, RR};

pub struct Film {
    pub size: [usize; 2],
    pub image: Option<RR<Image>>,
}

impl Film {
    pub fn from_json(json: &Value) -> Self {
        let size: Vec<usize> = serde_json::from_value(json["size"].clone()).unwrap();
        let size = [size[0], size[1]];
        let image = Some(Rc::new(RefCell::new(Image::new(size[0] as u32, size[1] as u32))));
        Self { size, image }
    }

    pub fn deposit(&mut self, xy: Vector2<usize>, spectrum: &SpectrumRGB) {
        let rgb = spectrum.rgb() * 255.0;
        self.image.as_ref().unwrap().borrow_mut().put_pixel(xy.x as u32, xy.y as u32, Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]));
    }

    pub fn save(&self, filename: &str, fmt: ImageFormat) {
        self.image.as_ref().unwrap().borrow().
            save_with_format(filename, fmt).expect("saving error");
    }
}