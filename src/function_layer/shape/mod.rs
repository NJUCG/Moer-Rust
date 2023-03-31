pub mod intersection;
pub mod shape;
mod triangle;
mod sphere;
mod parallelogram;
mod disk;
mod cylinder;
mod cone;
mod cube;

pub use shape::{Shape, fetch_v3f};
pub use intersection::{Intersection, compute_ray_differentials};