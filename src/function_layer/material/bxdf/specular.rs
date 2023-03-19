use nalgebra::Vector2;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::V3f;
use super::bsdf::BSDFSampleResult;
use super::bsdf::BSDF;


pub struct SpecularReflection {
    pub normal: V3f,
    pub tangent: V3f,
    pub bitangent: V3f,
}

impl BSDF for SpecularReflection {
    fn f(&self, _wo: &V3f, _wi: &V3f) -> SpectrumRGB {
        SpectrumRGB::same(0.0)
    }

    fn sample(&self, wo: &V3f, _sample: &Vector2<f32>) -> BSDFSampleResult {
        let wo_local = self.to_local(wo);
        let wi_local = V3f::new(-wo_local.x, wo_local.y, -wo_local.z);
        BSDFSampleResult {
            weight: SpectrumRGB::same(1.0),
            wi: self.to_world(&wi_local),
            pdf: 1.0,
        }
    }

    fn normal(&self) -> V3f {
        self.normal
    }

    fn tangent(&self) -> V3f {
        self.tangent
    }

    fn bitangent(&self) -> V3f {
        self.bitangent
    }
}