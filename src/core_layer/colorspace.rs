use std::ops::{AddAssign, Div, Mul};
use nalgebra::Vector3;
use crate::function_layer::V3f;

#[derive(Copy, Clone, PartialEq)]
pub struct SpectrumRGB {
    rgb: V3f,
}

impl SpectrumRGB {
    pub fn rgb(&self) -> V3f { self.rgb }
}

impl AddAssign<SpectrumRGB> for SpectrumRGB {
    fn add_assign(&mut self, rhs: SpectrumRGB) {
        self.rgb += rhs.rgb;
    }
}

impl Div<f32> for SpectrumRGB {
    type Output = SpectrumRGB;

    fn div(self, rhs: f32) -> Self::Output {
        SpectrumRGB { rgb: self.rgb / rhs }
    }
}

impl Default for SpectrumRGB {
    fn default() -> Self {
        Self { rgb: Vector3::zeros() }
    }
}

impl Mul<f32> for SpectrumRGB {
    type Output = SpectrumRGB;

    fn mul(self, rhs: f32) -> Self::Output {
        SpectrumRGB::from_rgb(self.rgb * rhs)
    }
}

impl Mul<SpectrumRGB> for SpectrumRGB {
    type Output = SpectrumRGB;

    fn mul(self, rhs: SpectrumRGB) -> Self::Output {
        SpectrumRGB::from_rgb(self.rgb.component_mul(&rhs.rgb))
    }
}

impl SpectrumRGB {
    pub fn same(f: f32) -> Self {
        Self { rgb: Vector3::from([f; 3]) }
    }

    #[allow(dead_code)]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { rgb: Vector3::new(r, g, b) }
    }

    pub fn from_rgb(rgb: V3f) -> Self {
        Self { rgb }
    }

    #[allow(dead_code)]
    pub fn to_slice(&self) -> [f32; 3] {
        self.rgb.as_ref().clone()
    }
}
