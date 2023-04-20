use super::bsdf::{BSDFSampleResult, BSDFType, BSDF, BSDFBase};
use super::warp::{square_to_cosine_hemisphere, square_to_cosine_hemisphere_pdf};
use crate::core_layer::{colorspace::SpectrumRGB, constants::INV_PI};
use crate::function_layer::V3f;
use cgmath::Vector2;

pub struct LambertReflection {
    albedo: SpectrumRGB,
    bsdf: BSDFBase,
}

impl LambertReflection {
    pub fn new(albedo: SpectrumRGB, normal: V3f, tangent: V3f, bitangent: V3f) -> Self {
        let bsdf = BSDFBase { normal, tangent, bitangent };
        Self {
            albedo,
            bsdf,
        }
    }
}

impl BSDF for LambertReflection {
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB {
        let _wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        // if wo_local[1] <= 0.0 || wi_local[1] <= 0.0 {
        //     SpectrumRGB::same(0.0)
        // } else { self.albedo * (INV_PI * wi_local[1]) }
        self.albedo * (INV_PI * wi_local[1])
    }

    fn sample(&self, _wo: V3f, sample: Vector2<f32>) -> BSDFSampleResult {
        let weight = self.albedo;
        let wi = square_to_cosine_hemisphere(sample.clone());
        let pdf = square_to_cosine_hemisphere_pdf(wi);
        BSDFSampleResult {
            weight,
            wi: self.to_world(wi),
            pdf,
            tp: BSDFType::Diffuse,
        }
    }

    fn bsdf(&self) -> &BSDFBase {
        &self.bsdf
    }
}
