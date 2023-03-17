use nalgebra::{Vector2, Vector3};
use crate::core_layer::colorspace::SpectrumRGB;


type V3f = Vector3<f32>;

pub struct BSDFSampleResult {
    pub weight: SpectrumRGB,
    pub wi: V3f,
    pub pdf: f32,
}

pub trait BSDF {
    fn f(&self, wo: &V3f, wi: &V3f) -> SpectrumRGB;
    fn sample(&self, wo: &V3f, sample: &Vector2<f32>) -> BSDFSampleResult;
    fn normal(&self) -> V3f;
    fn tangent(&self) -> V3f;
    fn bitangent(&self) -> V3f;
    fn to_local(&self, world: &V3f) -> V3f {
        let (normal, tangent, bitangent) = (self.normal(), self.tangent(), self.bitangent());
        V3f::new(tangent.dot(world), normal.dot(world), bitangent.dot(world))
    }
    fn to_world(&self, local: &V3f) -> V3f {
        local[0] * self.tangent() + local[1] * self.normal() + local[2] * self.bitangent()
    }
}