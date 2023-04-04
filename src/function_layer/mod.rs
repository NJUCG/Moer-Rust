use std::cell::RefCell;
use std::rc::Rc;
use image::{Rgb32FImage};
use cgmath::Vector3;

pub type Image = Rgb32FImage;
pub type V3f = Vector3<f32>;
pub type RR<T> = Rc<RefCell<T>>;

pub mod camera;
pub mod film;
mod ray;
pub mod scene;
pub mod acceleration;
pub mod light;
mod bounds3;
mod shape;
pub mod texture;
pub mod material;
pub mod integrator;
pub mod sampler;

pub use ray::Ray;
pub use light::{Light, InfiniteLight, light::construct_light};
pub use bounds3::Bounds3;
pub use shape::{Shape, Intersection, fetch_v3f, compute_ray_differentials, shape::construct_shape};
pub use texture::{Texture, texture::construct_texture};
pub use material::{Material, material::construct_material};
pub use integrator::{Integrator, integrator::construct_integrator};
pub use sampler::{Sampler, sampler::construct_sampler};
pub use camera::{Camera, construct_camera};
pub use scene::Scene;
pub use acceleration::{Acceleration, acceleration::{create_acceleration, set_acc_type}};
pub use film::Film;
