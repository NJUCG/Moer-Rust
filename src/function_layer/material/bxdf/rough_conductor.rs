use std::rc::Rc;
use cgmath::{ElementWise, InnerSpace, Vector2};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::material::bxdf::{BSDF, BSDFType};
use crate::function_layer::material::bxdf::bsdf::BSDFSampleResult;
use crate::function_layer::material::bxdf::warp::{square_to_cosine_hemisphere, square_to_cosine_hemisphere_pdf};
use crate::function_layer::V3f;
use super::super::NDF;
use super::bsdf::BSDFBase;

pub struct RoughConductorBSDF {
    bsdf: BSDFBase,
    albedo: SpectrumRGB,
    alpha: Vector2<f32>,
    eta: V3f,
    k: V3f,
    ndf: Option<Rc<dyn NDF>>,
}

impl RoughConductorBSDF {
    pub fn new(bsdf: BSDFBase,
               albedo: SpectrumRGB, alpha: Vector2<f32>, eta: V3f, k: V3f,
               ndf: Option<Rc<dyn NDF>>) -> Self {
        Self {
            bsdf,
            albedo,
            alpha,
            eta,
            k,
            ndf,
        }
    }

    fn get_r0(eta: V3f, k: V3f) -> V3f {
        let ones = V3f::from([1.0; 3]);
        ((eta - ones).mul_element_wise(eta - ones) + k.mul_element_wise(k)).div_element_wise(
            (eta + ones).mul_element_wise(eta + ones) + k.mul_element_wise(k))
    }

    fn get_fr(eta: V3f, k: V3f, cos_theta: f32) -> V3f {
        let r0 = Self::get_r0(eta, k);
        r0 + (V3f::from([1.0; 3]) - r0) * (1.0 - cos_theta).powf(5.0)
    }
}

impl BSDF for RoughConductorBSDF {
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB {
        let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        let wh_local = (wo_local + wi_local).normalize();

        let fr = Self::get_fr(self.eta, self.k, wi_local.y);
        let cos_iv = wo_local.dot(wi_local);
        let cos_ov = wo_local.dot(wi_local);

        let d = self.ndf.as_ref().unwrap().get_d(wh_local, self.alpha);
        let g = self.ndf.as_ref().unwrap().get_g(V3f::new(0.0, cos_ov, 0.0),
                                                 V3f::new(0.0, cos_iv, 0.0), self.alpha);

        self.albedo * fr * d * g / (4.0 * wo_local.y * wi_local.y)
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