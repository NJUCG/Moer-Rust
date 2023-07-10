use cgmath::Point2;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{SurfaceInteraction, Scene, V3f};

pub trait BSSRDF {
    fn bssrdf(&self) -> &BSSRDFBase;
    fn s(pi: &SurfaceInteraction, wi: &V3f) -> SpectrumRGB;
    fn sample_s( u1: f32, u2: Point2<f32>, )
}

struct BSSRDFBase {
    eta: f32
}
