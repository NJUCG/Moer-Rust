use cgmath::{InnerSpace, Vector2};
use cgmath::num_traits::Pow;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::material::bxdf::BSDFType;
use crate::function_layer::material::bxdf::warp::{square_to_cosine_hemisphere, square_to_cosine_hemisphere_pdf};
use crate::function_layer::V3f;
use super::BSDF;
use super::bsdf::{BSDFSampleResult, BSDFBase};

pub struct PhongReflection {
    albedo: SpectrumRGB,
    specular_reflectance: SpectrumRGB,
    kd: SpectrumRGB,
    ks: SpectrumRGB,
    p: f32,
    bsdf: BSDFBase,
}

impl PhongReflection {
    pub fn new(albedo: SpectrumRGB,
               kd: SpectrumRGB, ks: SpectrumRGB, p: f32, bsdf: BSDFBase) -> Self {
        Self {
            albedo,
            specular_reflectance: SpectrumRGB::same(1.0),
            kd,
            ks,
            p,
            bsdf,
        }
    }
    fn pdf(&self, _wo: V3f, wi: V3f) -> f32 {
        // let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        square_to_cosine_hemisphere_pdf(wi_local)
    }
}

impl BSDF for PhongReflection {
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB {
        let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        // let n = V3f::new(0.0, 1.0, 0.0);
        let l_r = wi_local - V3f::new(0.0, 2.0 * wi_local.y, 0.0); //wi - 2.0 * wi.dot(n) * n

        let diffuse = self.kd * wi_local.y; // self.kd * n.dot(wi_local)
        let specular = self.ks * wo_local.dot(l_r).min(0.0).pow(self.p);

        self.specular_reflectance * specular + self.albedo * diffuse
    }

    fn sample(&self, wo: V3f, sample: Vector2<f32>) -> BSDFSampleResult {
        let wi_local = square_to_cosine_hemisphere(sample);
        let wi = self.to_world(wi_local);
        let bsdf_f = self.f(wo, wi);
        let bsdf_pdf = self.pdf(wo, wi);
        BSDFSampleResult {
            weight: bsdf_f / bsdf_pdf,
            wi,
            pdf: bsdf_pdf,
            tp: BSDFType::Diffuse,
        }
    }

    fn bsdf(&self) -> &BSDFBase {
        &self.bsdf
    }
}
