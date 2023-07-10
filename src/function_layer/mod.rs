use cgmath::Vector3;
use image::Rgb32FImage;
use std::cell::RefCell;
use std::rc::Rc;

pub type Image = Rgb32FImage;
pub type V3f = Vector3<f32>;
pub type RR<T> = Rc<RefCell<T>>;

pub mod acceleration;
mod bounds3;
pub mod camera;
pub mod film;
pub mod integrator;
pub mod light;
pub mod material;
mod ray;
pub mod sampler;
pub mod scene;
mod shape;
pub mod texture;
mod medium;
mod interaction;

pub use acceleration::{
    acceleration::{create_acceleration, set_acc_type},
    Acceleration,
};
pub use bounds3::Bounds3;
pub use camera::{construct_camera, Camera};
pub use film::Film;
pub use integrator::{integrator::construct_integrator, Integrator};
pub use light::{light::construct_light, InfiniteLight, Light};
pub use material::{material::construct_material, Material, BSDF, NDF};
pub use ray::Ray;
pub use sampler::{sampler::construct_sampler, Sampler};
pub use scene::Scene;
pub use shape::{
    fetch_v3f, shape::construct_shape, Shape,
};
pub use texture::{texture::construct_texture, Texture};
pub use medium::medium::{Medium, MediumInterface, MediumInteraction};
pub use interaction::{Interaction, SurfaceInteraction, compute_ray_differentials};