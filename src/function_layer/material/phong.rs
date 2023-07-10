use super::bxdf::{bsdf::BSDFBase, phong::PhongReflection, BSDF};
use super::material::{fetch_albedo, fetch_normal_map};
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::material::material::fetch_spectrum;
use crate::function_layer::texture::{
    constant_texture::ConstantTexture, normal_texture::NormalTexture,
};
use crate::function_layer::{Material, SurfaceInteraction, Texture, V3f};
use cgmath::Zero;
use serde_json::Value;
use std::rc::Rc;

pub struct PhongMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    albedo: Rc<dyn Texture<SpectrumRGB>>,
    kd: Rc<dyn Texture<SpectrumRGB>>,
    ks: Rc<dyn Texture<SpectrumRGB>>,
    p: f32,
}

impl PhongMaterial {
    pub fn from_json(json: &Value) -> Self {
        let albedo = fetch_albedo(json);
        let normal_map = fetch_normal_map(json);
        let kd = fetch_spectrum(json, "kd");
        let ks = fetch_spectrum(json, "ks");
        let p = json["p"].as_f64().unwrap() as f32;

        Self {
            albedo,
            normal_map,
            kd,
            ks,
            p,
        }
    }
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self {
            normal_map: None,
            albedo: Rc::new(ConstantTexture::new(&SpectrumRGB::same(0.5))),
            kd: Rc::new(ConstantTexture::new(&SpectrumRGB::same(0.0))),
            ks: Rc::new(ConstantTexture::new(&SpectrumRGB::same(0.0))),
            p: 0.0,
        }
    }
}

impl Material for PhongMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &SurfaceInteraction) -> Box<dyn BSDF> {
        let mut normal = V3f::zero();
        let mut tangent = V3f::zero();
        let mut bitangent = V3f::zero();

        self.compute_shading_geometry(intersection, &mut normal, &mut tangent, &mut bitangent);

        let s = self.albedo.evaluate(intersection);
        let bsdf = BSDFBase {
            normal,
            tangent,
            bitangent,
        };
        let kd = self.kd.evaluate(intersection);
        let ks = self.ks.evaluate(intersection);
        Box::new(PhongReflection::new(s, kd, ks, self.p, bsdf))
    }
}
