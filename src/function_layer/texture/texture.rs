use std::rc::Rc;
use nalgebra::Vector2;
use crate::function_layer::shape::intersection::Intersection;


#[derive(Default)]
pub struct TextureCoord {
    pub coord: Vector2<f32>,
    pub duv_dx: Vector2<f32>,
    pub duv_dy: Vector2<f32>,
}

pub trait TextureMapping {
    fn map(&self, intersection: &Intersection) -> TextureCoord;
}

struct UVMapping;

impl TextureMapping for UVMapping {
    fn map(&self, intersection: &Intersection) -> TextureCoord {
        TextureCoord {
            coord: intersection.tex_coord,
            duv_dx: Vector2::new(intersection.du_dx, intersection.dv_dx),
            duv_dy: Vector2::new(intersection.du_dy, intersection.dv_dy),
        }
    }
}

pub trait Texture<TReturn> {
    fn size(&self) -> Vector2<i64>;
    fn mapping(&self) -> Rc<dyn TextureMapping>;
    fn evaluate(&self, intersection: &Intersection) -> TReturn;
    fn evaluate_coord(&self, tex_coord: &TextureCoord) -> TReturn;
}
