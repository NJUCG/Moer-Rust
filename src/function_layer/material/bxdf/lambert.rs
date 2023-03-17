use nalgebra::{Vector2, Vector3};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::core_layer::constants::INV_PI;
use crate::function_layer::material::bxdf::bsdf::{BSDF, BSDFSampleResult};
use crate::function_layer::material::bxdf::warp::{square_to_cosine_hemisphere, square_to_cosine_hemisphere_pdf};

type V3f = Vector3<f32>;

pub struct LambertReflection {
    albedo: SpectrumRGB,
    pub normal: V3f,
    pub tangent: V3f,
    pub bitangent: V3f,
}

impl BSDF for LambertReflection {
    fn f(&self, wo: &V3f, wi: &V3f) -> SpectrumRGB {
        let wo_local = self.to_local(wo);
        let wi_local = self.to_local(wi);
        if wo_local[1] <= 0.0 || wi_local[1] <= 0.0 {
            SpectrumRGB::same(0.0)
        } else { self.albedo * (INV_PI * wo_local[1]) }
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