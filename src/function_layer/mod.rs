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

pub use ray::Ray;
pub use light::Light;
pub use bounds3::Bounds3;
pub use shape::{Shape, Intersection};
pub use texture::{Texture, texture::construct_texture};
pub use material::{Material, material::construct_material};
pub use integrator::{Integrator, integrator::construct_integrator};
pub use sampler::{Sampler, sampler::construct_sampler};
pub use camera::{construct_camera, Camera};
pub use scene::Scene;