use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{Image, RR};
use cgmath::Vector2;
use image::{ImageBuffer, ImageFormat, ImageResult, Rgb};
use serde_json::Value;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

pub struct Film {
    pub size: [usize; 2],
    pub image: Option<RR<Image>>,
}

impl Film {
    pub fn from_json(json: &Value) -> Self {
        let size: Vec<usize> = serde_json::from_value(json["size"].clone()).unwrap();
        let size = [size[0], size[1]];
        let image = Some(Rc::new(RefCell::new(Image::new(
            size[0] as u32,
            size[1] as u32,
        ))));
        Self { size, image }
    }

    pub fn deposit(&mut self, xy: Vector2<usize>, spectrum: &SpectrumRGB) {
        let rgb = spectrum.rgb();
        let x = if rgb.x.is_subnormal() {0.0} else {rgb.x};
        let y = if rgb.y.is_subnormal() {0.0} else {rgb.y};
        let z = if rgb.z.is_subnormal() {0.0} else {rgb.z};

        self.image.as_ref().unwrap().borrow_mut().put_pixel(
            xy.x as u32,
            xy.y as u32,
            Rgb([x, y, z]),
        );
    }

    pub fn save(&self, filename: &str, fmt: ImageFormat) -> ImageResult<()> {
        match fmt {
            ImageFormat::Hdr => self.save_hdr(filename),
            ImageFormat::Png => self.save_png(filename),
            _ => panic!("Image format is not supported!"),
        }
    }

    pub fn save_png(&self, filename: &str) -> ImageResult<()> {
        let img = self.image.as_ref().unwrap().borrow();
        let mut png_image = ImageBuffer::new(img.width(), img.height());
        for (x, y, pixel) in img.enumerate_pixels() {
            let png_pixel = Rgb::from([
                (pixel[0] * 255.0) as u8,
                (pixel[1] * 255.0) as u8,
                (pixel[2] * 255.0) as u8,
            ]);
            png_image.put_pixel(x, y, png_pixel);
        }
        png_image.save(filename)?;
        Ok(())
    }

    fn save_hdr(&self, filename: &str) -> ImageResult<()> {
        let file = File::create(filename).unwrap();
        let img = self.image.as_ref().unwrap().borrow();
        let (width, height) = img.dimensions();
        let encoder = image::codecs::hdr::HdrEncoder::new(file);
        encoder.encode(
            &img.pixels()
                .map(|p: &Rgb<f32>| p.clone())
                .collect::<Vec<_>>()[..],
            width as usize,
            height as usize,
        )?;
        Ok(())
    }
}
