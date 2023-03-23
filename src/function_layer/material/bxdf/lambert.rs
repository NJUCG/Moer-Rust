use nalgebra::Vector2;
use crate::core_layer::{colorspace::SpectrumRGB, constants::INV_PI};
use crate::function_layer::V3f;
use super::bsdf::{BSDF, BSDFSampleResult};
use super::warp::{square_to_cosine_hemisphere, square_to_cosine_hemisphere_pdf};


pub struct LambertReflection {
    albedo: SpectrumRGB,
    pub normal: V3f,
    pub tangent: V3f,
    pub bitangent: V3f,
}

impl LambertReflection {
    pub fn new(albedo: SpectrumRGB, normal: V3f, tangent: V3f, bitangent: V3f) -> Self {
        Self { albedo, normal, tangent, bitangent }
    }
}

impl BSDF for LambertReflection {
    fn f(&self, wo: &V3f, wi: &V3f) -> SpectrumRGB {
        let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        if wo_local[1] <= 0.0 || wi_local[1] <= 0.0 {
            SpectrumRGB::same(0.0)
        } else { self.albedo * (INV_PI * wi_local[1]) }
        // self.albedo * (INV_PI * wi_local[1])
    }

    fn sample(&self, _wo: &V3f, sample: &Vector2<f32>) -> BSDFSampleResult {
        let weight = self.albedo;
        let wi = square_to_cosine_hemisphere(sample.clone());
        let pdf = square_to_cosine_hemisphere_pdf(wi);
        BSDFSampleResult {
            weight,
            wi: self.to_world(&wi),
            pdf,
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