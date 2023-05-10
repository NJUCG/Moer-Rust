use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::V3f;
use cgmath::InnerSpace;
use cgmath::Vector2;

#[derive(PartialEq)]
pub enum BSDFType {
    Diffuse,
    Specular,
}

pub struct BSDFSampleResult {
    pub weight: SpectrumRGB,
    pub wi: V3f,
    pub pdf: f32,
    pub tp: BSDFType,
}

pub trait BSDF {
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB;
    fn sample(&self, wo: V3f, sample: Vector2<f32>) -> BSDFSampleResult;
    fn bsdf(&self) -> &BSDFBase;
    fn to_local(&self, world: V3f) -> V3f {
        let BSDFBase { normal, tangent, bitangent } = self.bsdf();
        V3f::new(tangent.dot(world), normal.dot(world), bitangent.dot(world))
    }
    fn to_world(&self, local: V3f) -> V3f {
        let BSDFBase { normal, tangent, bitangent } = self.bsdf();
        local[0] * tangent + local[1] * normal + local[2] * bitangent
    }
}

#[derive(Debug, Clone)]
pub struct BSDFBase {
    pub(crate) normal: V3f,
    pub(crate) tangent: V3f,
    pub(crate) bitangent: V3f,
}