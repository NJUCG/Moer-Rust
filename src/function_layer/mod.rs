use image::Rgb32FImage;

pub mod camera;
pub mod film;
mod ray;
pub mod scene;
mod acceleration;
pub mod light;
mod bounds3;
mod shape;
pub mod texture;
pub mod material;
pub mod integrator;
pub mod sampler;

pub type Image = Rgb32FImage;