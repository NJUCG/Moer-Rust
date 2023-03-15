use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use nalgebra::{Vector2, Vector3};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::light::light::{Light, LightSampleResult, LightType};
use crate::function_layer::shape::intersection::Intersection;
use crate::function_layer::shape::shape::Shape;

pub struct AreaLight {
    pub shape: Option<Rc<RefCell<dyn Shape>>>,
    energy: SpectrumRGB,
}

impl Light for AreaLight {
    fn evaluate_emission(&self, intersection: &Intersection, wo: &Vector3<f32>) -> SpectrumRGB {
        todo!()
    }

    fn sample(&self, shading_point: &Intersection, sample: Vector2<f32>) -> LightSampleResult {
        todo!()
    }

    fn light_type(&self) -> LightType {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn do_equal(&self, rhs: &dyn Light) -> bool {
        if let Some(other) = rhs.as_any().downcast_ref::<Self>() {
            self.energy == other.energy &&
                match (&self.shape, &other.shape) {
                    (Some(l), Some(r)) => Rc::ptr_eq(l, r),
                    (None, None) => true,
                    (_, _) => false
                }
        } else { false }
    }
}