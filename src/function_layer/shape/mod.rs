mod cone;
mod cube;
mod cylinder;
mod disk;
pub mod intersection;
mod parallelogram;
pub mod shape;
mod sphere;
mod triangle;

pub use intersection::{compute_ray_differentials, Intersection};
pub use shape::{fetch_v3f, Shape};
