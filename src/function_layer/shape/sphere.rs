use nalgebra::Vector2;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Intersection, Ray, Shape};
use super::shape::ShapeBase;

struct Sphere {
    shape: ShapeBase,
}

impl Transformable for Sphere {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Sphere {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &Ray) -> Option<(u64, f32, f32)> {
        todo!()
    }

    fn fill_intersection(&self, distance: f32, prim_id: u64, u: f32, v: f32, intersection: &mut Intersection) {
        todo!()
    }

    fn uniform_sample_on_surface(&self, sample: Vector2<f32>) -> (Intersection, f32) {
        todo!()
    }

    fn init_internal_acceleration(&mut self) {}
}