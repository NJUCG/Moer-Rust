use cgmath::{InnerSpace, Vector2, Zero};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{BSDF, V3f};
use crate::function_layer::material::bxdf::bsdf::{BSDFBase, BSDFSampleResult};
use crate::function_layer::material::bxdf::BSDFType;

pub struct TransparentBSDF {
    pub bsdf: BSDFBase,
    pub ior: f32,
}

impl BSDF for TransparentBSDF {
    fn f(&self, _wo: V3f, _wi: V3f) -> SpectrumRGB {
        SpectrumRGB::same(0.0)
    }

    fn sample(&self, wo: V3f, sample: Vector2<f32>) -> BSDFSampleResult {
        let wo_local = self.to_local(wo);
        let fr = fresnel(wo_local, self.ior);

        if sample.x > sample.y {
            let wi_local = V3f::new(-wo_local.x, wo_local.y, -wo_local.z);
            BSDFSampleResult {
                weight: SpectrumRGB::same(fr * 2.0),
                wi: self.to_world(wi_local),
                pdf: 1.0,
                tp: BSDFType::Specular,
            }
        } else {
            let wi_local = refract(wo_local, self.ior).normalize();
            BSDFSampleResult {
                weight: SpectrumRGB::same(2.0 - fr * 2.0),
                wi: self.to_world(wi_local),
                pdf: 1.0,
                tp: BSDFType::Diffuse,
            }
        }
    }

    fn bsdf(&self) -> &BSDFBase {
        &self.bsdf
    }
}

fn refract(i: V3f, ior: f32) -> V3f {
    let cosi = i.y.abs();
    let (eta, n0) = if i.y > 0.0 { (1.0 / ior, 1.0) } else { (ior, -1.0) };
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        V3f::zero()
    } else {
        -eta * i + (eta * cosi - k.sqrt()) * V3f::new(0.0, n0, 0.0)
    }
}

fn fresnel(i: V3f, ior: f32) -> f32 {
    let cosi = i.y.abs();
    let (etai, etat) = if i.y > 0.0 { (1.0, ior) } else { (ior, 1.0) };
    let sint = etai / etat * (1.0 - cosi * cosi).sqrt();
    if sint < 1.0 {
        let cost = (1.0 - sint * sint).sqrt();
        let rs = (etat * cosi - etai * cost) / (etat * cosi + etai * cost);
        let rp = (etai * cosi - etat * cost) / (etai * cosi + etat * cost);
        (rs * rs + rp * rp) / 2.0
    } else {
        1.0
    }
}
