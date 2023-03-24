pub mod intersection;
pub mod shape;
mod triangle;
mod sphere;
mod parallelogram;
mod disk;
mod cylinder;
mod cone;
mod cube;

pub use shape::Shape;
pub use intersection::Intersection;