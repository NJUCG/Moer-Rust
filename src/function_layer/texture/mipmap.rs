use std::rc::Rc;
use image::imageops::FilterType;
use image::Rgb32FImage;
use nalgebra::{clamp, Vector2, Vector3};

pub struct MipMap {
    pub pyramid: Vec<Rc<Rgb32FImage>>,
}

impl MipMap {
    pub fn new(origin: Rc<Rgb32FImage>) -> Self {
        let size = origin.dimensions();
        if !size.0.is_power_of_two() {
            panic!("目前只支持对长宽为2的次幂的图片做mipmap");
        }
        let n_levels = 1 + size.0.max(size.1).ilog2();
        let mut pyramid = Vec::with_capacity(n_levels as usize);
        pyramid.push(origin);
        for _ in 1..n_levels {
            let previous = pyramid.last().unwrap();
            let p_size = previous.dimensions();
            let current = image::imageops::resize(previous.as_ref(),
                                                  p_size.0 / 2, p_size.1 / 2, FilterType::Nearest);
            pyramid.push(Rc::new(current));
        }
        Self {
            pyramid
        }
    }

    pub fn texel(&self, level: u32, x: i64, y: i64) -> Vector3<f32> {
        let image = &self.pyramid[level as usize];
        let x = clamp(x, 0, image.dimensions().0 as i64);
        let y = clamp(y, 0, image.dimensions().1 as i64);
        let rbg = image.get_pixel(x as u32, y as u32).0;
        Vector3::from(rbg)
    }

    pub fn bilinear(&self, level: u32, uv: Vector2<f32>) -> Vector3<f32> {
        let level = clamp(level, 0, self.pyramid.len() as u32 - 1);
        let (x, y) = self.pyramid[level as usize].dimensions();
        let x = uv.x * x as f32 - 0.5;
        let y = uv.y * y as f32 - 0.5;
        let x0 = x.floor() as i64;
        let y0 = y.floor() as i64;
        let dx = x - x.floor();
        let dy = y - y.floor();
        (1.0 - dx) * (1.0 - dy) * self.texel(level, x0, y0) +
            (1.0 - dx) * dy * self.texel(level, x0, y0 + 1) +
            dx * (1.0 - dy) * self.texel(level, x0 + 1, y0) +
            dx * dy * self.texel(level, x0 + 1, y0 + 1)
    }

    pub fn look_up(&self, uv: Vector2<f32>, duv0: Vector2<f32>, duv1: Vector2<f32>) -> Vector3<f32> {
        let width = duv0.amax().max(duv1.amax());

        let level = self.pyramid.len() as f32 - 1.0 + fastapprox::fast::log2(width.max(1e-8));
        // let x = uv.x * self.pyramid[0].dimensions().0 as f32;
        // let y = uv.y * self.pyramid[0].dimensions().0 as f32;
        if level < 0.0 {
            self.bilinear(0, uv)
        } else if level >= self.pyramid.len() as f32 - 1.0 {
            self.texel(self.pyramid.len() as u32 - 1, 0, 0)
        } else {
            let i_level = level.floor() as u32;
            let dl = level.fract();
            (1.0 - dl) * self.bilinear(i_level, uv) + dl * self.bilinear(i_level + 1, uv)
        }
    }
}