use cgmath::Vector2;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::core_layer::constants::INV_PI;
use crate::function_layer::V3f;
use super::{BSDF, BSDFType};
use super::bsdf::{BSDFBase, BSDFSampleResult};
use super::warp::{square_to_cosine_hemisphere, square_to_cosine_hemisphere_pdf};

pub struct OrenNayarBSDF {
    albedo: SpectrumRGB,
    sigma: f32,
    bsdf: BSDFBase,
}

impl OrenNayarBSDF {
    pub fn new(albedo: SpectrumRGB, sigma: f32, bsdf: BSDFBase) -> Self {
        Self {
            albedo,
            sigma,
            bsdf,
        }
    }
}

impl BSDF for OrenNayarBSDF {
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB {
        let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        let s2 = self.sigma * self.sigma;
        let a = 1.0 - 0.5 * s2 / (s2 + 0.33);
        let b = 0.45 * s2 / (s2 + 0.09);

        let sin_alpha = (1.0 - wo_local.y * wo_local.y).sqrt().
            max((1.0 - wi_local.y * wi_local.y).sqrt());
        let tan_beta = (1.0 / (wo_local.y * wo_local.y) - 1.0).sqrt().
            min((1.0 / (wi_local.y * wi_local.y) - 1.0).sqrt());

        let cos_dphi = (wo_local.x * wi_local.x + wo_local.z * wi_local.z) /
            (wo_local.x * wo_local.x + wo_local.z * wo_local.z).sqrt() /
            (wi_local.x * wi_local.x + wi_local.z * wi_local.z).sqrt();

        self.albedo * INV_PI * wi_local.y * (a + b * cos_dphi.max(0.0) * sin_alpha * tan_beta)
    }

    fn sample(&self, _wo: V3f, sample: Vector2<f32>) -> BSDFSampleResult {
        let wi = square_to_cosine_hemisphere(sample);
        let pdf = square_to_cosine_hemisphere_pdf(wi);
        BSDFSampleResult {
            weight: self.albedo,
            wi: self.to_world(wi),
            pdf,
            tp: BSDFType::Diffuse,
        }
    }

    fn bsdf(&self) -> &BSDFBase {
        &self.bsdf
    }
}