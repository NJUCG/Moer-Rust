use std::rc::Rc;
use cgmath::Zero;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{construct_texture, fetch_v3f, Intersection, Material, Texture, V3f};
use crate::function_layer::texture::constant_texture::ConstantTexture;
use super::material::fetch_normal_map;
use super::bxdf::{BSDF, bsdf::BSDFBase, phong::PhongReflection};
use crate::function_layer::texture::normal_texture::NormalTexture;

pub struct PhongMaterial {
    albedo: Rc<dyn Texture<SpectrumRGB>>,
    normal_map: Option<Rc<NormalTexture>>,
    kd: f32,
    ks: f32,
    p: f32,
}

impl PhongMaterial {
    pub fn from_json(json: &Value) -> Self {
        let albedo = if json["albedo"].is_object() {
            construct_texture::<SpectrumRGB>(json)
        } else if json["albedo"].is_array() {
            let s = fetch_v3f(json, "albedo", V3f::zero()).unwrap();
            Rc::new(ConstantTexture::new(&SpectrumRGB::from_rgb(s)))
        } else {
            panic!("Error in albedo format!")
        };
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
            albedo: Rc::new(ConstantTexture::new(&SpectrumRGB::same(0.5))),
            normal_map: None,
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