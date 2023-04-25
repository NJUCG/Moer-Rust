use std::rc::Rc;
use cgmath::{InnerSpace, Vector2};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{BSDF, NDF, V3f};
use crate::function_layer::material::bxdf::bsdf::BSDFSampleResult;
use crate::function_layer::material::bxdf::BSDFType;
use crate::function_layer::material::bxdf::warp::{square_to_cosine_hemisphere, square_to_cosine_hemisphere_pdf};
use super::bsdf::BSDFBase;

pub struct RoughDielectricBSDF {
    bsdf: BSDFBase,
    albedo: SpectrumRGB,
    alpha: Vector2<f32>,
    eta: f32,
    ndf: Option<Rc<dyn NDF>>,
}

impl RoughDielectricBSDF {
    pub fn new(bsdf: BSDFBase,
               albedo: SpectrumRGB, alpha: Vector2<f32>, eta: f32,
               ndf: Option<Rc<dyn NDF>>) -> Self {
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
        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    }
}

impl BSDF for RoughDielectricBSDF {
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB {
        // TODO
        // 1. 转换坐标系到局部坐标
        // 2. 根据公式计算 Fr, D, G
        // 3. return albedo * D * G * Fr / (4 * \cos\theta_o);
        // tips:
        // 不考虑多重介质，如果光线从真空射入介质，其eta即配置中填写的eta；
        // 如果光线从介质射出，则eta = 1/eta
        let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        let wh_local = (wo_local + wi_local).normalize();
        let fr = if wi_local.y > 0.0 {
            RoughDielectricBSDF::get_fr(self.eta, wi_local.y)
        } else {
            RoughDielectricBSDF::get_fr(1.0 / self.eta, wi_local.y)
        };
        let cos_iv = wo_local.dot(wi_local);
        let cos_ov = wo_local.dot(wi_local);

        let d = self.ndf.as_ref().unwrap().get_d(wh_local, self.alpha);
        let g = self.ndf.as_ref().unwrap().get_g(V3f::new(0.0, cos_ov, 0.0),
                                                 V3f::new(0.0,cos_iv, 0.0), self.alpha);

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