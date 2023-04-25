use std::rc::Rc;
use cgmath::Zero;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{Intersection, Material, Texture, V3f};
use crate::function_layer::texture::{constant_texture::ConstantTexture, normal_texture::NormalTexture};
use super::material::{fetch_normal_map, fetch_albedo};
use super::bxdf::{BSDF, bsdf::BSDFBase, phong::PhongReflection};


pub struct PhongMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    albedo: Rc<dyn Texture<SpectrumRGB>>,
    kd: f32,
    ks: f32,
    p: f32,
}

impl PhongMaterial {
    pub fn from_json(json: &Value) -> Self {
        let albedo = fetch_albedo(json);
        let normal_map = fetch_normal_map(json);
        let kd = json["kd"].as_f64().unwrap() as f32;
        let ks = json["ks"].as_f64().unwrap() as f32;
        let p = json["p"].as_f64().unwrap() as f32;

        Self { albedo, normal_map, kd, ks, p }
    }
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self {
            normal_map: None,
            albedo: Rc::new(ConstantTexture::new(&SpectrumRGB::same(0.5))),
            kd: 0.0,
            ks: 0.0,
            p: 0.0,
        }
    }
}

impl Material for PhongMaterial {
    fn normal_map(&self) -> Option<Rc<NormalTexture>> {
        self.normal_map.clone()
    }

    fn compute_bsdf(&self, intersection: &Intersection) -> Box<dyn BSDF> {
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
        Box::new(PhongReflection::new(s, self.kd, self.ks, self.p, bsdf))
    }
}