use super::bsdf::BSDFBase;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::material::bxdf::bsdf::BSDFSampleResult;
use crate::function_layer::material::bxdf::warp::{
    square_to_cosine_hemisphere, square_to_cosine_hemisphere_pdf,
};
use crate::function_layer::material::bxdf::BSDFType;
use crate::function_layer::{V3f, BSDF, NDF};
use cgmath::{InnerSpace, Vector2};
use std::rc::Rc;

pub struct RoughDielectricBSDF {
    bsdf: BSDFBase,
    albedo: SpectrumRGB,
    alpha: Vector2<f32>,
    eta: f32,
    ndf: Option<Rc<dyn NDF>>,
}

impl RoughDielectricBSDF {
    pub fn new(
        bsdf: BSDFBase,
        albedo: SpectrumRGB,
        alpha: Vector2<f32>,
        eta: f32,
        ndf: Option<Rc<dyn NDF>>,
    ) -> Self {
        Self {
            bsdf,
            albedo,
            alpha,
            eta,
            ndf,
        }
    }

    fn get_r0(eta_o: f32) -> f32 {
        let q = (eta_o - 1.0) / (eta_o + 1.0);
        q * q
    }

    fn get_fr(eta_o: f32, cos_theta: f32) -> f32 {
        let r0 = Self::get_r0(eta_o);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl BSDF for RoughDielectricBSDF {
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB {
        let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        let wh_local = (wo_local + wi_local).normalize();
        let cj = wh_local.dot(wi_local);
        let fr = if wi_local.y > 0.0 {
            RoughDielectricBSDF::get_fr(self.eta, cj)
        } else {
            RoughDielectricBSDF::get_fr(1.0 / self.eta, cj)
        };

        let d = self.ndf.as_ref().unwrap().get_d(wh_local, self.alpha);
        let g = self
            .ndf
            .as_ref()
            .unwrap()
            .get_g(wo_local, wi_local, self.alpha);

        self.albedo * fr * d * g / (4.0 * wo_local.y)
    }

    fn sample(&self, wo: V3f, sample: Vector2<f32>) -> BSDFSampleResult {
        let wi = square_to_cosine_hemisphere(sample);
        let pdf = square_to_cosine_hemisphere_pdf(wi);
        let wi_world = self.to_world(wi);
        BSDFSampleResult {
            weight: self.f(wo, wi_world) / pdf,
            wi: wi_world,
            pdf,
            tp: BSDFType::Diffuse,
        }
    }

    fn bsdf(&self) -> &BSDFBase {
        &self.bsdf
    }
}
