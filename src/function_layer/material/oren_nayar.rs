use std::rc::Rc;
use cgmath::Zero;
use serde_json::Value;
use crate::core_layer::colorspace::SpectrumRGB;
use crate::function_layer::{Intersection, Material, Texture, V3f};
use crate::function_layer::material::bxdf::bsdf::BSDFBase;
use crate::function_layer::material::bxdf::oren_nayar::OrenNayarBSDF;
use crate::function_layer::material::material::{fetch_albedo, fetch_normal_map};
use super::bxdf::BSDF;
use crate::function_layer::texture::constant_texture::ConstantTexture;
use crate::function_layer::texture::normal_texture::NormalTexture;

pub struct OrenNayarMaterial {
    normal_map: Option<Rc<NormalTexture>>,
    albedo: Rc<dyn Texture<SpectrumRGB>>,
    roughness: f32,
}

impl OrenNayarMaterial {
    pub fn from_json(json: &Value) -> Self {
        let normal_map = fetch_normal_map(json);
        let albedo = fetch_albedo(json);
        let roughness = json["roughness"].as_f64().unwrap() as f32;
        Self {
            normal_map,
            albedo,
            roughness,
        }
    }
}

impl Default for OrenNayarMaterial {
    fn default() -> Self {
        Self {
            normal_map: None,
            albedo: Rc::new(ConstantTexture::new(&SpectrumRGB::same(0.5))),
            roughness: 0.0,
        }
    }
}

impl Material for OrenNayarMaterial {
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
        Box::new(OrenNayarBSDF::new(s, self.roughness, bsdf))
    }
}